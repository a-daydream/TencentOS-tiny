

use cty::*;

// #[repr(C)]
// pub enum  k_err {
//     K_ERR_NONE                                  = 0isize,

//     K_ERR_BARRIER_COUNT_INVALID                 = 5isize,
//     K_ERR_BARRIER_OVERFLOW,

//     K_ERR_BITMAP_EXCEED                         = 10isize,

//     K_ERR_BIN_HEAP_FULL                     = 15isize,
//     K_ERR_BIN_HEAP_EMPTY,
//     K_ERR_BIN_HEAP_ITEM_SIZE_NOT_MATCH,

//     K_ERR_COMPLETION_OVERFLOW               = 25isize,

//     K_ERR_COUNTDOWNLATCH_OVERFLOW           = 50isize,

//     K_ERR_DELAY_ZERO                            = 100isize,
//     K_ERR_DELAY_FOREVER,

//     K_ERR_EVENT_PEND_OPT_INVALID                = 200isize,

//     K_ERR_IN_IRQ                                    = 400isize,

//     K_ERR_KNL_NOT_RUNNING                       = 500isize,
//     K_ERR_KNL_RUNNING,

//     K_ERR_LOCK_NESTING_OVERFLOW                 = 600isize,

//     K_ERR_MMBLK_POOL_FULL                       = 700isize,
//     K_ERR_MMBLK_POOL_EMPTY,
//     K_ERR_MMBLK_INVALID_BLK_SIZE,
//     K_ERR_MMBLK_INVALID_POOL_ADDR,
//     K_ERR_MMBLK_POOL_OUT_OF_MEMORY,
//     K_ERR_MMBLK_OUT_OF_MEMORY,

//     K_ERR_MMHEAP_INVALID_POOL_ADDR              = 800isize,
//     K_ERR_MMHEAP_INVALID_POOL_SIZE,
//     K_ERR_MMHEAP_POOL_OVERFLOW,
//     K_ERR_MMHEAP_POOL_ALREADY_EXIST,
//     K_ERR_MMHEAP_POOL_NOT_EXIST,

//     K_ERR_MUTEX_NOT_OWNER                       = 1000isize,
//     K_ERR_MUTEX_NESTING,
//     K_ERR_MUTEX_NESTING_OVERFLOW,

//     K_ERR_OBJ_PTR_NULL                          = 1100isize,
//     K_ERR_OBJ_INVALID,
//     K_ERR_OBJ_INVALID_ALLOC_TYPE,

//     K_ERR_OUT_OF_MEMORY                         = 1150isize,

//     K_ERR_PEND_NOWAIT                           = 1200isize,
//     K_ERR_PEND_SCHED_LOCKED,
//     K_ERR_PEND_ABNORMAL,
//     K_ERR_PEND_TIMEOUT,
//     K_ERR_PEND_DESTROY,
//     K_ERR_PEND_OWNER_DIE,

//     K_ERR_PM_DEVICE_ALREADY_REG                 = 1300isize,
//     K_ERR_PM_DEVICE_OVERFLOW,
//     K_ERR_PM_WKUP_SOURCE_NOT_INSTALL,

//     K_ERR_PRIO_Q_EMPTY                      = 1400isize,
//     K_ERR_PRIO_Q_FULL,
//     K_ERR_PRIO_Q_SLOT_NOT_TAKEN,
//     K_ERR_PRIO_Q_ITEM_SIZE_NOT_MATCH,

//     K_ERR_RING_Q_FULL                           = 1500isize,
//     K_ERR_RING_Q_EMPTY,
//     K_ERR_RING_Q_ITEM_SIZE_NOT_MATCH,

//     K_ERR_RWLOCK_READERS_TO_MANY            = 1600isize,
//     K_ERR_RWLOCK_IS_READING,
//     K_ERR_RWLOCK_IS_WRITTING,
//     K_ERR_RWLOCK_NOT_READING,
//     K_ERR_RWLOCK_NOT_WRITTING,
//     K_ERR_RWLOCK_NOT_TAKEN,
//     K_ERR_RWLOCK_WAITING_WRITERS_TO_MANY,

//     K_ERR_SCHED_LOCKED                          = 1700isize,
//     K_ERR_SCHED_NOT_LOCKED,

//     K_ERR_SEM_OVERFLOW                          = 1800isize,

//     K_ERR_TASK_ALREADY_CREATED                  = 1900isize,
//     K_ERR_TASK_DESTROY_IDLE,
//     K_ERR_TASK_NOT_DELAY,
//     K_ERR_TASK_PRIO_INVALID,
//     K_ERR_TASK_RESUME_SELF,
//     K_ERR_TASK_SUSPENDED,
//     K_ERR_TASK_SUSPEND_IDLE,
//     K_ERR_TASK_STK_OVERFLOW,
//     K_ERR_TASK_STK_SIZE_INVALID,
//     K_ERR_TASK_OUT_OF_MEMORY,

//     K_ERR_TICKLESS_WKUP_ALARM_NOT_INSTALLED     = 2000isize,
//     K_ERR_TICKLESS_WKUP_ALARM_NO_INIT,
//     K_ERR_TICKLESS_WKUP_ALARM_INIT_FAILED,

