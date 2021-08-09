/*

    TencentOS API wrapper for rust world

*/

#include "tos_k.h"
#include "esp8266_tencent_firmware.h"
#include "tencent_firmware_module_wrapper.h"
#include "ch20_parser.h"
#include "oled.h"
#include "user_config.h"

#include "tos_at.h"

#define REPORT_DATA_TEMPLATE    "{\\\"method\\\":\\\"report\\\"\\,\\\"clientToken\\\":\\\"00000001\\\"\\,\\\"params\\\":{\\\"ch20_ppm_value\\\":%.3f}}"

#define RECV_LEN            1024
uint8_t recv_data[RECV_LEN];

void default_message_handler(mqtt_message_t* msg)
{
    printf("callback:\r\n");
    printf("---------------------------------------------------------\r\n");
    printf("\ttopic:%s\r\n", msg->topic);
    printf("\tpayload:%s\r\n", msg->payload);
    printf("---------------------------------------------------------\r\n");
}

char payload[256] = {0};
static char report_topic_name[TOPIC_NAME_MAX_SIZE] = {0};
static char report_reply_topic_name[TOPIC_NAME_MAX_SIZE] = {0};

k_mail_q_t mail_q;
ch20_data_t ch20_value;
uint8_t ch20_value_pool[5 * sizeof(ch20_data_t)];

void rust_mqtt_daemon()
{
    char *str = "TencentOS XXXX";

    OLED_Init();
    OLED_Clear();
    OLED_ShowString(0, 0, (uint8_t*)str, 16);


    int ret = 0;
    int size = 0;
    int lightness = 0;
    mqtt_state_t state;

    char *product_id = PRODUCT_ID;
    char *device_name = DEVICE_NAME;
    char *key = DEVICE_KEY;

    device_info_t dev_info;
    memset(&dev_info, 0, sizeof(device_info_t));

    size_t mail_size;
    float  ch20_ppm_value;
    char   ch20_ppm_str[20];


    /* OLED显示日志 */
    OLED_ShowString(0, 2, (uint8_t*)"connecting...", 16);

    /*

        Please Choose your AT Port first, default is HAL_UART_2(USART2)

        网络层初始化, 这个函数位于

            tos/devices/esp8266_tencent_firmware/esp8266_tencent_firmware.c

    */

    ret = esp8266_tencent_firmware_sal_init(HAL_UART_PORT_2);

    if (ret < 0) {
        printf("esp8266 tencent firmware sal init fail, ret is %d\r\n", ret);
    }

    esp8266_tencent_firmware_join_ap(WIFI_NAME, WIFI_PASSWORD);

    strncpy(dev_info.product_id, product_id, PRODUCT_ID_MAX_SIZE);
    strncpy(dev_info.device_name, device_name, DEVICE_NAME_MAX_SIZE);
    strncpy(dev_info.device_serc, key, DEVICE_SERC_MAX_SIZE);
    tos_tf_module_info_set(&dev_info, TLS_MODE_PSK);

    /*

        DEFAULT_MQTT_PARAMS 定义在 tos/net/tencent_firmware_module_wrapper/tencent_firmware_module_wrapper.h

        typedef struct mqtt_param_st {
            tls_mode_t  tls_mode;
            uint32_t    command_timeout;
            uint32_t    keep_alive_interval_ms;
            uint8_t     clean_session;
            uint8_t     auto_connect_enable;
        } mqtt_param_t;
        #define DEFAULT_MQTT_PARAMS { TLS_MODE_PSK, MQTT_COMMAND_TIMEOUT, 240, 1, 1 }

    */
    mqtt_param_t init_params = DEFAULT_MQTT_PARAMS;
    if (tos_tf_module_mqtt_conn(init_params) != 0) {
        printf("module mqtt conn fail\r\n");
    } else {
        printf("module mqtt conn success\r\n");
    }

    if (tos_tf_module_mqtt_state_get(&state) != -1) {
        printf("MQTT: %s\n", state == MQTT_STATE_CONNECTED ? "CONNECTED" : "DISCONNECTED");
    }

    /* 开始订阅topic */
    size = snprintf(report_reply_topic_name, TOPIC_NAME_MAX_SIZE, "$thing/down/property/%s/%s", product_id, device_name);

    if (size < 0 || size > sizeof(report_reply_topic_name) - 1) {
        printf("sub topic content length not enough! content size:%d  buf size:%d", size, (int)sizeof(report_reply_topic_name));
    }
    if (tos_tf_module_mqtt_sub(report_reply_topic_name, QOS0, default_message_handler) != 0) {
        printf("module mqtt sub fail\n");
    } else {
        printf("module mqtt sub success\n");
    }

    memset(report_topic_name, sizeof(report_topic_name), 0);
    size = snprintf(report_topic_name, TOPIC_NAME_MAX_SIZE, "$thing/up/property/%s/%s", product_id, device_name);

    if (size < 0 || size > sizeof(report_topic_name) - 1) {
        printf("pub topic content length not enough! content size:%d  buf size:%d", size, (int)sizeof(report_topic_name));
    }

    /* 创建邮箱 */
    tos_mail_q_create(&mail_q, ch20_value_pool, 3, sizeof(ch20_data_t));

    HAL_NVIC_DisableIRQ(USART3_4_IRQn);

    if (ch20_parser_init() == -1) {
        printf("ch20 parser init fail\r\n");
        return;
    }

    while (1) {
        /* 通过接收邮件来读取数据 */
        HAL_NVIC_EnableIRQ(USART3_4_IRQn);
        tos_mail_q_pend(&mail_q, (uint8_t*)&ch20_value, &mail_size, TOS_TIME_FOREVER);
        HAL_NVIC_DisableIRQ(USART3_4_IRQn);

        /* 接收到之后打印信息 */
        ch20_ppm_value = ch20_value.data / 1000.0;
        printf("ch20 value: %.3f\r\n", ch20_ppm_value);

        /* OLED显示值 */
        sprintf(ch20_ppm_str, "%.3f ppm(mg/m3)", ch20_ppm_value);
        OLED_ShowString(0, 2, (uint8_t*)ch20_ppm_str, 16);

        /* 上报值 */
        memset(payload, 0, sizeof(payload));
        snprintf(payload, sizeof(payload), REPORT_DATA_TEMPLATE, ch20_ppm_value);

        if (lightness > 100) {
            lightness = 0;
        }

        if (tos_tf_module_mqtt_pub(report_topic_name, QOS0, payload) != 0) {
            printf("module mqtt pub fail\n");
            break;
        } else {
            printf("module mqtt pub success\n");
        }

        tos_sleep_ms(5000);
    }
}

