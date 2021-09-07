#![no_std]

extern crate cortex_m;


mod bridge;

use crate::k_err_en::*;

use crate::bridge::*;
use cty::*;



#[no_mangle]
pub extern "C" fn application_entry_rust() -> c_void {
    unsafe {
        rust_print(b"[+] Welcome to the RUST-WORLD in TencentOS :)\r\n\0".as_ptr());

        rust_oled_init();
        rust_oled_clear();
        rust_oled_print(0,0,b"TencentOS RUST\0".as_ptr());
       


        tos_meet_rust_daemon();
        // ************************start tos_task_test***************************
        // rust_test_tos_task_create();
        // rust_test_tos_task_destroy();
        // rust_test_tos_task_delay();
        // rust_test_tos_task_delay_abort(); // some bug
        // rust_test_tos_task_suspend_resume();
        // rust_test_tos_task_prio_change();
        // rust_test_tos_task_yeild();
        //*************************end of tos_task_test**************************
        
  
        
        // ************************start tos mmblk***************************
        // rust_test_tos_mmblk_pool_create();
        // rust_test_tos_mmblk_pool_destroy();
        // rust_test_tos_mmblk_alloc();
        // rust_test_tos_mmblk_free();
        //*************************end of tos mmblk**************************
        
        // ************************start tos mutex***************************
        // rust_test_tos_mutex_create();
        // rust_test_tos_mutex_create_dyn();
        // rust_test_tos_mutex_destroy(); 
        // rust_test_tos_mutex_pend();
        // rust_test_tos_mutex_pend_timed();
        // rust_test_tos_mutex_post();
        // ************************end tos mutex***************************

        // ************************start tos sem***************************
        // rust_test_tos_sem_create();
        // rust_test_tos_sem_create_dyn();
        // rust_test_tos_sem_destroy();
        // rust_test_tos_sem_pend();
        // rust_test_tos_sem_pend_timed(); 
        // rust_test_tos_sem_post_all(); // some bug
        // ************************end tos sem***************************
        //end
        
        // ************************start tos event***************************
        // rust_test_tos_event_create();
        // rust_test_tos_event_destroy();
        // rust_test_tos_event_pend_all();
        // rust_test_tos_event_pend_any();
        // rust_test_tos_event_pend_timed(); 
        // rust_test_tos_event_post_keep();
        // ************************end tos event***************************

        // ************************start tos mail_q***************************
        // rust_test_tos_mail_queue_create();
        // rust_test_tos_mail_queue_destroy();
        // rust_test_tos_mail_queue_pend();
        // rust_test_tos_mail_queue_pend_dyn();
        // rust_test_tos_mail_queue_pend_timed();
        // rust_test_tos_mail_queue_post_all(); //some bug
        // rust_test_tos_mail_queue_flush();
        // ************************end tos mail_q***************************

        // ************************start tos timer***************************
        // rust_test_tos_timer_create();
        // rust_test_tos_timer_destroy();
        // rust_test_tos_timer_stop(); // some bug
        // rust_test_tos_timer_oneshot_functional();
        // rust_test_tos_timer_periodic_functional();// some bug 

        // ************************end tos timer***************************


        // ************************start tos message queue***************************
        // rust_test_tos_message_queue_create();
        // rust_test_tos_message_queue_destroy();
        // rust_test_tos_message_queue_pend(); // some bug
        // ************************end tos message queue***************************

        // ************************start tos_chr_fifo_test***********************
        // rust_test_tos_fifo_create();
        // rust_test_tos_fifo_destory();
        // rust_test_tos_fifo_char_push();
        // rust_test_tos_fifo_char_push_dyn();
        // rust_test_tos_fifo_stream_push();
        
        //*************************end of tos_chr_fifo_test**********************



              //to test
        // ************************start tos mmheap***************************
        // to do
        //*************************end of tos mmheap**************************
        //****************************start tos ring_q test********************************
        // to do
        //****************************end  tos ring_q test********************************
        
        // ************************start tos completion***************************
        // todo 
        // ************************end tos completion***************************
        
        // ************************start tos countdown latch***************************
        // todo 
        // ************************end tos countdown latch***************************

        // ************************start tos barrier***************************
        // todo 
        // ************************end tos barrier***************************

        // ************************start tos rwlock***************************
        // todo 
        // ************************end tos rwlock***************************

        // rust_sleep(5000u32);
        // rust_mqtt_daemon();
    }

    loop {
        unsafe {
            // rust_print(b"[+] This is a mqtt demo!".as_ptr());
            rust_sleep(5000u32);
        }
    }
}

//***************************tos_task test*****************************
extern "C" {
    #[no_mangle]
    static  mut k_idle_task : k_task_t;
    static mut k_curr_task : *mut k_task_t;
}
static mut test_task_stack_00 : [k_stack_t;512] = [0;512];
static mut test_task_stack_01 : [k_stack_t;512] = [0;512];
static mut test_task_stack_02 : [k_stack_t;512] = [0;512];


unsafe extern "C" fn  test_task_entry1(arg : *mut c_void) 
{
    let mut test_task_01 = k_task_t::default();
    let  task_name2 =   "test_task_entry2".as_ptr() as *mut c_char ;
    let  mut entry2  : k_task_entry_t = Some(test_task_entry2);
    let mut err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
        task_name2, entry2, 
        &mut 0 as *mut _ as *mut c_void, 
        2 as k_prio_t , 
        &mut test_task_stack_01[0], 
        512, 
        0);
    if(err != K_ERR_NONE ){
        rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
        return ;
    }
    while(true){
        rust_print("Hello,test_task_entry1!\r\n\0".as_ptr());
        rust_tos_sleep_ms(100);
    }
    
    return ;
}
unsafe extern "C" fn  test_task_entry2(arg : *mut c_void) 
{

    while(true){
        rust_print("Hello,entry2!\r\n\0".as_ptr());
        // rust_tos_task_yield();
        rust_tos_sleep_ms(100);
    }
    return ;
}

pub fn tos_meet_rust_daemon(){
    unsafe{
        let mut test_task_00 = k_task_t::default();
        
        let  task_name1 =   "test_task_entry1".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry1);
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name1, entry, 
            &mut 0 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        while(true){
            rust_tos_task_yield();
        }
        rust_print("RUST: tos_meet_rust_daemon pass\r\n\0".as_ptr());
    }
}



// static mut count_delay_abort : i32 = 0;
unsafe extern "C" fn  test_task_entry(arg : *mut c_void) 
{
    // count_delay_abort = count_delay_abort + 1;
    // let delay_1 : k_time_t = u32::MAX;
    // rust_tos_task_delay(rust_tos_millisec2tick(delay_1));
    rust_print("test_task_entry\r\n\0".as_ptr());
    // rust_tos_sleep_ms(1000);
    return ;
}

pub fn rust_test_tos_task_create(){
    unsafe{
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry);


        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                            task_name, entry, 
                                            0 as *mut c_void /*K_NULL*/, 
                                            9 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
                                            &mut test_task_stack_00[0], 
                                            512, 
                                            0);
        if(err != K_ERR_TASK_PRIO_INVALID){
            rust_print("RUST: rust_tos_task_create  test idle failed\r\n\0".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                    task_name, entry, 
                                    0 as *mut c_void /*K_NULL*/, 
                                    10 as k_prio_t /*K_TASK_PRIO_IDLE+1  10*/, 
                                    &mut test_task_stack_00[0], 
                                    512, 
                                    0);               
        if(err != K_ERR_TASK_PRIO_INVALID){
            rust_print("RUST: rust_tos_task_create out of prio failed\r\n\0".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                    task_name, entry, 
                                    0 as *mut c_void /*K_NULL*/, 
                                    3 as k_prio_t /*K_TASK_PRIO_IDLE+1  10*/, 
                                    &mut test_task_stack_00[0], 
                                    512, 
                                    0);                 
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy  failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_task_create pass\r\n\0".as_ptr());
    }
}


pub fn rust_test_tos_task_destroy(){
    unsafe{
        let mut err = rust_tos_task_destroy(&mut k_idle_task as *mut _);

        if(err != K_ERR_TASK_DESTROY_IDLE){
            rust_print("RUST: rust_tos_task_destroy  test idle failed\r\n\0".as_ptr());
            return ;
        }
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry);

        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            0 as *mut c_void /*K_NULL*/, 
            3 as k_prio_t /*K_TASK_PRIO_IDLE+1  10*/, 
            &mut test_task_stack_00[0], 
            512, 
            0);
               
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);  
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy test_task_00 failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_task_destroy pass\r\n\0".as_ptr()); 
    }
}