//     K_ERR_TIMER_INACTIVE                        = 2100isize,
//     K_ERR_TIMER_DELAY_FOREVER,
//     K_ERR_TIMER_PERIOD_FOREVER,
//     K_ERR_TIMER_INVALID_DELAY,
//     K_ERR_TIMER_INVALID_PERIOD,
//     K_ERR_TIMER_INVALID_STATE,
//     K_ERR_TIMER_INVALID_OPT,
//     K_ERR_TIMER_STOPPED,
//     K_ERR_TIMER_RUNNING,

//     K_ERR_SEM_OUT_OF_MEMORY                     = 2200isize,

//     K_ERR_MUTEX_OUT_OF_MEMORY                   = 2300isize,
// }

// #[repr(C)]
// pub enum osStatus{
//     osOK                    =     0,       //< function completed; no error or event occurred.
//     osEventSignal           =  0x08,       //< function completed; signal event occurred.
//     osEventMessage          =  0x10,       //< function completed; message event occurred.
//     osEventMail             =  0x20,       //< function completed; mail event occurred.
//     osEventTimeout          =  0x40,       //< function completed; timeout occurred.
//     osErrorParameter        =  0x80,       //< parameter error: a mandatory parameter was missing or specified an incorrect object.
//     osErrorResource         =  0x81,       //< resource not available: a specified resource was not available.
//     osErrorTimeoutResource  =  0xC1,       //< resource not available within given time: a specified resource was not available within the timeout period.
//     osErrorISR              =  0x82,       //< not allowed in ISR context: the function cannot be called from interrupt service routines.
//     osErrorISRRecursive     =  0x83,       //< function called multiple times from ISR with same object.
//     osErrorPriority         =  0x84,       //< system cannot determine priority or thread has illegal priority.
//     osErrorNoMemory         =  0x85,       //< system is out of memory: it was impossible to allocate or reserve memory for the operation.
//     osErrorValue            =  0x86,       //< value of a parameter is out of range.
//     osErrorOS               =  0xFF,       //< unspecified RTOS error: run-time error but no other error message fits.
//     os_status_reserved      =  0x7FFFFFFF,  //< prevent from enum down-size compiler optimization.
// } 


// #[repr(u8)]
// pub enum  knl_obj_alloc_type {
//     KNL_OBJ_ALLOC_TYPE_NONE = 0,
//     KNL_OBJ_ALLOC_TYPE_STATIC,
//     KNL_OBJ_ALLOC_TYPE_DYNAMIC,
// }

// #[repr(C)]
// pub struct knl_obj {
//     alloc_type : knl_obj_alloc_type,
// }

// #[repr(C)]
// pub struct  k_list{
//     next : *mut k_list,
//     prev : *mut k_list,
// } 

// #[repr(C)]
// pub struct pend_obj {
//     list : k_list,
// }

// #[repr(C)]
// pub struct k_ring_q {
//     knl_rustobj : knl_obj,

//     head : u16,
//     tail : u16,
//     total : usize,
//     pool : *mut u8,
    
//     item_size : usize,
//     item_cnt : usize,
// }

// #[repr(C)]
// pub struct k_mail_q {
//     knl_rustobj : knl_obj,
//     pend_rustobj: pend_obj,
//     ring_q: k_ring_q,
// }

/// These glue functions are from tosglue.c
extern {
    //CMSIS
    // osStatus osKernelStart(void);
    // int32_t osKernelRunning(void);
    pub fn rust_osKernelRunning() -> i32;
    pub fn rust_osKernelSysTick() ->u32;

    //system management
    // __API__ int tos_knl_is_running(void)
    pub fn rust_tos_knl_is_running() -> i32;

    // tos_mutex
    // k_err_t tos_mutex_create(k_mutex_t *mutex);
    // pub fn rust_tos_mutex_create()

    //tos_mail
    // __API__ k_err_t tos_mail_q_create(k_mail_q_t *mail_q, void *pool, size_t mail_cnt, size_t mail_size);
    // pub fn rust_tos_mail_q_create( pool : *mut c_void , mail_cnt : usize , mail_size : usize ) -> k_err; 
    // __API__ k_err_t tos_mail_q_post(k_mail_q_t *mail_q, void *mail_buf, size_t mail_size);
    // pub fn rust_tos_mail_q_post(mail_q : *mut k_mail_q,mail_buf : *mut c_void,mail_size : usize) -> k_err;
    // __API__ k_err_t rust_tos_mail_q_pend(k_mail_q_t *mail_q, void *mail_buf, size_t *mail_size, k_tick_t timeout)
    // pub fn rust_tos_mail_q_pend(mail_q : *mut k_mail_q, mail_buf : *mut c_void, mail_size : usize, time_out : u32) -> k_err;


    //OLED
    pub fn rust_oled_print(x : u32, y : u32 ,msg: *const u8);
    pub fn rust_oled_init();
    pub fn rust_oled_clear();  


    pub fn rust_print_num(num : u32);

    pub fn rust_print(msg: *const u8);
    
    //wifi
    pub fn rust_wifi_init() -> c_int;
    pub fn rust_wifi_connect(ssid: *const u8, passwd: *const u8);
    
    pub fn rust_sleep(ms: u32);
    pub fn rust_mqtt_daemon() -> c_void;
}