//tos task
__API__ k_err_t rust_tos_task_create(k_task_t *task, 
                                                    char *name, 
                                                    k_task_entry_t entry, 
                                                    void *arg, k_prio_t prio, 
                                                    k_stack_t *stk_base, 
                                                    long unsigned int stk_size, 
                                                    k_timeslice_t timeslice)
{
    return tos_task_create(task,name,entry,arg,prio,stk_base,stk_size,timeslice);
}

__API__ k_err_t rust_tos_task_destroy(k_task_t *task){
    return tos_task_destroy(task);
}

__API__ k_err_t rust_tos_task_create_dyn(k_task_t **task, 
                                                    char *name,
                                                    k_task_entry_t entry,
                                                    void *arg,
                                                    k_prio_t prio,
                                                    long unsigned int  stk_size,
                                                    k_timeslice_t timeslice)
{
    return tos_task_create_dyn(task,name,entry,arg,prio,stk_size,timeslice);
}

__API__ k_err_t rust_tos_task_delay(k_tick_t delay){
    return tos_task_delay(delay);
}

__API__ k_err_t rust_tos_task_delay_abort(k_task_t *task){
    return tos_task_delay_abort(task);
}

__API__ k_err_t rust_tos_task_suspend(k_task_t *task){
    return tos_task_suspend(task);
}

__API__ k_err_t rust_tos_task_resume(k_task_t *task){
    return tos_task_resume(task);
}

