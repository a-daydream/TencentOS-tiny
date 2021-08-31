/*

    TencentOS API wrapper for rust world

*/

#include "tos_k.h"
#include "esp8266_tencent_firmware.h"
#include "tencent_firmware_module_wrapper.h"
#include "ch20_parser.h"
#include "oled.h"
#include "user_config.h"
#include "cmsis_os.h"

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
    printf("sizeof(void *) * 5 : %d\r\n",sizeof(void *) * 5 );
    // OLED_Init();
    // OLED_Clear();
    // OLED_ShowString(0, 0, (uint8_t*)str, 16);


    // int ret = 0;
    // int size = 0;
    // int lightness = 0;
    // mqtt_state_t state;

    // char *product_id = PRODUCT_ID;
    // char *device_name = DEVICE_NAME;
    // char *key = DEVICE_KEY;

    // device_info_t dev_info;
    // memset(&dev_info, 0, sizeof(device_info_t));

    // size_t mail_size;
    // float  ch20_ppm_value;
    // char   ch20_ppm_str[20];


    // /* OLED显示日志 */
    // OLED_ShowString(0, 2, (uint8_t*)"connecting...", 16);

    // /*

    //     Please Choose your AT Port first, default is HAL_UART_2(USART2)

    //     网络层初始化, 这个函数位于

    //         tos/devices/esp8266_tencent_firmware/esp8266_tencent_firmware.c

    // */

    // ret = esp8266_tencent_firmware_sal_init(HAL_UART_PORT_2);

    // if (ret < 0) {
    //     printf("esp8266 tencent firmware sal init fail, ret is %d\r\n", ret);
    // }

    // esp8266_tencent_firmware_join_ap(WIFI_NAME, WIFI_PASSWORD);

    // strncpy(dev_info.product_id, product_id, PRODUCT_ID_MAX_SIZE);
    // strncpy(dev_info.device_name, device_name, DEVICE_NAME_MAX_SIZE);
    // strncpy(dev_info.device_serc, key, DEVICE_SERC_MAX_SIZE);
    // tos_tf_module_info_set(&dev_info, TLS_MODE_PSK);

    // /*

    //     DEFAULT_MQTT_PARAMS 定义在 tos/net/tencent_firmware_module_wrapper/tencent_firmware_module_wrapper.h

    //     typedef struct mqtt_param_st {
    //         tls_mode_t  tls_mode;
    //         uint32_t    command_timeout;
    //         uint32_t    keep_alive_interval_ms;
    //         uint8_t     clean_session;
    //         uint8_t     auto_connect_enable;
    //     } mqtt_param_t;
    //     #define DEFAULT_MQTT_PARAMS { TLS_MODE_PSK, MQTT_COMMAND_TIMEOUT, 240, 1, 1 }

    // */
    // mqtt_param_t init_params = DEFAULT_MQTT_PARAMS;
    // if (tos_tf_module_mqtt_conn(init_params) != 0) {
    //     printf("module mqtt conn fail\r\n");
    // } else {
    //     printf("module mqtt conn success\r\n");
    // }

    // if (tos_tf_module_mqtt_state_get(&state) != -1) {
    //     printf("MQTT: %s\n", state == MQTT_STATE_CONNECTED ? "CONNECTED" : "DISCONNECTED");
    // }

    // /* 开始订阅topic */
    // size = snprintf(report_reply_topic_name, TOPIC_NAME_MAX_SIZE, "$thing/down/property/%s/%s", product_id, device_name);

    // if (size < 0 || size > sizeof(report_reply_topic_name) - 1) {
    //     printf("sub topic content length not enough! content size:%d  buf size:%d", size, (int)sizeof(report_reply_topic_name));
    // }
    // if (tos_tf_module_mqtt_sub(report_reply_topic_name, QOS0, default_message_handler) != 0) {
    //     printf("module mqtt sub fail\n");
    // } else {
    //     printf("module mqtt sub success\n");
    // }

    // memset(report_topic_name, sizeof(report_topic_name), 0);
    // size = snprintf(report_topic_name, TOPIC_NAME_MAX_SIZE, "$thing/up/property/%s/%s", product_id, device_name);

    // if (size < 0 || size > sizeof(report_topic_name) - 1) {
    //     printf("pub topic content length not enough! content size:%d  buf size:%d", size, (int)sizeof(report_topic_name));
    // }

    // /* 创建邮箱 */
    // tos_mail_q_create(&mail_q, ch20_value_pool, 3, sizeof(ch20_data_t));

    // HAL_NVIC_DisableIRQ(USART3_4_IRQn);

    // if (ch20_parser_init() == -1) {
    //     printf("ch20 parser init fail\r\n");
    //     return;
    // }

    // while (1) {
    //     /* 通过接收邮件来读取数据 */
    //     HAL_NVIC_EnableIRQ(USART3_4_IRQn);
    //     tos_mail_q_pend(&mail_q, (uint8_t*)&ch20_value, &mail_size, TOS_TIME_FOREVER);
    //     HAL_NVIC_DisableIRQ(USART3_4_IRQn);

    //     /* 接收到之后打印信息 */
    //     ch20_ppm_value = ch20_value.data / 1000.0;
    //     printf("ch20 value: %.3f\r\n", ch20_ppm_value);

    //     /* OLED显示值 */
    //     sprintf(ch20_ppm_str, "%.3f ppm(mg/m3)", ch20_ppm_value);
    //     OLED_ShowString(0, 2, (uint8_t*)ch20_ppm_str, 16);

    //     /* 上报值 */
    //     memset(payload, 0, sizeof(payload));
    //     snprintf(payload, sizeof(payload), REPORT_DATA_TEMPLATE, ch20_ppm_value);

    //     if (lightness > 100) {
    //         lightness = 0;
    //     }

    //     if (tos_tf_module_mqtt_pub(report_topic_name, QOS0, payload) != 0) {
    //         printf("module mqtt pub fail\n");
    //         break;
    //     } else {
    //         printf("module mqtt pub success\n");
    //     }

    //     tos_sleep_ms(5000);
    // }
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