pub fn rust_test_tos_task_delay(){

    unsafe{
        // let mut count = 0;
        let mut begin: k_tick_t = 10;
        let mut end: k_tick_t = 10;
        let mut delay:*mut k_tick_t = 2000 as *mut k_tick_t;
        let mut deviation: k_tick_t = 10;
        
        begin = rust_tos_systick_get();
        
        let err = rust_tos_task_delay(delay);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_delay failed\r\n\0".as_ptr());
            return ;
        }
        end = rust_tos_systick_get();
        rust_print_num(begin);
        rust_print_num(end);
        rust_print("RUST: rust_test_tos_task_delay pass\r\n\0".as_ptr());
    }
}

unsafe extern "C" fn  test_tos_task_delay_abort_delay_task(arg : *mut c_void) 
{
    rust_print("enter test_tos_task_delay_abort_delay_task\r\n\0".as_ptr());
    let mut begin = rust_tos_systick_get();
    let mut delay:*mut k_tick_t = 2000 as *mut k_tick_t;
    let mut err =rust_tos_task_delay(delay);
    if(err != K_ERR_NONE){
        rust_print("RUST: rust_tos_task_delay 4000 failed\r\n\0".as_ptr());
        return ;
    }
    let mut end = rust_tos_systick_get();
    rust_print_num(begin);
    rust_print_num(end);
    rust_print("RUST: rust_tos_task_delay_abort\r\n\0".as_ptr());
    return ;
}
unsafe extern "C" fn  test_tos_task_delay_abort_task(arg : *mut c_void) 
{
    let mut test_task_01 = k_task_t::default();
       
    let  task_name =   "test_task".as_ptr() as *mut c_char ;
    let  mut entry  : k_task_entry_t = Some(test_tos_task_delay_abort_delay_task);
    let mut err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
        task_name, entry, 
        &mut test_task_01 as *mut _ as *mut c_void , 
        2 as k_prio_t, 
        &mut test_task_stack_01[0], 
        512, 
        0);
    // rust_tos_task_yield();
    err =rust_tos_task_delay_abort(&mut test_task_01 as *mut _);
    // rust_print_num(err as u32);
    if(err != K_ERR_NONE){
        rust_print("RUST: rust_tos_task_delay 4000 failed\r\n\0".as_ptr());
        return ;
    }
    return ;
}
pub fn rust_test_tos_task_delay_abort(){
    unsafe{
        let mut delay: k_tick_t = u32::MAX;
        // delay_abort_count = 0;
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_tos_task_delay_abort_task);
       
        

        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                                task_name, entry, 
                                                &mut test_task_00 as *mut _ as *mut c_void , 
                                                3 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
                                                &mut test_task_stack_00[0], 
                                                512, 
                                                0);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create  test delay abort failed\r\n\0".as_ptr());
            return ;
        }

        while(true){
            rust_tos_task_yield();
        }
 

    }
}


unsafe extern "C" fn  test_tos_task_suspend_resume_entry(arg : *mut c_void) 
{

    rust_print("test_task_entry\r\n\0".as_ptr());
    return ;
}
pub fn rust_test_tos_task_suspend_resume(){
    unsafe{
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_tos_task_suspend_resume_entry);
    
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            0 as *mut c_void , 
            4 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
            &mut test_task_stack_00[0], 
            512, 
            0);  
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create  test delay abort failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_suspend(&mut test_task_00 as *mut _);
       
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_suspend failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: before rust_tos_task_resume,task delay\r\n\0".as_ptr());
        let delay_1 : k_time_t = 1000;
        rust_tos_task_delay(rust_tos_millisec2tick(delay_1) as *mut k_tick_t);
        

        
        err = rust_tos_task_resume(&mut test_task_00 as *mut _);
        rust_print("RUST: after rust_tos_task_resume,task delay \r\n\0".as_ptr());
        rust_tos_task_delay(rust_tos_millisec2tick(delay_1) as *mut k_tick_t);
        
        

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_suspend_resume pass\r\n\0".as_ptr());
    }
 
}