__API__ k_err_t rust_tos_task_prio_change(k_task_t *task, k_prio_t prio_new){
    return tos_task_prio_change(task,prio_new);
}

__API__ void    rust_tos_task_yield(void){
    return tos_task_yield();
}

__API__ k_task_t *rust_tos_task_curr_task_get(void){
    return tos_task_curr_task_get();
}

__API__ k_err_t rust_tos_task_stack_draught_depth(k_task_t *task, int *depth){
    return tos_task_stack_draught_depth(task,depth);
}

__API__ void rust_tos_task_walkthru(k_task_walker_t walker){
    return tos_task_walkthru(walker);
}

//tos_mail

__API__ k_err_t rust_tos_mail_q_create(k_mail_q_t *mail_q, void *pool, size_t mail_cnt, size_t mail_size){
    return tos_mail_q_create(mail_q,pool,mail_cnt,mail_size);
}

// __API__ k_err_t rust_tos_mail_q_post(k_mail_q_t *mail_q, void *mail_buf, size_t mail_size){
//     return tos_mail_q_post(mail_q,mail_buf,mail_size);
// }

// __API__ k_err_t rust_tos_mail_q_pend(k_mail_q_t *mail_q, void *mail_buf, size_t *mail_size, k_tick_t timeout){
//     return tos_mail_q_pend(mail_q,mail_buf,mail_size,timeout);
// }


//tos event
__API__ k_err_t rust_tos_event_create(k_event_t *event, k_event_flag_t init_flag){
    return tos_event_create(event,init_flag);
}

__API__ k_err_t rust_tos_event_destroy(k_event_t *event){
    return tos_event_destroy(event);
}
__API__ k_err_t rust_tos_event_pend(k_event_t *event, k_event_flag_t flag_expect, k_event_flag_t *flag_match, k_tick_t timeout, k_opt_t opt_pend){
    return tos_event_pend(event,flag_expect,flag_match,timeout,opt_pend);
}

__API__ k_err_t rust_tos_event_post(k_event_t *event, k_event_flag_t flag){
    return tos_event_post(event,flag);
}

__API__ k_err_t rust_tos_event_post_keep(k_event_t *event, k_event_flag_t flag){
    return tos_event_post_keep(event,flag);
}


//tos_sem
__API__ k_err_t rust_tos_sem_create_max(k_sem_t *sem, k_sem_cnt_t init_count, k_sem_cnt_t max_count){
    return tos_sem_create_max(sem,init_count,max_count);
}

__API__ k_err_t rust_tos_sem_create(k_sem_t *sem, k_sem_cnt_t init_count){
    return tos_sem_create(sem,init_count);
}

__API__ k_err_t rust_tos_sem_create_max_dyn(k_sem_t **sem, k_sem_cnt_t init_count, k_sem_cnt_t max_count){
    return tos_sem_create_max_dyn(sem,init_count,max_count);
}

__API__ k_err_t rust_tos_sem_create_dyn(k_sem_t **sem, k_sem_cnt_t init_count){
    return tos_sem_create_dyn(sem,init_count);
}

__API__ k_err_t rust_tos_sem_destroy(k_sem_t *sem){
    return tos_sem_destroy(sem);
}

__API__ k_err_t rust_tos_sem_pend(k_sem_t *sem, k_tick_t timeout){
    return tos_sem_pend(sem,timeout);
}

__API__ k_err_t rust_tos_sem_post(k_sem_t *sem){
    return tos_sem_post(sem);
}

__API__ k_err_t rust_tos_sem_post_all(k_sem_t *sem){
    return tos_sem_post_all(sem);
}   


//tos_chr_fifo
k_err_t rust_tos_chr_fifo_create(k_chr_fifo_t *chr_fifo, void *buffer, size_t  size){
    return tos_chr_fifo_create(chr_fifo,buffer,size);
}


k_err_t rust_tos_chr_fifo_destroy(k_chr_fifo_t *chr_fifo){
    return tos_chr_fifo_destroy(chr_fifo);
}


k_err_t rust_tos_chr_fifo_create_dyn(k_chr_fifo_t *chr_fifo, size_t  fifo_size){
    return tos_chr_fifo_create_dyn(chr_fifo,fifo_size);
}