__API__ k_err_t rust_tos_task_delay(k_tick_t *delay){
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
//end of task 

// tos mmheap
__API__ k_err_t rust_tos_mmheap_pool_add(void *pool_start, long unsigned int  pool_size){
    return tos_mmheap_pool_add(pool_start,pool_size);
}

__API__ k_err_t rust_tos_mmheap_pool_rmv(void *pool_start){
    return tos_mmheap_pool_rmv(pool_start);
}

__API__ void   *rust_tos_mmheap_alloc(long unsigned int  size){
    return tos_mmheap_alloc(size);
}

__API__ void   *rust_tos_mmheap_aligned_alloc(long unsigned int  size, long unsigned int  align){
    return tos_mmheap_aligned_alloc(size,align);
}

__API__ void   *rust_tos_mmheap_realloc(void *ptr, long unsigned int  size){
    return tos_mmheap_realloc(ptr,size);
}

__API__ void    rust_tos_mmheap_free(void *ptr){
    return tos_mmheap_free(ptr);
}

__API__ k_err_t rust_tos_mmheap_pool_check(void *pool_start, k_mmheap_info_t *info){
    return tos_mmheap_pool_check(pool_start,info);
}

__API__ k_err_t rust_tos_mmheap_check(k_mmheap_info_t *info){
    return tos_mmheap_check(info);
}
// end of mmheap

// tos mmblk
__API__ k_err_t rust_tos_mmblk_pool_create(k_mmblk_pool_t *mbp, void *pool_start, long unsigned int blk_num, long unsigned int blk_size){
    return tos_mmblk_pool_create(mbp,pool_start,blk_num,blk_size);
}

__API__ k_err_t rust_tos_mmblk_pool_destroy(k_mmblk_pool_t *mbp){
    return tos_mmblk_pool_destroy(mbp);
}

__API__ k_err_t rust_tos_mmblk_alloc(k_mmblk_pool_t *mbp, void **blk){
    return tos_mmblk_alloc(mbp,blk);
}

__API__ k_err_t rust_tos_mmblk_free(k_mmblk_pool_t *mbp, void *blk){
    return tos_mmblk_free(mbp,blk);
}
// end of tos mmblk


//**********************************tos mutex ****************************************
__API__ k_err_t rust_tos_mutex_create(k_mutex_t *mutex){
    return tos_mutex_create(mutex);
}

__API__ k_err_t rust_tos_mutex_create_dyn(k_mutex_t **mutex){
    return tos_mutex_create_dyn(mutex);
}

__API__ k_err_t rust_tos_mutex_destroy(k_mutex_t *mutex){
    return tos_mutex_destroy(mutex);
}

__API__ k_err_t rust_tos_mutex_pend_timed(k_mutex_t *mutex, k_tick_t *timeout){
    return tos_mutex_pend_timed(mutex,timeout);
}

__API__ k_err_t rust_tos_mutex_pend(k_mutex_t *mutex){
    return tos_mutex_pend(mutex);
}

__API__ k_err_t rust_tos_mutex_post(k_mutex_t *mutex){
    return tos_mutex_post(mutex);
}

__KNL__ void rust_mutex_release(k_mutex_t *mutex){
    return mutex_release(mutex);
}
//**********************************tos mutex ****************************************

#if TOS_CFG_COMPLETION_EN > 0
//**********************************tos completion ****************************************
__API__ k_err_t rust_tos_completion_create(k_completion_t *completion){
    return tos_completion_create(completion);
}

__API__ k_err_t rust_tos_completion_destroy(k_completion_t *completion){
    return tos_completion_destroy(completion);
}

__API__ k_err_t rust_tos_completion_pend_timed(k_completion_t *completion, k_tick_t timeout){
    return tos_completion_pend_timed(completion,timeout);
}

__API__ k_err_t rust_tos_completion_pend(k_completion_t *completion){
    return tos_completion_pend(completion);
}

__API__ k_err_t rust_tos_completion_post(k_completion_t *completion){
    return  tos_completion_post(completion);
}

__API__ k_err_t rust_tos_completion_post_all(k_completion_t *completion){
    return tos_completion_post_all(completion);
}

__API__ k_err_t rust_tos_completion_reset(k_completion_t *completion){
    return tos_completion_reset(completion);
}

__API__ int rust_tos_completion_is_done(k_completion_t *completion){
    return tos_completion_is_done(completion);
}
//**********************************tos completion ****************************************
#endif

#if TOS_CFG_COUNTDOWNLATCH_EN > 0
//**********************************tos countdownlatch ****************************************
__API__ k_err_t rust_tos_countdownlatch_create(k_countdownlatch_t *countdownlatch, k_countdownlatch_cnt_t count){
    return tos_countdownlatch_create(countdownlatch,count);
}

__API__ k_err_t rust_tos_countdownlatch_destroy(k_countdownlatch_t *countdownlatch){
    return tos_countdownlatch_destroy(countdownlatch);
}

__API__ k_err_t rust_tos_countdownlatch_pend_timed(k_countdownlatch_t *countdownlatch, k_tick_t timeout){
    return tos_countdownlatch_pend_timed(countdownlatch,timeout);
}

__API__ k_err_t rust_tos_countdownlatch_pend(k_countdownlatch_t *countdownlatch){
    return tos_countdownlatch_pend(countdownlatch);
}

__API__ k_err_t rust_tos_countdownlatch_post(k_countdownlatch_t *countdownlatch){
    return tos_countdownlatch_post(countdownlatch);
}

__API__ k_err_t rust_tos_countdownlatch_reset(k_countdownlatch_t *countdownlatch, k_countdownlatch_cnt_t count){
    return rust_tos_countdownlatch_reset(countdownlatch,count);
}
//**********************************tos countdownlatch ****************************************
#endif

#if TOS_CFG_BARRIER_EN > 0
//**********************************tos barrier ****************************************
__API__ k_err_t rust_tos_barrier_create(k_barrier_t *barrier, k_barrier_cnt_t count){
    return tos_barrier_create(barrier, count);
}

__API__ k_err_t rust_tos_barrier_destroy(k_barrier_t *barrier){
    return tos_barrier_destroy(barrier);
}

__API__ k_err_t rust_tos_barrier_pend(k_barrier_t *barrier){
    return tos_barrier_pend(barrier);
}

__API__ k_err_t rust_tos_barrier_reset(k_barrier_t *barrier, k_barrier_cnt_t count){
    return tos_barrier_reset(barrier, count);
}

//**********************************tos barrier ****************************************
#endif

#if (TOS_CFG_SEM_EN > 0u) && (TOS_CFG_MUTEX_EN > 0u)
//**********************************tos rwlock ****************************************
__API__ k_err_t rust_tos_rwlock_create(k_rwlock_t *rwlock){
    return tos_rwlock_create(rwlock);
}

__API__ k_err_t rust_tos_rwlock_destroy(k_rwlock_t *rwlock){
    return tos_rwlock_destroy(rwlock);
}

__API__ k_err_t rust_tos_rwlock_rpend_timed(k_rwlock_t *rwlock, k_tick_t timeout){
    return tos_rwlock_rpend_timed(rwlock,timeout);
}

__API__ k_err_t rust_tos_rwlock_rpend(k_rwlock_t *rwlock){
    return tos_rwlock_rpend(rwlock);
}

__API__ k_err_t rust_tos_rwlock_rpend_try(k_rwlock_t *rwlock){
    return tos_rwlock_rpend_try(rwlock);
}

__API__ k_err_t rust_tos_rwlock_wpend_timed(k_rwlock_t *rwlock, k_tick_t timeout){
    return tos_rwlock_wpend_timed(rwlock,timeout);
}

__API__ k_err_t rust_tos_rwlock_wpend(k_rwlock_t *rwlock){
    return tos_rwlock_wpend(rwlock);
}

__API__ k_err_t rust_tos_rwlock_wpend_try(k_rwlock_t *rwlock){
    return tos_rwlock_wpend_try(rwlock);
}

__API__ k_err_t rust_tos_rwlock_rpost(k_rwlock_t *rwlock){
    return tos_rwlock_rpost(rwlock);
}

__API__ k_err_t rust_tos_rwlock_wpost(k_rwlock_t *rwlock){
    return tos_rwlock_wpost(rwlock);
}

__API__ k_err_t rust_tos_rwlock_post(k_rwlock_t *rwlock){
    return tos_rwlock_post(rwlock);
}
//**********************************tos rwlock ****************************************
#endif

#if TOS_CFG_MESSAGE_QUEUE_EN > 0u
//**********************************tos msg_q ****************************************
__API__ k_err_t rust_tos_msg_q_create(k_msg_q_t *msg_q, void *pool, unsigned long int msg_cnt){
    return tos_msg_q_create(msg_q,pool,msg_cnt);
}

__API__ k_err_t rust_tos_msg_q_destroy(k_msg_q_t *msg_q){
    return tos_msg_q_destroy(msg_q);
}

__API__ k_err_t rust_tos_msg_q_create_dyn(k_msg_q_t *msg_q, unsigned long int msg_cnt){
    return tos_msg_q_create_dyn(msg_q,msg_cnt);
}

__API__ k_err_t rust_tos_msg_q_destroy_dyn(k_msg_q_t *msg_q){
    return tos_msg_q_destroy_dyn(msg_q);
}

__API__ k_err_t rust_tos_msg_q_flush(k_msg_q_t *msg_q){
    return tos_msg_q_flush(msg_q);
}

__API__ k_err_t rust_tos_msg_q_pend(k_msg_q_t *msg_q, void **msg_ptr, k_tick_t timeout){
    return tos_msg_q_pend(msg_q,msg_ptr,timeout);
}

__API__ k_err_t rust_tos_msg_q_post(k_msg_q_t *msg_q, void *msg_ptr){
    return tos_msg_q_post(msg_q,msg_ptr);
}

__API__ k_err_t rust_tos_msg_q_post_all(k_msg_q_t *msg_q, void *msg_ptr){
    return tos_msg_q_post_all(msg_q,msg_ptr);
}
//**********************************tos msg_q ****************************************
#endif

#if TOS_CFG_MAIL_QUEUE_EN > 0u
//**********************************tos tos_mail ****************************************
__API__ k_err_t rust_tos_mail_q_create(k_mail_q_t *mail_q, void *pool, size_t mail_cnt, size_t mail_size){
    return tos_mail_q_create(mail_q,pool,mail_cnt,mail_size);
}

__API__ k_err_t rust_tos_mail_q_destroy(k_mail_q_t *mail_q){
    return tos_mail_q_destroy(mail_q);
}

__API__ k_err_t rust_tos_mail_q_create_dyn(k_mail_q_t *mail_q, size_t mail_cnt, size_t mail_size){
    return tos_mail_q_create_dyn(mail_q,mail_cnt,mail_size);
}

__API__ k_err_t rust_tos_mail_q_destroy_dyn(k_mail_q_t *mail_q){
    return tos_mail_q_destroy_dyn(mail_q);
}

__API__ k_err_t rust_tos_mail_q_flush(k_mail_q_t *mail_q){
    return tos_mail_q_flush(mail_q);
}

__API__ k_err_t rust_tos_mail_q_pend(k_mail_q_t *mail_q, void *mail_buf, size_t *mail_size, k_tick_t *timeout){
    return tos_mail_q_pend(mail_q,mail_buf,mail_size,timeout);
}

__API__ k_err_t rust_tos_mail_q_post(k_mail_q_t *mail_q, void *mail_buf, size_t mail_size){
    return tos_mail_q_post(mail_q,mail_buf,mail_size);
}

__API__ k_err_t rust_tos_mail_q_post_all(k_mail_q_t *mail_q, void *mail_buf, size_t mail_size){
    return tos_mail_q_post_all(mail_q,mail_buf,mail_size);
}
//**********************************tos tos_mail ****************************************
#endif

#if TOS_CFG_PRIORITY_MESSAGE_QUEUE_EN > 0u
//**********************************tos prio msg_q ****************************************
__API__ k_err_t rust_tos_prio_msg_q_create(k_prio_msg_q_t *prio_msg_q, void *pool, size_t msg_cnt){
    return tos_prio_msg_q_create(prio_msg_q,pool,msg_cnt);
}

__API__ k_err_t rust_tos_prio_msg_q_destroy(k_prio_msg_q_t *prio_msg_q){
    return tos_prio_msg_q_destroy(prio_msg_q);
}

__API__ k_err_t rust_tos_prio_msg_q_create_dyn(k_prio_msg_q_t *prio_msg_q, size_t msg_cnt){
    return tos_prio_msg_q_create_dyn(prio_msg_q,msg_cnt);
}

__API__ k_err_t rust_tos_prio_msg_q_destroy_dyn(k_prio_msg_q_t *prio_msg_q){
    return tos_prio_msg_q_destroy_dyn(prio_msg_q);
}

__API__ k_err_t rust_tos_prio_msg_q_flush(k_prio_msg_q_t *prio_msg_q){
    return tos_prio_msg_q_flush(prio_msg_q);
}

__API__ k_err_t rust_tos_prio_msg_q_pend(k_prio_msg_q_t *prio_msg_q, void **msg_ptr, k_tick_t timeout){
    return tos_prio_msg_q_pend(prio_msg_q, msg_ptr, timeout);
}

__API__ k_err_t rust_tos_prio_msg_q_post(k_prio_msg_q_t *prio_msg_q, void *msg_ptr, k_prio_t prio){
    return tos_prio_msg_q_post(prio_msg_q,msg_ptr,prio);
}

__API__ k_err_t rust_tos_prio_msg_q_post_all(k_prio_msg_q_t *prio_msg_q, void *msg_ptr, k_prio_t prio){
    return tos_prio_msg_q_post_all(prio_msg_q,msg_ptr,prio);
}
//**********************************tos prio msg_q ****************************************
#endif


#if TOS_CFG_PRIORITY_MAIL_QUEUE_EN > 0u
//**********************************tos prio mail_q ****************************************
__API__ k_err_t rust_tos_prio_mail_q_create(k_prio_mail_q_t *prio_mail_q, void *pool, size_t mail_cnt, size_t mail_size){
    return tos_prio_mail_q_create(prio_mail_q,pool,mail_cnt,mail_size);
}

__API__ k_err_t rust_tos_prio_mail_q_destroy(k_prio_mail_q_t *prio_mail_q){
    return tos_prio_mail_q_destroy(prio_mail_q);
}

__API__ k_err_t rust_tos_prio_mail_q_create_dyn(k_prio_mail_q_t *prio_mail_q, size_t mail_cnt, size_t mail_size){
    return tos_prio_mail_q_create_dyn(prio_mail_q,mail_cnt,mail_size);
}

__API__ k_err_t rust_tos_prio_mail_q_destroy_dyn(k_prio_mail_q_t *prio_mail_q){
    return tos_prio_mail_q_destroy_dyn(prio_mail_q);
}

__API__ k_err_t rust_tos_prio_mail_q_flush(k_prio_mail_q_t *prio_mail_q){
    return tos_prio_mail_q_flush(prio_mail_q);
}

__API__ k_err_t rust_tos_prio_mail_q_pend(k_prio_mail_q_t *prio_mail_q, void *mail_buf, size_t *mail_size, k_tick_t timeout){
    return tos_prio_mail_q_pend(prio_mail_q,mail_buf,mail_size,timeout);
}

__API__ k_err_t rust_tos_prio_mail_q_post(k_prio_mail_q_t *prio_mail_q, void *mail_buf, size_t mail_size, k_prio_t prio){
    return tos_prio_mail_q_post(prio_mail_q,mail_buf,mail_size,prio);
}

__API__ k_err_t rust_tos_prio_mail_q_post_all(k_prio_mail_q_t *prio_mail_q, void *mail_buf, size_t mail_size, k_prio_t prio){
    return tos_prio_mail_q_post_all(prio_mail_q,mail_buf,mail_size,prio);
}
//**********************************tos prio mail_q ****************************************
#endif

//**********************************tos ring_q ****************************************
__API__ k_err_t rust_tos_ring_q_create(k_ring_q_t *ring_q, void *pool, size_t item_cnt, size_t item_size){
    return tos_ring_q_create(ring_q,pool,item_cnt,item_size);
}

__API__ k_err_t rust_tos_ring_q_destroy(k_ring_q_t *ring_q){
    return tos_ring_q_destroy(ring_q);
}

__API__ k_err_t rust_tos_ring_q_create_dyn(k_ring_q_t *ring_q, size_t item_cnt, size_t item_size){
    return tos_ring_q_create_dyn(ring_q,item_cnt,item_size);
}

__API__ k_err_t rust_tos_ring_q_destroy_dyn(k_ring_q_t *ring_q){
    return tos_ring_q_destroy_dyn(ring_q);
}

__API__ k_err_t rust_tos_ring_q_enqueue(k_ring_q_t *ring_q, void *item, size_t item_size){
    return tos_ring_q_enqueue(ring_q,item,item_size);
}

__API__ k_err_t rust_tos_ring_q_dequeue(k_ring_q_t *ring_q, void *item, size_t *item_size){
    return tos_ring_q_dequeue(ring_q,item,item_size);
}

__API__ k_err_t rust_tos_ring_q_flush(k_ring_q_t *ring_q){
    return tos_ring_q_flush(ring_q);
}

__API__ int     rust_tos_ring_q_is_empty(k_ring_q_t *ring_q){
    return tos_ring_q_is_empty(ring_q);
}

__API__ int     rust_tos_ring_q_is_full(k_ring_q_t *ring_q){
    return tos_ring_q_is_full(ring_q);
}
//**********************************tos ring_q ****************************************

//**********************************tos bin_heap ****************************************
__API__ k_err_t rust_tos_bin_heap_create(k_bin_heap_t *bin_heap, void *pool, size_t item_cnt, size_t item_size, k_bin_heap_cmp cmp){
    return tos_bin_heap_create(bin_heap,pool,item_cnt,item_size,cmp);
}

__API__ k_err_t rust_tos_bin_heap_destroy(k_bin_heap_t *bin_heap){
    return tos_bin_heap_destroy(bin_heap);
}

__API__ k_err_t rust_tos_bin_heap_create_dyn(k_bin_heap_t *bin_heap, size_t item_cnt, size_t item_size, k_bin_heap_cmp cmp){
    return tos_bin_heap_create_dyn(bin_heap,item_cnt,item_size,cmp);
}

__API__ k_err_t rust_tos_bin_heap_destroy_dyn(k_bin_heap_t *bin_heap){
    return tos_bin_heap_destroy_dyn(bin_heap);
}

__API__ k_err_t rust_tos_bin_heap_push(k_bin_heap_t *bin_heap, void *item, size_t item_size){
    return tos_bin_heap_push(bin_heap,item,item_size);
}

__API__ k_err_t rust_tos_bin_heap_pop(k_bin_heap_t *bin_heap, void *item, size_t *item_size){
    return tos_bin_heap_pop(bin_heap,item,item_size);
}

__API__ k_err_t rust_tos_bin_heap_flush(k_bin_heap_t *bin_heap){
    return tos_bin_heap_flush(bin_heap);
}

__API__ int rust_tos_bin_heap_is_empty(k_bin_heap_t *bin_heap){
    return tos_bin_heap_is_empty(bin_heap);
}

__API__ int rust_tos_bin_heap_is_full(k_bin_heap_t *bin_heap){
    return tos_bin_heap_is_full(bin_heap);
}
//**********************************tos bin_heap ****************************************


//**********************************tos pro_q ****************************************
__API__ k_err_t rust_tos_prio_q_create(k_prio_q_t *prio_q, void *mgr_array, void *pool, size_t item_cnt, size_t item_size){
    return tos_prio_q_create(prio_q,mgr_array,pool,item_cnt,item_size);
}

__API__ k_err_t rust_tos_prio_q_destroy(k_prio_q_t *prio_q){
    return tos_prio_q_destroy(prio_q);
}

__API__ k_err_t rust_tos_prio_q_create_dyn(k_prio_q_t *prio_q, size_t item_cnt, size_t item_size){
    return tos_prio_q_create_dyn(prio_q,item_cnt,item_size);
}

__API__ k_err_t rust_tos_prio_q_destroy_dyn(k_prio_q_t *prio_q){
    return tos_prio_q_destroy_dyn(prio_q);
}

__API__ k_err_t rust_tos_prio_q_enqueue(k_prio_q_t *prio_q, void *item, size_t item_size, k_prio_t prio){
    return tos_prio_q_enqueue(prio_q,item,item_size,prio);
}

__API__ k_err_t rust_tos_prio_q_dequeue(k_prio_q_t *prio_q, void *item, size_t *item_size, k_prio_t *prio){
    return tos_prio_q_dequeue(prio_q,item,item_size,prio);
}

__API__ k_err_t rust_tos_prio_q_flush(k_prio_q_t *prio_q){
    return tos_prio_q_flush(prio_q);
}

__API__ int rust_tos_prio_q_is_empty(k_prio_q_t *prio_q){
    return tos_prio_q_is_empty(prio_q);
}

__API__ int rust_tos_prio_q_is_full(k_prio_q_t *prio_q){
    return tos_prio_q_is_full(prio_q);
}
//**********************************tos pro_q ****************************************

#if TOS_CFG_TIMER_EN > 0u
//**********************************tos timer ****************************************
__API__ k_err_t rust_tos_timer_create(k_timer_t *tmr, k_tick_t *delay, k_tick_t *period,
                                        k_timer_callback_t callback, void *cb_arg, k_opt_t opt)
{
    return tos_timer_create(tmr,delay,period,callback,cb_arg,opt);
}

__API__ k_err_t rust_tos_timer_destroy(k_timer_t *tmr){
    return tos_timer_destroy(tmr);
}

__API__ k_err_t rust_tos_timer_start(k_timer_t *tmr){
    return tos_timer_start(tmr);
}

__API__ k_err_t rust_tos_timer_stop(k_timer_t *tmr){
    return tos_timer_stop(tmr);
}

__API__ k_err_t rust_tos_timer_delay_change(k_timer_t *tmr, k_tick_t delay){
    return tos_timer_delay_change(tmr,delay);
}

__API__ k_err_t rust_tos_timer_period_change(k_timer_t *tmr, k_tick_t period){
    return tos_timer_period_change(tmr,period);
}

__KNL__ void rust_timer_update(void){
    return timer_update();
}

__KNL__ k_err_t rust_timer_init(void){
    return timer_init();
}

__KNL__ k_tick_t rust_timer_next_expires_get(void){
    return timer_next_expires_get();
}
//**********************************tos timer ****************************************
#endif

//**********************************tos stopwatch ****************************************
__API__ k_err_t rust_tos_stopwatch_create(k_stopwatch_t *stopwatch){
    return tos_stopwatch_create(stopwatch);
}

__API__ k_err_t rust_tos_stopwatch_destroy(k_stopwatch_t *stopwatch){
    return tos_stopwatch_destroy(stopwatch);
}

__API__ k_err_t rust_tos_stopwatch_countdown(k_stopwatch_t *stopwatch, k_tick_t tick){
    return tos_stopwatch_countdown(stopwatch,tick);
}

__API__ k_err_t rust_tos_stopwatch_countdown_ms(k_stopwatch_t *stopwatch, k_time_t millisec){
    return tos_stopwatch_countdown_ms(stopwatch,millisec);
}

__API__ void rust_tos_stopwatch_delay(k_tick_t tick){
    return tos_stopwatch_delay(tick);
}

__API__ void rust_tos_stopwatch_delay_ms(k_time_t millisec){
    return tos_stopwatch_delay_ms(millisec);
}

__API__ k_tick_t rust_tos_stopwatch_remain(k_stopwatch_t *stopwatch){
    return tos_stopwatch_remain(stopwatch);
}

__API__ k_time_t rust_tos_stopwatch_remain_ms(k_stopwatch_t *stopwatch){
    return tos_stopwatch_remain_ms(stopwatch);
}

__API__ int rust_tos_stopwatch_is_expired(k_stopwatch_t *stopwatch){
    return tos_stopwatch_is_expired(stopwatch);
}
//**********************************tos stopwatch ****************************************

//**********************************tos bitmap ****************************************
__API__ k_err_t     rust_tos_bitmap_create_empty(k_bitmap_t *bitmap, k_bmtbl_t *bitmap_tbl, uint32_t bit_max){
    return tos_bitmap_create_empty(bitmap,bitmap_tbl,bit_max);
}

__API__ k_err_t     rust_tos_bitmap_create_full(k_bitmap_t *bitmap, k_bmtbl_t *bitmap_tbl, uint32_t bit_max){
    return tos_bitmap_create_full(bitmap,bitmap_tbl,bit_max);
}

__API__ k_err_t     rust_tos_bitmap_destroy(k_bitmap_t *bitmap){
    return tos_bitmap_destroy(bitmap);
}

__API__ k_err_t     rust_tos_bitmap_set(k_bitmap_t *bitmap, uint32_t bit){
    return tos_bitmap_set(bitmap,bit);
}

__API__ k_err_t     rust_tos_bitmap_reset(k_bitmap_t *bitmap, uint32_t bit){
    return tos_bitmap_reset(bitmap,bit);
}

__API__ int         rust_tos_bitmap_is_set(k_bitmap_t *bitmap, uint32_t bit){
    return tos_bitmap_is_set(bitmap,bit);
}

__API__ int         rust_tos_bitmap_is_reset(k_bitmap_t *bitmap, uint32_t bit){
    return tos_bitmap_is_reset(bitmap,bit);
}
__API__ int         rust_tos_bitmap_lsb(k_bitmap_t *bitmap){
    return tos_bitmap_lsb(bitmap);
}
//**********************************tos bitmap ****************************************

//**********************************tos pm management ****************************************
#if TOS_CFG_PWR_MGR_EN > 0u
#if TOS_CFG_TICKLESS_EN > 0u
__API__ k_err_t rust_tos_pm_cpu_lpwr_mode_set(k_cpu_lpwr_mode_t cpu_lpwr_mode){
    return tos_pm_cpu_lpwr_mode_set(cpu_lpwr_mode);
}
#endif
__API__ k_err_t rust_tos_pm_device_register(k_pm_device_t *device){
    return tos_pm_device_register(device);
}
#endif

#if TOS_CFG_TICKLESS_EN > 0u
__API__ void rust_tos_tickless_wkup_alarm_install(k_cpu_lpwr_mode_t mode, k_tickless_wkup_alarm_t *wkup_alarm){
    return tos_tickless_wkup_alarm_install(mode,wkup_alarm);
}

__API__ k_err_t rust_tos_tickless_wkup_alarm_init(k_cpu_lpwr_mode_t mode){
    return tos_tickless_wkup_alarm_init(mode);
}
#endif
//**********************************tos pm management ****************************************



//tos event
__API__ k_err_t rust_tos_event_create(k_event_t *event, k_event_flag_t init_flag){
    return tos_event_create(event,init_flag);
}

__API__ k_err_t rust_tos_event_destroy(k_event_t *event){
    return tos_event_destroy(event);
}
__API__ k_err_t rust_tos_event_pend(k_event_t *event, k_event_flag_t flag_expect, k_event_flag_t *flag_match, k_tick_t *timeout, k_opt_t opt_pend){
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

__API__ k_err_t rust_tos_sem_pend(k_sem_t *sem, k_tick_t *timeout){
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
osStatus rust_osKernelInitialize(void){
    return osKernelInitialize();
}

osStatus rust_osKernelStart(void){
    return osKernelStart();
}

int32_t rust_osKernelRunning(void){
    return osKernelRunning();
}

uint32_t rust_osKernelSysTick(void){
    return osKernelSysTick();
}

osThreadId rust_osThreadCreate(const osThreadDef_t *thread_def, void *argument){
    return osThreadCreate(thread_def,argument);
}

osThreadId rust_osThreadGetId(void){
    return osThreadGetId();
}

osStatus rust_osThreadTerminate(osThreadId thread_id){
    return osThreadTerminate(thread_id);
}

osStatus rust_osThreadYield(void){
    return osThreadYield();
}

osStatus rust_osThreadSetPriority(osThreadId thread_id, osPriority priority){
    return osThreadSetPriority(thread_id,priority);
}

osPriority rust_osThreadGetPriority(osThreadId thread_id){
    return osThreadGetPriority(thread_id);
}

osStatus rust_osDelay(uint32_t millisec){
    return osDelay(millisec);
}

osTimerId rust_osTimerCreate(const osTimerDef_t *timer_def, os_timer_type type, void *argument){
    return osTimerCreate(timer_def,type,argument);
}

osStatus rust_osTimerStart(osTimerId timer_id, uint32_t millisec){
    return osTimerStart(timer_id,millisec);
}

osStatus rust_osTimerStop(osTimerId timer_id){
    return osTimerStop(timer_id);
}

osStatus rust_osTimerDelete(osTimerId timer_id){
    return osTimerDelete(timer_id);
}

osMutexId rust_osMutexCreate(const osMutexDef_t *mutex_def){
    return osMutexCreate(mutex_def);
}

osStatus rust_osMutexWait(osMutexId mutex_id, uint32_t millisec){
    return osMutexWait(mutex_id,millisec);
}

osStatus rust_osMutexRelease(osMutexId mutex_id){
    return osMutexRelease(mutex_id);
}

osStatus rust_osMutexDelete(osMutexId mutex_id){
    return osMutexDelete(mutex_id);
}



osSemaphoreId rust_osSemaphoreCreate(const osSemaphoreDef_t *semaphore_def, int32_t count){
    return osSemaphoreCreate(semaphore_def,count);
}

int32_t rust_osSemaphoreWait(osSemaphoreId semaphore_id, uint32_t millisec){
    return osSemaphoreWait(semaphore_id,millisec);
}

osStatus rust_osSemaphoreRelease(osSemaphoreId semaphore_id){
    return osSemaphoreRelease(semaphore_id);
}

osStatus rust_osSemaphoreDelete(osSemaphoreId semaphore_id){
    return osSemaphoreDelete(semaphore_id);
}


osPoolId rust_osPoolCreate(const osPoolDef_t *pool_def){
    return osPoolCreate(pool_def);
}

void * rust_osPoolAlloc(osPoolId pool_id){
    return osPoolAlloc(pool_id);
}

void * rust_osPoolCAlloc(osPoolId pool_id){
    return osPoolCAlloc(pool_id);
}

osStatus rust_osPoolFree(osPoolId pool_id, void *block){
    return osPoolFree(pool_id,block);
}

#if TOS_CFG_MESSAGE_QUEUE_EN > 0u
osMessageQId rust_osMessageCreate(const osMessageQDef_t *queue_def, osThreadId thread_id){
    return osMessageCreate(queue_def,thread_id);
}

osStatus rust_osMessagePut(osMessageQId queue_id, uint32_t info, uint32_t millisec){
    return osMessagePut(queue_id,info,millisec);
}

osEvent rust_osMessageGet(osMessageQId queue_id, uint32_t millisec){
    return osMessageGet(queue_id, millisec);
}
#endif



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
    printf("this num = %d\r\n",num);
}

void rust_print_i32(int num){
    printf("this num = %d\r\n",num);
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