pub fn rust_test_tos_task_prio_change(){
    unsafe{
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry);
    
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            0 as *mut c_void , 
            4 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
            &mut test_task_stack_00[0], 
            512, 
            0);  
        
        err = rust_tos_task_prio_change(&mut test_task_00 as *mut _,  9 as k_prio_t);
        if(err != K_ERR_TASK_PRIO_INVALID){
            rust_print("RUST: rust_tos_task_prio_change to idle  test failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_prio_change(&mut test_task_00 as *mut _,  4 as k_prio_t);
        if(test_task_00.prio != (4 as k_prio_t)){
            rust_print("RUST: rust_tos_task_prio_change  failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_prio_change(&mut test_task_00 as *mut _,  2 as k_prio_t);
        if(test_task_00.prio != (2 as k_prio_t)){
            rust_print("RUST: rust_tos_task_prio_change  failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_prio_change pass\r\n\0".as_ptr());
    }
}


unsafe extern "C" fn  test_tos_task_yeild_entry1(arg : *mut c_void) 
{
    let mut test_task_01 = k_task_t::default();
    let  task_name2 =   "test_tos_task_yeild_entry2".as_ptr() as *mut c_char ;
    let  mut entry2  : k_task_entry_t = Some(test_tos_task_yeild_entry2);
    let mut err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
        task_name2, entry2, 
        &mut 0 as *mut _ as *mut c_void, 
        2 as k_prio_t , 
        &mut test_task_stack_01[0], 
        512, 
        0);
    if(err != K_ERR_NONE ){
        rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
        return ;
    }
    while(true){
        rust_print("Hello,test_task_entry1!\r\n\0".as_ptr());
        // rust_tos_sleep_ms(100);
        rust_tos_task_yield();
    }
    
    return ;
}
unsafe extern "C" fn  test_tos_task_yeild_entry2(arg : *mut c_void) 
{

    while(true){
        rust_print("Hello,entry2!\r\n\0".as_ptr());
        rust_tos_task_yield();
        // rust_tos_sleep_ms(100);
    }
    return ;
}
pub fn rust_test_tos_task_yeild(){
    unsafe{
        let mut test_task_00 = k_task_t::default();
        
        let  task_name1 =   "test_tos_task_yeild_entry1".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_tos_task_yeild_entry1);
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name1, entry, 
            &mut 0 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        while(true){
            rust_tos_task_yield();
        }
        rust_print("RUST: rust_test_tos_task_yeild pass\r\n\0".as_ptr());
    }
}

//***************************end of tos_task test*****************************


//***************************start tos mmheap test*****************************
pub fn rust_test_tos_mmheap(){
    //to do
}

//***************************end of  tos mmheap test***************************

//***************************start tos mmblk test******************************
static mut mmblk_pool_buffer_00 : [u8; (5*0x20)] = [0;(5*0x20)];
static mut mmblk_pool_buffer_01 : [u8; (5*0x20)] = [0;(5*0x20)];
static mut mmblk_pool_buffer_02 : [u8; (5*0x20)] = [0;(5*0x20)];

pub fn rust_test_tos_mmblk_pool_create(){
    unsafe{
        let mut test_mmblk_pool_00 = k_mmblk_pool_t::default() ;
        let mut test_mmblk_pool_01 = k_mmblk_pool_t::default() ;
        let mut test_mmblk_pool_02 = k_mmblk_pool_t::default() ;

        let mut err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, &mut mmblk_pool_buffer_00 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_01 as *mut _, &mut mmblk_pool_buffer_01 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_02 as *mut _, &mut mmblk_pool_buffer_02 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_01);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_02);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, 0 as *mut c_void, 5, 32);
        if(err != K_ERR_OBJ_PTR_NULL){
            rust_print("RUST: rust_tos_mmblk_pool_create 0 failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_01 as *mut _, &mut mmblk_pool_buffer_01 as *mut _ as *mut c_void, 5, 33);
        if(err != K_ERR_MMBLK_INVALID_BLK_SIZE){
            rust_print("RUST: rust_tos_mmblk_pool_create size failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_mmblk_pool_create pass\r\n\0".as_ptr());
    }

}

pub fn rust_test_tos_mmblk_pool_destroy(){
    let mut  test_mmblk_pool_00 :  *mut *mut k_mmblk_pool_t = &mut 0 as *mut _ as *mut *mut k_mmblk_pool_t;
    unsafe{
        let mut err = rust_tos_mmblk_pool_destroy(*test_mmblk_pool_00);
        if (  err != K_ERR_OBJ_PTR_NULL ) {
            rust_print("RUST: rust_tos_mmblk_pool_destroy  failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_pool_destroy pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mmblk_alloc(){
    unsafe{
        let mut test_mmblk_pool_00 = k_mmblk_pool_t::default();
        let mut err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, &mut mmblk_pool_buffer_00 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n\0".as_ptr());
            return ;
        }
        let mut blk = test_mmblk_pool_00.free_list;
        let pblk = &mut blk as *mut *mut c_void;
        for i in 1..6  {
            err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, pblk);
            if(err != K_ERR_NONE){
                rust_print("RUST: rust_tos_mmblk_alloc failed\r\n\0".as_ptr());
                return ;
            }
            if(pblk == (0 as * mut  *mut c_void)){
                rust_print("RUST: blk ==null failed\r\n\0".as_ptr());
                return ;
            }
            
        }
        err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, 0 as * mut  *mut c_void);
        if(err != K_ERR_MMBLK_POOL_EMPTY){
            rust_print("RUST: rust_tos_mmblk_alloc empty\r\n\0".as_ptr());
            return ;
        }

        err =rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_alloc pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mmblk_free(){
    unsafe{
        let mut test_mmblk_pool_00 = k_mmblk_pool_t::default();
        let mut err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _,&mut  mmblk_pool_buffer_00 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut blk = test_mmblk_pool_00.free_list;
        let pblk = &mut blk as *mut *mut c_void;
        err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, pblk);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_alloc failed\r\n\0".as_ptr());
            return ;
        }
        if(pblk == (0 as * mut  *mut c_void)){
            rust_print("RUST: blk ==null failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_free(&mut test_mmblk_pool_00 as *mut _, blk);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_alloc failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_free(&mut test_mmblk_pool_00 as *mut _, blk);
        if(err != K_ERR_MMBLK_POOL_FULL){
            rust_print("RUST: rust_tos_mmblk_alloc free failed\r\n\0".as_ptr());
            return ;
        }

        err =rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_free pass\r\n\0".as_ptr());
    }
}

//***************************end of  tos mmblk test****************************

//****************************tos mutex test********************************
pub fn rust_test_tos_mutex_create(){
    unsafe{
        let mut  test_mutex_00 :  k_mutex_t = k_mutex_t::default();
        let mut err = rust_tos_mutex_create(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_destroy(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mutex_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mutex_create_dyn(){
    unsafe{
        let mut  test_mutex_00 :  k_mutex_t = k_mutex_t::default();
        let mut  test_mutex_00_dyn :  *mut *mut k_mutex_t = &mut test_mutex_00 as *mut _ as *mut *mut k_mutex_t;
        let mut err = rust_tos_mutex_create_dyn(test_mutex_00_dyn);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_create_dyn failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_destroy(*test_mutex_00_dyn);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mutex_create_dyn pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mutex_destroy(){
    unsafe{
        let mut  test_mutex_destroy :  *mut *mut k_mutex_t = &mut 0 as *mut _ as *mut *mut k_mutex_t;
        let err = rust_tos_mutex_destroy(*test_mutex_destroy);
        if(err != K_ERR_OBJ_PTR_NULL){
            rust_print("RUST: rust_tos_mutex_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mutex_destroy pass\r\n\0".as_ptr());
    }
}

unsafe extern "C" fn  test_mutex_pend_task_entry(arg : *mut c_void) 
{
    let mut mutex = arg as *mut _ as *mut k_mutex_t;
    let mut err = rust_tos_mutex_pend(mutex);
    if(err != K_ERR_NONE){
        rust_print("RUST: rust_tos_mutex_pend failed\r\n\0".as_ptr());
        return ;
    }
    rust_print("test_task_entry\r\n\0".as_ptr());
    err = rust_tos_mutex_post(mutex);
    if(err != K_ERR_NONE){
        rust_print("RUST: rust_tos_mutex_post failed\r\n\0".as_ptr());
        return ;
    }
    rust_tos_sleep_ms(1000);
    return ;
}
pub fn rust_test_tos_mutex_pend(){
    unsafe{
        let mut  test_mutex_00 :  k_mutex_t = k_mutex_t::default();
        let mut err = rust_tos_mutex_create(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_pend(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_pend failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: sleep before post \r\n\0".as_ptr());
        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mutex_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mutex_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mutex_00 as *mut _ as *mut c_void, 
            3 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        rust_tos_sleep_ms(1000);
        
        
        rust_print("RUST: sleep after  post \r\n\0".as_ptr());   
        err = rust_tos_mutex_post(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_post failed\r\n\0".as_ptr());
            return ;
        }

        rust_tos_sleep_ms(1000);
        

        err = rust_tos_mutex_destroy(&mut test_mutex_00 as *mut _);
        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        rust_print("RUST: rust_test_tos_mutex_pend pass \r\n\0".as_ptr());        
    }
}


unsafe extern "C" fn  test_mutex_pend_timed_task_entry(arg : *mut c_void) 
{
    let mut mutex = arg as *mut _ as *mut k_mutex_t;
    let pend_time :*mut  k_tick_t = 100 as *mut  k_tick_t;

    let mut err = rust_tos_mutex_pend_timed(mutex,pend_time);
    
    if (err == K_ERR_PEND_TIMEOUT){
        rust_print("RUST: rust_tos_mutex_pend_timed return K_ERR_PEND_TIMEOUT\r\n\0".as_ptr());
        return ;
    }

    rust_print("test_task_entry\r\n\0".as_ptr());
  
    rust_tos_sleep_ms(1000);
    return ;
}
pub fn rust_test_tos_mutex_pend_timed(){
    unsafe{
        let mut  test_mutex_00 :  k_mutex_t = k_mutex_t::default();
        let mut err = rust_tos_mutex_create(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_pend(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_pend failed\r\n\0".as_ptr());
            return ;
        }

        
        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mutex_pend_timed_task_entry".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mutex_pend_timed_task_entry);
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mutex_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        

        rust_print_num((*k_curr_task).prio as u32);
        let mut delay_ticks :  k_tick_t = 2000u32;
        rust_print("RUST: before  delay\r\n\0".as_ptr());
        rust_tos_sleep_ms(rust_tos_tick2millisec(delay_ticks));
        rust_print("RUST: after  2000 ticks\r\n\0".as_ptr());
        
        
        err = rust_tos_mutex_post(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_post failed\r\n\0".as_ptr());
            return ;
        }
        rust_print_num((*k_curr_task).prio as u32);

     
        rust_print("RUST: rust_test_tos_mutex_pend_timed pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mutex_post(){
    unsafe{
        let mut  test_mutex_00 :  k_mutex_t = k_mutex_t::default();
        let mut err = rust_tos_mutex_create(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_pend(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_pend failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_post(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_post failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_post(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_MUTEX_NOT_OWNER){
            rust_print("RUST: rust_tos_mutex_post failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mutex_destroy(&mut test_mutex_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mutex_destroy failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_mutex_post pass\r\n\0".as_ptr());
    }
}

//****************************end  of tos mutex test************************

//****************************tos sem test********************************
pub fn rust_test_tos_sem_create(){
    unsafe{
        let mut test_sem_00 : k_sem_t = k_sem_t::default();
        let mut test_sem_01 : k_sem_t = k_sem_t::default();
        let mut test_sem_02 : k_sem_t = k_sem_t::default();
        let mut sem_cnt : k_sem_cnt_t = 0u16;
        let mut err = rust_tos_sem_create(&mut test_sem_00 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 00 failed\r\n\0".as_ptr());
            return ;
        }

        sem_cnt = 255u16;
        let mut err = rust_tos_sem_create(&mut test_sem_01 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 01 failed\r\n\0".as_ptr());
            return ;
        }

        sem_cnt = 65535u16;
        let mut err = rust_tos_sem_create(&mut test_sem_02 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 02 failed\r\n\0".as_ptr());
            return ;
        }
        
        let mut err = rust_tos_sem_destroy(&mut test_sem_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }

        let mut err = rust_tos_sem_destroy(&mut test_sem_01 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }

        let mut err = rust_tos_sem_destroy(&mut test_sem_02 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_sem_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_sem_create_dyn(){
    unsafe{
        let mut test_sem_00 : k_sem_t = k_sem_t::default();
        let mut  test_sem_00_ptr :  *mut *mut k_sem_t = &mut test_sem_00 as *mut _ as *mut *mut k_sem_t;
        let mut test_sem_01 : k_sem_t = k_sem_t::default();
        let mut  test_sem_01_ptr :  *mut *mut k_sem_t = &mut test_sem_01 as *mut _ as *mut *mut k_sem_t;
        let mut test_sem_02 : k_sem_t = k_sem_t::default();
        let mut  test_sem_02_ptr :  *mut *mut k_sem_t = &mut test_sem_02 as *mut _ as *mut *mut k_sem_t;


        let mut sem_cnt : k_sem_cnt_t = 0u16;
        let mut err = rust_tos_sem_create_dyn(test_sem_00_ptr,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 00 failed\r\n\0".as_ptr());
            return ;
        }

        sem_cnt = 255u16;
        let mut err = rust_tos_sem_create_dyn(test_sem_01_ptr,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 01 failed\r\n\0".as_ptr());
            return ;
        }

        sem_cnt = 65535u16;
        let mut err = rust_tos_sem_create_dyn(test_sem_02_ptr,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create 02 failed\r\n\0".as_ptr());
            return ;
        }
        
        let mut err = rust_tos_sem_destroy(*test_sem_00_ptr);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }

        let mut err = rust_tos_sem_destroy(*test_sem_01_ptr);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }

        let mut err = rust_tos_sem_destroy(*test_sem_02_ptr);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_sem_create_dyn pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_sem_destroy(){
    unsafe{
        let mut  test_sem_00_ptr :  *mut *mut k_sem_t = &mut 0 as *mut _ as *mut *mut k_sem_t;
        let mut err = rust_tos_sem_destroy(*test_sem_00_ptr);
        if(err != K_ERR_OBJ_PTR_NULL){
            rust_print("RUST: rust_tos_sem_destroy return K_ERR_OBJ_PTR_NULL\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_sem_destroy pass\r\n\0".as_ptr());
    }
}

unsafe extern "C" fn  test_sem_pend_task_entry(arg : *mut c_void) 
{
    let mut sem = arg as *mut _ as *mut k_sem_t;
    let mut sem_cnt :*mut  k_tick_t = (u32::MAX) as *mut  k_tick_t ;

    while (true) {

        let mut err = rust_tos_sem_pend(sem,sem_cnt);
        if(err != K_ERR_NONE && err != K_ERR_PEND_DESTROY){
            rust_print("RUST: rust_tos_sem_pend failed\r\n\0".as_ptr());
            return ;
        }

        if(err == K_ERR_PEND_DESTROY){
            rust_print("RUST: K_ERR_PEND_DESTROY \r\n\0".as_ptr());
            return ;
        }
        rust_print("test_task_entry\r\n\0".as_ptr());
    }
}
pub fn rust_test_tos_sem_pend(){
    unsafe{
        let mut  test_sem_00 :  k_sem_t = k_sem_t::default();
        let mut sem_cnt : k_sem_cnt_t = 0u16;
        let mut err = rust_tos_sem_create(&mut test_sem_00 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create failed\r\n\0".as_ptr());
            return ;
        }

        
        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_sem_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_sem_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_sem_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
  
        err = rust_tos_sem_post(&mut test_sem_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_post failed\r\n\0".as_ptr());
            return ;
        }
  

        err = rust_tos_sem_destroy(&mut test_sem_00 as *mut _);


        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        rust_print("RUST: rust_test_tos_mutex_pend pass \r\n\0".as_ptr());    
    }
}

unsafe extern "C" fn  test_sem_pend_timed_task_entry(arg : *mut c_void) 
{
    let mut sem = arg as *mut _ as *mut k_sem_t;
    let pend_time : *mut k_tick_t = 100 as *mut k_tick_t;
    let mut err = rust_tos_sem_pend(sem,pend_time);
    

    if(err == K_ERR_PEND_TIMEOUT){
        rust_print("RUST: rust_tos_sem_pend return K_ERR_PEND_TIMEOUT\r\n\0".as_ptr());
        return ;
    }

    if(err != K_ERR_NONE){
        rust_print("RUST: rust_tos_sem_pend failed\r\n\0".as_ptr());
        return ;
    }
    rust_print("test_task_entry\r\n\0".as_ptr());

    return ;
}
pub fn rust_test_tos_sem_pend_timed(){
    unsafe{
        let mut  test_sem_00 :  k_sem_t = k_sem_t::default();
        let mut sem_cnt : k_sem_cnt_t = 0u16;
        let mut err = rust_tos_sem_create(&mut test_sem_00 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create failed\r\n\0".as_ptr());
            return ;
        }

        
        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_sem_pend_timed_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_sem_pend_timed_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_sem_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
            
   
        let mut delay_ticks :  k_tick_t = 2000u32;
        rust_print("RUST: before  delay\r\n\0".as_ptr());
        rust_tos_sleep_ms(rust_tos_tick2millisec(delay_ticks));
        rust_print("RUST: after  2000 ticks\r\n\0".as_ptr());

        err = rust_tos_sem_post(&mut test_sem_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_post failed\r\n\0".as_ptr());
            return ;
        }
  

        err = rust_tos_sem_destroy(&mut test_sem_00 as *mut _);


        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        rust_print("RUST: rust_test_tos_sem_pend_timed pass \r\n\0".as_ptr());    
    }
}

unsafe extern "C" fn  test_sem_postall_entry(arg : *mut c_void) 
{
    let mut sem = arg as *mut _ as *mut k_sem_t;
    let mut sem_cnt : *mut k_tick_t = (u32::MAX) as *mut k_tick_t;

    rust_print("RUST: test_sem_postall_entry \r\n\0".as_ptr());
    let mut err = rust_tos_sem_pend(sem,sem_cnt);
    
    if(err != K_ERR_NONE && err != K_ERR_PEND_DESTROY){
        rust_print("RUST: rust_tos_sem_pend failed\r\n\0".as_ptr());
        return ;
    }

    if(err == K_ERR_PEND_DESTROY){
        rust_print("RUST: K_ERR_PEND_DESTROY \r\n\0".as_ptr());
        rust_tos_sleep_ms(1000);
        return ;
    }
    rust_print("test_task_entry\r\n\0".as_ptr());
    rust_tos_sleep_ms(1000);
}

pub fn rust_test_tos_sem_post_all(){
    unsafe{
        let mut  test_sem_00 :  k_sem_t = k_sem_t::default();
        let mut sem_cnt : k_sem_cnt_t = 0u16;
        let mut err = rust_tos_sem_create(&mut test_sem_00 as *mut _,sem_cnt);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_task_00".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_sem_postall_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_sem_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }
        
        let mut test_task_01 = k_task_t::default();
        let  task_name =   "test_task_01".as_ptr() as *mut c_char ;
        err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
            task_name, entry, 
            &mut test_sem_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_01[0], 
            512, 
            0);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        // let mut test_task_02 = k_task_t::default();
        // let  task_name =   "test_task_02".as_ptr() as *mut c_char ;
        // let  mut entry  : k_task_entry_t = Some(test_sem_pend_task_entry);
        // err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
        //     task_name, entry, 
        //     &mut test_sem_00 as *mut _ as *mut c_void, 
        //     2 as k_prio_t , 
        //     &mut test_task_stack_02[0], 
        //     512, 
        //     0);
        // if(err != K_ERR_NONE){
        //     rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
        //     return ;
        // }

        rust_print("RUST: before rust_tos_sem_post_all \r\n\0".as_ptr());
        err = rust_tos_sem_post_all(&mut test_sem_00 as *mut _);
        rust_print_num(err as u32);
        err = rust_tos_sem_post(&mut test_sem_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_sem_post_all failed\r\n\0".as_ptr());
            return ;
        }
        // err = rust_tos_sem_post(&mut test_sem_00 as *mut _);
        // if(err != K_ERR_NONE){
        //     rust_print("RUST: rust_tos_sem_post failed\r\n\0".as_ptr());
        //     return ;
        // }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        err = rust_tos_task_destroy(&mut test_task_01 as *mut _);
        // err = rust_tos_task_destroy(&mut test_task_02 as *mut _);

        err = rust_tos_sem_destroy(&mut test_sem_00 as *mut _);
        rust_print("RUST: rust_tos_sem_post_all pass\r\n\0".as_ptr());
    }
}

//****************************end  of tos sem test************************

//****************************tos event test********************************
pub fn rust_test_tos_event_create(){
    unsafe{
        let mut test_event_00 : k_event_t = k_event_t::default();
        let mut test_event_01 : k_event_t = k_event_t::default();
        let mut test_event_02 : k_event_t = k_event_t::default();
        let flag1 : k_event_flag_t = 0;
        let flag2 : k_event_flag_t = 255;
        let flag3 : k_event_flag_t = 65535;

        let mut err = rust_tos_event_create(&mut test_event_00 as *mut _, flag1);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }
        err = rust_tos_event_create(&mut test_event_01 as *mut _, flag2);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }
        err = rust_tos_event_create(&mut test_event_02 as *mut _, flag3);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }

        err =rust_tos_event_destroy(&mut test_event_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err =rust_tos_event_destroy(&mut test_event_01 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err =rust_tos_event_destroy(&mut test_event_02 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_event_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_event_destroy(){
    unsafe{
        let mut  test_event_00_ptr :  *mut *mut k_event_t = &mut 0 as *mut _ as *mut *mut k_event_t;
        let mut err = rust_tos_event_destroy(*test_event_00_ptr);
        if(err != K_ERR_OBJ_PTR_NULL){
            rust_print("RUST: rust_tos_event_destroy return K_ERR_OBJ_PTR_NULL\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_event_destroy pass\r\n\0".as_ptr());
    }
}


static event_expect_00 : k_event_flag_t = (1 << 0) as k_event_flag_t;
static event_expect_01 : k_event_flag_t = (1 << 1) as k_event_flag_t;
static event_expect_02 : k_event_flag_t = (1 << 2) as k_event_flag_t;
static event_expect_dummy : k_event_flag_t = (1 << 3) as k_event_flag_t;
unsafe extern "C" fn  test_event_pend_all_task_entry(arg : *mut c_void) 
{
    
    let mut flag_match : k_event_flag_t = 0 as k_event_flag_t;
    let timeout = (u32::MAX  - 1) as *mut  k_tick_t;
    let tos_opt_all : k_opt_t = 0x0002 as k_opt_t;
    let tos_opt_clr : k_opt_t = 0x0004 as k_opt_t;

    while (true){
        let mut event = arg as *mut _ as *mut k_event_t;
        rust_print("before pend\r\n\0".as_ptr());
        let mut err = rust_tos_event_pend(event,  (event_expect_00 | event_expect_01 | event_expect_02), &mut flag_match, timeout,(tos_opt_all | tos_opt_clr));
        if(err == K_ERR_PEND_DESTROY){
            rust_print("RUST: rust_tos_event_pend return K_ERR_PEND_DESTROY\r\n\0".as_ptr());
            return ;
        }
    
        rust_print("test_task_entry\r\n\0".as_ptr());

    }

}
pub fn rust_test_tos_event_pend_all(){
    unsafe{
        let mut test_event_00 : k_event_t = k_event_t::default();
        let mut err = rust_tos_event_create(&mut test_event_00 as *mut _, 0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_event_pend_all".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_event_pend_all_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_event_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

       

        rust_print("RUST: before event_expect_dummy\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_dummy);
        rust_tos_sleep_ms(1000);

        rust_print("RUST: before event_expect_00\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_00);
        rust_tos_sleep_ms(1000);
  
        rust_print("RUST: before event_expect_all\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_00 | event_expect_01 | event_expect_02 );
        rust_tos_sleep_ms(1000);

        rust_tos_event_destroy(&mut test_event_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_event_pend_all pass\r\n\0".as_ptr());
    }
}

unsafe extern "C" fn  test_event_pend_any_task_entry(arg : *mut c_void) 
{
   
    let mut flag_match : k_event_flag_t = 0 as k_event_flag_t;
    let timeout = (u32::MAX  - 1) as *mut k_tick_t;
    let tos_opt_any : k_opt_t = 0x0001 as k_opt_t;
    let tos_opt_clr : k_opt_t = 0x0004 as k_opt_t;

    while (true){
        let mut event = arg as *mut _ as *mut k_event_t;
        rust_print("before pend\r\n\0".as_ptr());
        let mut err = rust_tos_event_pend(event,  (event_expect_00 | event_expect_01 | event_expect_02), &mut flag_match, timeout,(tos_opt_any | tos_opt_clr));
        if(err == K_ERR_PEND_DESTROY){
            rust_print("RUST: rust_tos_event_pend return K_ERR_PEND_DESTROY\r\n\0".as_ptr());
            return ;
        }
        rust_print("test_task_entry\r\n\0".as_ptr());
    }

}
pub fn rust_test_tos_event_pend_any(){
    unsafe{
        let mut test_event_00 : k_event_t = k_event_t::default();
        let mut err = rust_tos_event_create(&mut test_event_00 as *mut _, 0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_event_pend_all".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_event_pend_any_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_event_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: before event_expect_all\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_00 | event_expect_01 | event_expect_02 );
        rust_tos_sleep_ms(1000);
 

        rust_print("RUST: before event_expect_dummy\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_dummy);
        rust_tos_sleep_ms(1000);

        rust_print("RUST: before event_expect_00\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_00);
        rust_tos_sleep_ms(1000);
  
       
        rust_tos_event_destroy(&mut test_event_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_event_pend_any pass\r\n\0".as_ptr());
    }
}


unsafe extern "C" fn  test_event_pend_timed_task_entry(arg : *mut c_void) 
{
    
    let mut flag_match : k_event_flag_t = 0 as k_event_flag_t;
    let timeout = 100 as *mut  k_tick_t;
    let tos_opt_any : k_opt_t = 0x0001 as k_opt_t;
    let tos_opt_clr : k_opt_t = 0x0004 as k_opt_t;
    let mut event = arg as *mut _ as *mut k_event_t;
    while (true){
        
        rust_print("before pend\r\n\0".as_ptr());
        let mut err = rust_tos_event_pend(event,  (event_expect_00 | event_expect_01 | event_expect_02), &mut flag_match, timeout,(tos_opt_any | tos_opt_clr));

        if(err == K_ERR_PEND_TIMEOUT){
            rust_print("RUST: rust_tos_event_pend return K_ERR_PEND_TIMEOUT\r\n\0".as_ptr());
            return ;
        }
        rust_print("test_task_entry\r\n\0".as_ptr());

    }

}
pub fn rust_test_tos_event_pend_timed(){
    unsafe{
        let mut test_event_00 : k_event_t = k_event_t::default();
        let mut err = rust_tos_event_create(&mut test_event_00 as *mut _, 0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_event_pend_all".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_event_pend_timed_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_event_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

 
        let mut delay_ticks :  k_tick_t = 2000u32;
        rust_print("RUST: before  delay\r\n\0".as_ptr());
        rust_tos_sleep_ms(rust_tos_tick2millisec(delay_ticks));
        rust_print("RUST: after  2000 ticks\r\n\0".as_ptr());



        err =rust_tos_event_destroy(&mut test_event_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }
        
        rust_tos_task_destroy(&mut test_task_00 as *mut _);

        rust_print("RUST: rust_test_tos_event_pend_timed pass\r\n\0".as_ptr());
    }
}


unsafe extern "C" fn  test_event_post_keep_task_entry(arg : *mut c_void) 
{
    
    let mut flag_match : k_event_flag_t = 0 as k_event_flag_t;

    let tos_opt_all : k_opt_t = 0x0002 as k_opt_t;
    let tos_opt_clr : k_opt_t = 0x0004 as k_opt_t;
    let time_forever :*mut  k_tick_t = (u32::MAX ) as *mut  k_tick_t; 
    let mut event = arg as *mut _ as *mut k_event_t;
    while (true){
        
        rust_print("before pend\r\n\0".as_ptr());
        let mut err = rust_tos_event_pend(event,  (event_expect_00 | event_expect_01 | event_expect_02), &mut flag_match, time_forever,(tos_opt_all | tos_opt_clr));

        if (err == K_ERR_PEND_DESTROY) {
            rust_print("RUST: rust_tos_event_pend return K_ERR_PEND_DESTROY\r\n\0".as_ptr());
            return ;
        }

        rust_print("test_task_entry\r\n\0".as_ptr());
    }

}
pub fn rust_test_tos_event_post_keep(){
    unsafe{
        let mut test_event_00 : k_event_t = k_event_t::default();
        let mut err = rust_tos_event_create(&mut test_event_00 as *mut _, 0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_event_pend_all".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_event_post_keep_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_event_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }


        rust_print("RUST: rust_tos_event_post event_expect_dummy\r\n\0".as_ptr());
        rust_tos_event_post(&mut test_event_00 as *mut _, event_expect_dummy);
        rust_tos_sleep_ms(1000);


        rust_print("RUST: rust_tos_event_post_keep event_expect_00\r\n\0".as_ptr());
        rust_tos_event_post_keep(&mut test_event_00 as *mut _, event_expect_00);
        rust_tos_sleep_ms(1000);

        rust_print("RUST: rust_tos_event_post_keep event_expect_01\r\n\0".as_ptr());
        rust_tos_event_post_keep(&mut test_event_00 as *mut _, event_expect_01);
        rust_tos_sleep_ms(1000);
        
        rust_print("RUST: rust_tos_event_post_keep event_expect_02\r\n\0".as_ptr());
        rust_tos_event_post_keep(&mut test_event_00 as *mut _, event_expect_02);
        rust_tos_sleep_ms(1000);

        rust_print("RUST: before destroy\r\n\0".as_ptr());
        err =rust_tos_event_destroy(&mut test_event_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_event_destroy failed\r\n\0".as_ptr());
            return ;
        }
        
        rust_tos_task_destroy(&mut test_task_00 as *mut _);

        rust_print("RUST: test_tos_event_post_keep pass\r\n\0".as_ptr());
    }
}

//****************************end  of tos event test************************

//****************************tos completion test********************************
//to do
//****************************end  of tos completion test************************

//****************************tos countdownlatch test********************************
//to do
//****************************end  of tos countdownlatch test************************

//****************************tos rwlock test********************************
//to do
//****************************end  of tos rwlock test************************

//****************************tos mes_q test********************************
static mut test_msg_q_pool_00 : [u8;20] = [0;20]; //20: sizeof(void *) * TEST_MESSAGE_Q_MSG_CNT
static mut test_msg_q_pool_01 : [u8;20] = [0;20];
static mut test_msg_q_pool_02 : [u8;20] = [0;20];
pub fn rust_test_tos_message_queue_create(){
    unsafe{
        let mut test_msg_q_00 : k_msg_q_t = k_msg_q_t::default();
        let mut test_msg_q_01 : k_msg_q_t = k_msg_q_t::default();
        let mut test_msg_q_02 : k_msg_q_t = k_msg_q_t::default();
        let mut err = rust_tos_msg_q_create(&mut test_msg_q_00 as *mut _, &mut test_msg_q_pool_00 as *mut _ as *mut c_void,5);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_msg_q_create(&mut test_msg_q_01 as *mut _, &mut test_msg_q_pool_01 as *mut _ as *mut c_void,5);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_msg_q_create(&mut test_msg_q_02 as *mut _, &mut test_msg_q_pool_02 as *mut _ as *mut c_void,5);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_msg_q_destroy(&mut test_msg_q_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_destroy failed\r\n\0".as_ptr());
            return ;
        }
        
        err = rust_tos_msg_q_destroy(&mut test_msg_q_01 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_msg_q_destroy(&mut test_msg_q_02 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_message_queue_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_message_queue_destroy(){
    let mut  test_msg_q_00 :  *mut *mut k_msg_q_t = &mut 0 as *mut _ as *mut *mut k_msg_q_t;
    unsafe{
        let mut err = rust_tos_msg_q_destroy(*test_msg_q_00);
        if (  err != K_ERR_OBJ_PTR_NULL ) {
            rust_print("RUST: rust_tos_msg_q_destroy  failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: test_tos_message_queue_destroy pass\r\n\0".as_ptr());
    }
}


unsafe extern "C" fn  test_message_queue_pend_task_entry(arg : *mut c_void) 
{
    
   
    let time_forever : k_tick_t = u32::MAX; 
    let mut msg_q = arg as *mut _ as *mut k_msg_q_t;
    let mut msg : [u8;20] = ['b' as c_char;20] ;
    

    while (true){
        rust_print("before pend msg_q\r\n\0".as_ptr());
        let mut err = rust_tos_msg_q_pend(msg_q, &mut msg as *mut _ as *mut  *mut c_void,time_forever);

        if (err == K_ERR_NONE) {
            rust_print_num(msg[0] as u32);
        }
        rust_print("test_task_entry\r\n\0".as_ptr());
        rust_tos_sleep_ms(1000);
    }

}
pub fn rust_test_tos_message_queue_pend(){
    unsafe{
        let mut test_msg_q_00 : k_msg_q_t = k_msg_q_t::default();
        let mut err = rust_tos_msg_q_create(&mut test_msg_q_00 as *mut _, &mut test_msg_q_pool_00 as *mut _ as *mut c_void,5);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_message_queue_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_message_queue_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_msg_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }


        let mut msg  = ['a' as c_char;20] ;
        
        rust_print_num(msg[0] as u32 );
        
        err = rust_tos_msg_q_post(&mut test_msg_q_00 as *mut _, &mut msg  as  *mut _ as *mut c_void);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_post failed\r\n\0".as_ptr());
            return ;
        }
        
        err = rust_tos_msg_q_destroy(&mut test_msg_q_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_msg_q_destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_message_queue_pend failed\r\n\0".as_ptr());
    }
}
//****************************end  of tos mes_q test************************

//****************************tos mail_q test********************************
static  mut pool:[i32;4] = [-1;4];
pub fn rust_test_tos_mail_queue_create(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();
        let pool_ptr : *mut c_void = &mut pool as *mut _ as *mut c_void;


        let mut err = rust_tos_mail_q_create(&mut test_mail_q_00 as *mut _,pool_ptr,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mail_queue_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mail_queue_destroy(){
    unsafe{
        let mut  test_mail_q_00 :  *mut *mut k_mail_q_t = &mut 0 as *mut _ as *mut *mut k_mail_q_t;
        let mut err = rust_tos_mail_q_destroy(*test_mail_q_00);
        if (  err != K_ERR_OBJ_PTR_NULL ) {
            rust_print("RUST: rust_tos_mail_q_destroy  failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mail_queue_destroy pass\r\n\0".as_ptr());
    }
}


        
unsafe extern "C" fn  test_mail_queue_pend_task_entry(arg : *mut c_void) 
{
    let mut mail_q = arg as *mut _ as *mut k_mail_q_t;
    let mut test_pend : i32 = 1;
    let time_forever : *mut k_tick_t = (u32::MAX) as *mut k_tick_t; 

    while (true){
        rust_print("before pend mail_q\r\n\0".as_ptr());
        let mut err = rust_tos_mail_q_pend(mail_q,&mut test_pend as *mut _ as *mut c_void,4 as *mut u32,time_forever);

        if (err == K_ERR_NONE) {
            rust_print_num(test_pend as u32);
        }
        rust_print("test_task_entry\r\n\0".as_ptr());
        rust_tos_sleep_ms(1000);
    }

}
pub fn rust_test_tos_mail_queue_pend(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();

        let mut err = rust_tos_mail_q_create(&mut test_mail_q_00 as *mut _,&mut pool as *mut _ as *mut c_void,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mail_queue_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mail_queue_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_post : i32 = 2;
        rust_print("RUST: before post \r\n\0".as_ptr());

        rust_tos_mail_q_post(&mut test_mail_q_00 as *mut _,  &mut test_post as *mut _ as *mut c_void, 4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_post failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        rust_print("RUST: rust_test_tos_mail_queue_pend pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mail_queue_pend_dyn(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();
        // let pool_ptr : *mut c_void = &mut pool as *mut _ as *mut c_void;


        let mut err = rust_tos_mail_q_create_dyn(&mut test_mail_q_00 as *mut _,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create_dyn failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mail_queue_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mail_queue_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_post : i32 = 2;
        rust_print("RUST: before post \r\n\0".as_ptr());

        err = rust_tos_mail_q_post(&mut test_mail_q_00 as *mut _,  &mut test_post as *mut _ as *mut c_void, 4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_post failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        rust_print("RUST: rust_test_tos_mail_queue_pend_dyn pass\r\n\0".as_ptr());
    }
}


unsafe extern "C" fn  test_mail_queue_pend_timed_task_entry(arg : *mut c_void) 
{
    let mut mail_q = arg as *mut _ as *mut k_mail_q_t;
    let mut test_pend : i32 = 1;
    let mut time_forever :*mut  k_tick_t = 200 as *mut  k_tick_t; 
    while (true){
        rust_print("before pend mail_q\r\n\0".as_ptr());
        let mut err = rust_tos_mail_q_pend(mail_q,&mut test_pend as *mut _ as *mut c_void,4 as *mut u32,time_forever);
        
        if (err == K_ERR_NONE) {
            rust_print_num(test_pend as u32);
        }
        if(err == K_ERR_PEND_TIMEOUT){
            rust_print("pend mail_q: K_ERR_PEND_TIMEOUT\r\n\0".as_ptr());
            return ;
        }

    }

}
pub fn rust_test_tos_mail_queue_pend_timed(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();

        let mut err = rust_tos_mail_q_create(&mut test_mail_q_00 as *mut _,&mut pool as *mut _ as *mut c_void,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mail_queue_pend_timed_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mail_queue_pend_timed_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_post : i32 = 2;
        rust_tos_sleep_ms(1000);


        err = rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        rust_print("RUST: rust_test_tos_mail_queue_pend_timed pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mail_queue_post_all(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();

        let mut err = rust_tos_mail_q_create(&mut test_mail_q_00 as *mut _,&mut pool as *mut _ as *mut c_void,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_task_00 = k_task_t::default();
        let mut test_task_01 = k_task_t::default();
        let mut test_task_02 = k_task_t::default();
        let  task_name =   "test_mail_queue_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mail_queue_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_01 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_01[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_02 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            2 as k_prio_t , 
            &mut test_task_stack_01[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_post : i32 = 2;
        rust_print("RUST: before post \r\n\0".as_ptr());
        err = rust_tos_mail_q_post_all(&mut test_mail_q_00 as *mut _,&mut test_post as *mut _ as *mut c_void, 4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_post_all failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_destroy failed\r\n\0".as_ptr());
            return ;
        }
        err = rust_tos_task_destroy(&mut test_task_01 as *mut _);
        err = rust_tos_task_destroy(&mut test_task_02 as *mut _);

        err =rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_destroy failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_mail_queue_post_all pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_mail_queue_flush(){
    unsafe{
        let  mut test_mail_q_00 = k_mail_q_t::default();

        let mut err = rust_tos_mail_q_create(&mut test_mail_q_00 as *mut _,&mut pool as *mut _ as *mut c_void,3,4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_create failed\r\n\0".as_ptr());
            return ;
        }


        let mut test_task_00 = k_task_t::default();
        let  task_name =   "test_mail_queue_pend_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_mail_queue_pend_task_entry);
        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            &mut test_mail_q_00 as *mut _ as *mut c_void, 
            4 as k_prio_t , 
            &mut test_task_stack_00[0], 
            512, 
            0);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_task_create failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_post : i32 = 2;

        err = rust_tos_mail_q_post(&mut test_mail_q_00 as *mut _,  &mut test_post as *mut _ as *mut c_void, 4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_post failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_mail_q_flush(&mut test_mail_q_00 as *mut _);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_flush failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: before post we flush it \r\n\0".as_ptr());

        rust_tos_sleep_ms(1000);

        rust_print("RUST: we post after we flush it \r\n\0".as_ptr());

        err = rust_tos_mail_q_post(&mut test_mail_q_00 as *mut _,  &mut test_post as *mut _ as *mut c_void, 4);
        if(err != K_ERR_NONE ){
            rust_print("RUST: rust_tos_mail_q_post failed\r\n\0".as_ptr());
            return ;
        }

        rust_tos_sleep_ms(1000);

        err = rust_tos_mail_q_destroy(&mut test_mail_q_00 as *mut _);
        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        rust_print("RUST: rust_test_tos_mail_queue_pend_timed pass\r\n\0".as_ptr());
    }
}
//****************************end  of tos mail_q test************************


//**************************** tos_prio_msg_q test********************************
//to do
//****************************end  of  tos_prio_msg_q test************************

//**************************** tos_prio_mail_q test********************************
//to do
//****************************end  of  tos_prio_mail_q test************************

//**************************** tos ring_q test********************************
//to do
//****************************end  of  ring_q test************************

//**************************** tos bin_heap test********************************
//to do
//****************************end  of  bin_heap test************************

//**************************** tos prio_q test********************************
//to do
//****************************end  of  prio_q test************************

//**************************** tos timer test********************************
unsafe extern "C" fn  test_timer_call_back_dummy(arg : *mut c_void)
{

}

unsafe extern "C" fn  test_timer_oneshot_call_back(arg : *mut c_void)
{
    rust_print("RUST: test_timer_oneshot_call_back \r\n\0".as_ptr());
}


unsafe extern "C" fn  test_timer_periodic_call_back(arg : *mut c_void)
{
    rust_print("RUST: test_timer_periodic_call_back \r\n\0".as_ptr());
    // rust_tos_task_yield();
}

pub fn rust_test_tos_timer_create(){
    unsafe{
        let mut test_timer_00 : k_timer_t = k_timer_t::default();
        let mut test_timer_01 : k_timer_t = k_timer_t::default();
        let tos_opt_timer_oneshot : k_opt_t = 0x0001u32 as k_opt_t;

        let mut err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                            0 as *mut  k_tick_t,
                                            0 as *mut k_tick_t,
                                            Some(test_timer_call_back_dummy),
                                            0 as *mut c_void,
                                            tos_opt_timer_oneshot);
        
        if(err != K_ERR_TIMER_INVALID_DELAY){
            rust_print("RUST: rust_tos_timer_create 00 not return  K_ERR_TIMER_INVALID_DELAY\r\n\0".as_ptr());
            return ;
        }      
        
        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                            0 as *mut  k_tick_t,
                                            200 as *mut  k_tick_t,
                                            Some(test_timer_call_back_dummy),
                                            0 as *mut c_void,
                                            tos_opt_timer_oneshot);
        
        if(err != K_ERR_TIMER_INVALID_DELAY){
            rust_print("RUST: rust_tos_timer_create 0 200 not return  K_ERR_TIMER_INVALID_DELAY\r\n\0".as_ptr());
            return ;
        } 
        

        let tos_opt_timer_periodic : k_opt_t = 0x0002u32 as k_opt_t;
        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                            200 as *mut  k_tick_t,
                                            0 as *mut  k_tick_t,
                                            Some(test_timer_call_back_dummy),
                                            0 as *mut c_void,
                                            tos_opt_timer_periodic);
        
        if(err != K_ERR_TIMER_INVALID_PERIOD){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_TIMER_INVALID_PERIOD\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                            u32::MAX as  *mut  k_tick_t,
                                            0 as *mut  k_tick_t,
                                            Some(test_timer_call_back_dummy),
                                            0 as *mut c_void,
                                            tos_opt_timer_oneshot);
        
      
        if(err != K_ERR_TIMER_DELAY_FOREVER){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_TIMER_DELAY_FOREVER\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                            0 as *mut  k_tick_t,
                                            u32::MAX  as *mut  k_tick_t,
                                            Some(test_timer_call_back_dummy),
                                            0 as *mut c_void,
                                            tos_opt_timer_periodic);
        
        if(err != K_ERR_TIMER_PERIOD_FOREVER){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_TIMER_PERIOD_FOREVER\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                    200 as *mut  k_tick_t,
                                    0 as *mut  k_tick_t,
                                    Some(test_timer_call_back_dummy),
                                    0 as *mut c_void,
                                    tos_opt_timer_oneshot);

        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                    0 as *mut  k_tick_t,
                                    200 as *mut  k_tick_t,
                                    Some(test_timer_call_back_dummy),
                                    0 as *mut c_void,
                                    tos_opt_timer_periodic);

        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        rust_print("RUST: test_tos_timer_create pass\r\n\0".as_ptr());
    }
}


pub fn rust_test_tos_timer_destroy(){
    unsafe{
        let mut  test_timer_00 :  *mut *mut k_timer_t = &mut 0 as *mut _ as *mut *mut k_timer_t;
        let mut err = rust_tos_timer_destroy(*test_timer_00);
        if (  err != K_ERR_OBJ_PTR_NULL ) {
            rust_print("RUST: rust_tos_timer_destroy  failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_timer_destroy pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_timer_stop(){
    unsafe{
        let mut  test_timer_00_null :  *mut *mut k_timer_t = &mut 0 as *mut _ as *mut *mut k_timer_t;
        let mut err = rust_tos_timer_stop(*test_timer_00_null);
        if (  err != K_ERR_OBJ_PTR_NULL ) {
            rust_print("RUST: rust_tos_timer_stop  failed\r\n\0".as_ptr());
            return ;
        }

        let mut test_timer_00 : k_timer_t = k_timer_t::default();
        let tos_opt_timer_oneshot : k_opt_t = 0x0001u32 as k_opt_t;
        err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                    500 as *mut  k_tick_t,
                                    0 as *mut  k_tick_t,
                                    Some(test_timer_call_back_dummy),
                                    0 as *mut c_void,
                                    tos_opt_timer_oneshot);

        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_stop(&mut test_timer_00 as *mut _ );
    
        if(err != K_ERR_TIMER_STOPPED){
            rust_print("RUST: err != K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_timer_stop(&mut test_timer_00 as *mut _);
        if(err != K_ERR_TIMER_STOPPED){
            rust_print("RUST: err != K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_timer_start(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_start not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        // if(test_timer_00.state != timer_state_en::TIMER_STATE_RUNNING){
        //     rust_print("RUST: test_timer_00.state != timer_state_en::TIMER_STATE_RUNNING\r\n\0".as_ptr());
        //     return ;
        // }

        err = rust_tos_task_delay(510 as *mut k_tick_t);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_delay not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        // if(test_timer_00.state != timer_state_en::TIMER_STATE_COMPLETED){
        //     rust_print("RUST: test_timer_00.state != timer_state_en::TIMER_STATE_STOPPED\r\n\0".as_ptr());
        //     return ;
        // }

        err = rust_tos_timer_stop(&mut test_timer_00 as *mut _);
        if(err != K_ERR_TIMER_STOPPED){
            rust_print("RUST: rust_tos_timer_stop not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_timer_start(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_start not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_stop(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_stop not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_timer_destroy(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_destroy not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_tos_timer_stop pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_timer_oneshot_functional(){
    unsafe{
        let mut test_timer_00 : k_timer_t = k_timer_t::default();
        let tos_opt_timer_oneshot : k_opt_t = 0x0001u32 as k_opt_t;
        let mut err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                    500 as *mut  k_tick_t,
                                    0 as *mut  k_tick_t,
                                    Some(test_timer_oneshot_call_back),
                                    0 as *mut c_void,
                                    tos_opt_timer_oneshot);

        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_start(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_start not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_task_delay(2000 as *mut k_tick_t);
        rust_print("RUST: after 2000 ticks\r\n\0".as_ptr());
        err = rust_tos_timer_stop(&mut test_timer_00 as *mut _);
        if(err != K_ERR_TIMER_STOPPED){
            rust_print("RUST: rust_tos_timer_stop not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_timer_destroy(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_destroy not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_timer_oneshot_functional pass\r\n\0".as_ptr());

    }
}

pub fn rust_test_tos_timer_periodic_functional(){
    unsafe{
        let mut test_timer_00 : k_timer_t = k_timer_t::default();
        let tos_opt_timer_periodic : k_opt_t = 0x0002u32 as k_opt_t;
        let mut err = rust_tos_timer_create(&mut test_timer_00 as *mut _, 
                                    2000 as *mut  k_tick_t,
                                    4000 as *mut  k_tick_t,
                                    Some(test_timer_periodic_call_back),
                                    0 as *mut c_void,
                                    tos_opt_timer_periodic);

        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_create not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_timer_start(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_start not return  K_ERR_NONE\r\n\0".as_ptr());
            return ;
        } 

        err = rust_tos_task_delay(4000 as *mut k_tick_t);

        err = rust_tos_timer_destroy(&mut test_timer_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_timer_destroy not return  K_ERR_TIMER_STOPPED\r\n\0".as_ptr());
            return ;
        }

        rust_print("RUST: test_tos_timer_periodic_functional pass\r\n\0".as_ptr());

    }
}
//****************************end  of  timer test************************

//**************************** tos stopwatch test********************************
//to do
//****************************end  of  stopwatch test************************

//**************************** tos bitmap test********************************
//to do
//****************************end  of  bitmap test************************

//**************************** tos pm management test********************************
//to do
//****************************end  of  pm management test************************

//**************************** tos cmsis test********************************
//to do
//****************************end  of  cmsis test************************

//****************************tos_chr_fifo test************************
pub fn  rust_test_tos_fifo_create() {
    let mut test_fifo_00 = k_chr_fifo_t::default();
    let mut test_fifo_01 = k_chr_fifo_t::default();
    let mut test_fifo_02 = k_chr_fifo_t::default();


    let mut fifo_buffer_00 : [u8;5] = [0;5];
    let mut fifo_buffer_01 : [u8;5] = [0;5];
    let mut fifo_buffer_02 : [u8;5] = [0;5];
    let fifo_buffer_00_ptr : *mut c_void = &mut fifo_buffer_00 as *mut _ as *mut c_void;
    let fifo_buffer_01_ptr : *mut c_void = &mut fifo_buffer_01 as *mut _ as *mut c_void;
    let fifo_buffer_02_ptr : *mut c_void = &mut fifo_buffer_02 as *mut _ as *mut c_void;

    unsafe {
        let mut err = rust_tos_chr_fifo_create(&mut test_fifo_00 as *mut _,fifo_buffer_00_ptr , 5);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_00 create failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_create(&mut test_fifo_01 as *mut _,fifo_buffer_01_ptr , 5);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_01 create failed\r\n\0".as_ptr());
            return ;
        }
        
        err = rust_tos_chr_fifo_create(&mut test_fifo_02 as *mut _,fifo_buffer_02_ptr , 5);
        if (err != K_ERR_NONE){
            rust_print("RUST: fifo_02 create failed\r\n\0".as_ptr());
            return ;
        }
        
        err = rust_tos_chr_fifo_destroy(&mut test_fifo_00  as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_00 destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_destroy(&mut test_fifo_01  as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_01 destroy failed\r\n\0".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_destroy(&mut test_fifo_02 as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_02 destroy failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_fifo_create pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_fifo_destory(){
    let mut test_fifo_00 = k_chr_fifo_t::default();
    unsafe{
        let mut err = rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _);
        if (  err != K_ERR_OBJ_INVALID_ALLOC_TYPE && err != K_ERR_NONE ) {
            rust_print("RUST: rust_test_tos_fifo_destory  failed\r\n\0".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_fifo_destory pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_fifo_char_push(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();
        let mut fifo_buffer_00 : [u8;5] = [0;5];
        let fifo_buffer_00_ptr : *mut c_void = &mut fifo_buffer_00 as *mut _ as *mut c_void;
        let mut err; 
        err = rust_tos_chr_fifo_create(&mut test_fifo_00 as *mut _,fifo_buffer_00_ptr , 5);
        if(err != K_ERR_NONE){
            rust_print("RUST: tos_chr_fifo_create failed\r\n\0".as_ptr());
            return ;
        }else{
            let mut push_char : c_char = 'a' as c_char;

            let  mut data :  c_char = 'b' as c_char;
            
            let c : char = 'a';
            // push 
            for c in b'a'..=b'e'{
                push_char = c as c_char;
                err  = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char);
                if(err != K_ERR_NONE){
                    rust_print("RUST: rust_tos_chr_fifo_push failed\r\n\0".as_ptr());
                    return ;
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print("RUST: rust_tos_chr_fifo_push full failed\r\n\0".as_ptr());
                return ;
            }
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                rust_print_char(&data);
                if( err != K_ERR_NONE){
                    rust_print("RUST: rust_tos_chr_fifo_pop failed\r\n\0".as_ptr());
                    return ;
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);

            if( err != K_ERR_RING_Q_EMPTY){
                rust_print("RUST: rust_tos_chr_fifo_pop empty failed\r\n\0".as_ptr());
                return ;
            }


            err = rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _);
            if( err != K_ERR_NONE){
                rust_print("RUST: rust_tos_chr_fifo_destroy_dyn  failed\r\n\0".as_ptr());
                return ;
            }
        }


        rust_print("RUST: rust_test_tos_fifo_char_push pass\r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_fifo_char_push_dyn(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();

        let mut err; 
       
        err = rust_tos_chr_fifo_create_dyn(&mut test_fifo_00 as *mut _, 5);
        if(err != K_ERR_NONE){
           rust_print("RUST: rust_tos_chr_fifo_create_dyn failed\r\n\0 ".as_ptr());
           return ;
        }else{
            let mut push_char : c_char = 'a' as c_char;

            let  mut data :  c_char = 'b' as c_char;
            
            let c : char = 'a';
            // push 
            for c in b'a'..=b'e'{
                push_char = c as c_char;
                err  = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char);
                if(err != K_ERR_NONE){
                    rust_print("RUST: rust_tos_chr_fifo_push failed\r\n\0".as_ptr());
                    return ;
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print("RUST: rust_tos_chr_fifo_push full failed\r\n\0".as_ptr());
                return ;
            }
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                rust_print_char(&data);
                if( err != K_ERR_NONE){
                    rust_print("RUST: rust_tos_chr_fifo_pop full failed\r\n\0".as_ptr());
                    return ;
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);


            if( err != K_ERR_RING_Q_EMPTY){
                rust_print("RUST: rust_tos_chr_fifo_pop empty failed\r\n\0".as_ptr());
                return ;
            }


            err = rust_tos_chr_fifo_destroy_dyn(&mut test_fifo_00 as *mut _);
            if( err != K_ERR_NONE){
                rust_print("RUST: rust_tos_chr_fifo_destroy_dyn failed\r\n\0".as_ptr());
            }
            
        }
        rust_print("RUST: rust_test_tos_fifo_char_push_dyn pass \r\n\0".as_ptr());
    }
}

pub fn rust_test_tos_fifo_stream_push(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();
        let mut fifo_buffer_00 : [u8;5] = [0;5];
        let fifo_buffer_00_ptr : *mut c_void = &mut fifo_buffer_00 as *mut _ as *mut c_void;
        let mut err; 
        err = rust_tos_chr_fifo_create(&mut test_fifo_00 as *mut _,fifo_buffer_00_ptr , 5);
        
        if(err !=  K_ERR_NONE){
            rust_print_num(err as u32);
            rust_print("rust_tos_chr_fifo_create failed\r\n\0".as_ptr());
            return ;
        }
        
        let mut stream : [u8;5] = ['a' as c_char , 'b' as c_char, 'c' as c_char, 'd' as c_char, 'e' as c_char ] ;
        let mut stream_dummy : [u8;1] =['z' as c_char];
        let mut stream_pop: [u8;5]=['f' as c_char ;5];

        let mut count  = rust_tos_chr_fifo_push_stream(&mut test_fifo_00 as *mut _, &mut stream[0], 5);
        if(count != 5){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_push_stream &mut stream[0] failed\r\n\0".as_ptr());
            return ;
        }
        
        count = rust_tos_chr_fifo_push_stream(&mut test_fifo_00 as *mut _, &mut stream_dummy[0], 1);
        if(count != 0){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_push_stream &stream_dummy[0] failed\r\n\0".as_ptr());
            return ;
        }
        
        count = rust_tos_chr_fifo_is_full(&mut test_fifo_00 as *mut _);
        if(count != 1){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_is_full failed\r\n\0".as_ptr());
            return;
        }
        
        count = rust_tos_chr_fifo_pop_stream(&mut test_fifo_00 as *mut _, &mut stream_pop[0], 5);
        if(count != 5){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_pop_stream &mut stream_pop[0] failed\r\n\0".as_ptr());
            return ;
        }
        rust_print_char(&stream_pop[0]);
        rust_print_char(&stream_pop[1]);
        rust_print_char(&stream_pop[2]);
        rust_print_char(&stream_pop[3]);
        rust_print_char(&stream_pop[4]);
        count = rust_tos_chr_fifo_pop_stream(&mut test_fifo_00 as *mut _, &mut stream_pop[0], 1);
        if(count != 0){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_pop_stream &mut stream_pop[0] failed\r\n\0".as_ptr());
            return ;
        }

        count = rust_tos_chr_fifo_is_empty(&mut test_fifo_00 as *mut _);
        if(count != 1){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_is_empty failed\r\n\0".as_ptr());
            return ;
        }

        rust_print("rust_test_tos_fifo_stream_push pass\r\n\0".as_ptr());
    }
}

//****************************end of tos_chr_fifo test************************