k_err_t rust_tos_chr_fifo_destroy_dyn(k_chr_fifo_t *chr_fifo){
    return tos_chr_fifo_destroy_dyn(chr_fifo);
}


k_err_t rust_tos_chr_fifo_push(k_chr_fifo_t *chr_fifo, uint8_t data){
    return tos_chr_fifo_push(chr_fifo,data);
}

int rust_tos_chr_fifo_push_stream(k_chr_fifo_t *chr_fifo, uint8_t *stream, size_t size){
    return tos_chr_fifo_push_stream(chr_fifo,stream,size);
}


k_err_t rust_tos_chr_fifo_pop(k_chr_fifo_t *chr_fifo, uint8_t *out){
    k_err_t kerr = tos_chr_fifo_pop(chr_fifo,out);
    return kerr;
}


int rust_tos_chr_fifo_pop_stream(k_chr_fifo_t *chr_fifo, uint8_t *buffer, size_t size){
    return tos_chr_fifo_pop_stream(chr_fifo,buffer,size);
}


k_err_t rust_tos_chr_fifo_flush(k_chr_fifo_t *chr_fifo){
    return tos_chr_fifo_flush(chr_fifo);
}


int rust_tos_chr_fifo_is_empty(k_chr_fifo_t *chr_fifo){
    return tos_chr_fifo_is_empty(chr_fifo);
}


int rust_tos_chr_fifo_is_full(k_chr_fifo_t *chr_fifo){
    return tos_chr_fifo_is_full(chr_fifo);
}



//CMSIS
int rust_osKernelRunning(){
    return osKernelRunning();
}

// uint32_t
uint32_t rust_osKernelSysTick(){
    return osKernelSysTick();
}

//system management
__API__ int rust_tos_knl_is_running(){
    return tos_knl_is_running();
}

__API__ void    rust_tos_knl_irq_enter(void){
    return tos_knl_irq_enter();
}

__API__ void    rust_tos_knl_irq_leave(void){
    return tos_knl_irq_leave();
}

__API__ k_err_t rust_tos_knl_sched_lock(void){
    return tos_knl_sched_lock();
}

__API__ k_err_t rust_tos_knl_sched_unlock(void){
    return tos_knl_sched_unlock();
}

//start of time management
__API__ k_tick_t    rust_tos_systick_get(void){
    return tos_systick_get();
}

__API__ void        rust_tos_systick_set(k_tick_t tick){
    return tos_systick_set(tick);
}

__API__ k_time_t    rust_tos_tick2millisec(k_tick_t tick){
    return tos_tick2millisec(tick);
}

__API__ k_tick_t    rust_tos_millisec2tick(k_time_t millisec){
    return tos_millisec2tick(millisec);
}

__API__ k_err_t     rust_tos_sleep_ms(k_time_t millisec){
    return tos_sleep_ms(millisec);
}

__API__ k_err_t     rust_tos_sleep_hmsm(k_time_t hour, k_time_t minute, k_time_t second, k_time_t millisec){
    return tos_sleep_hmsm(hour,minute,second,millisec);
}
//end


void rust_print(const char *msg) {
    printf("%s\r\n", msg);
}

void rust_print_char(const char *msg) {
    printf("This char is %c.\r\n", *msg);
}


void rust_print_num(size_t num){
    printf("this num = %d\n",num);
}

void rust_print_i32(int num){
    printf("this num = %d\n",num);
}



//OLED
void rust_oled_init(){
    OLED_Init();
}

void rust_oled_clear(){
    OLED_Clear();
}

void rust_oled_print(unsigned int x, unsigned int y, char *msg) {
    OLED_ShowString(x, y, (uint8_t*)msg, 16);
}


//wifi
int rust_wifi_init() {
    return esp8266_tencent_firmware_sal_init(HAL_UART_PORT_2);
}

void rust_wifi_connect(const char *ssid, const char *psd) {
    esp8266_tencent_firmware_join_ap(ssid, psd);
}


//
void rust_sleep(unsigned int ms) {
    tos_sleep_ms(ms);
}
