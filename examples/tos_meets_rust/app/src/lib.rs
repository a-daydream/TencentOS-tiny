#![no_std]

extern crate cortex_m;


mod bridge;

use crate::k_err_en::*;

use crate::bridge::*;
use cty::*;



#[no_mangle]
pub extern "C" fn application_entry_rust() -> c_void {
    unsafe {
        rust_print(b"[+] Welcome to the RUST-WORLD in TencentOS :)\r\n".as_ptr());

        rust_oled_init();
        rust_oled_clear();
        rust_oled_print(0,0,b"TencentOS RUST\0".as_ptr());
        // rust_print(b"[+] This is a mqtt demo!".as_ptr());

        // let  mut mail_struct = k_mail_q_t::default();
        // let  mut pool:[i32;4] = [-1;4];
        // let pool_ptr : *mut c_void = &mut pool as *mut _ as *mut c_void;

        // if (rust_tos_mail_q_create(&mut mail_struct as *mut _,pool_ptr,3,4) == K_ERR_NONE){
        //     rust_oled_print(0,0,b"RUST: tos_mail create sucessful\0".as_ptr());
        // }

        // let mut event =  k_event_t::default();
        // let mut flag : k_event_flag_t = 0;
        // if(rust_tos_event_create(&mut event as *mut _,flag) == K_ERR_NONE){
        //     if(rust_tos_event_destroy(&mut event as *mut _) == K_ERR_NONE){
        //         rust_oled_print(0,0,b"rust_tos_event_destroy xxxx\0".as_ptr());
        //     }
        // }

        // let time_forever : k_tick_t = u32::MAX;
        // let time_nowait : k_tick_t = 0u32;
        // let mut sem_test = k_sem_t::default();
        // let mut sem_count : k_sem_cnt_t = 1;
        // if(rust_tos_sem_create(&mut sem_test as *mut _,sem_count) == K_ERR_NONE){
        //     rust_oled_print(0,0,b"RUST: sem_test\0".as_ptr());
        //     if(rust_tos_sem_pend(&mut sem_test as *mut _,time_forever) == K_ERR_NONE){
        //         rust_oled_print(0,0,b"RUST: sem_test get sem sucessful\0".as_ptr());
        //     }else{
        //         rust_oled_print(0,0,b"RUST: sem_test test failed\0".as_ptr());
        //     }
        // }

        // ************************start tos_task_test***************************
        // rust_test_tos_task_create();
        // rust_test_tos_task_destroy();
        // rust_test_tos_task_delay();
        // rust_test_tos_task_delay_abort();
        // rust_test_tos_task_suspend_resume();
        // rust_test_tos_task_prio_change();
        // rust_test_tos_task_yeild();
        //*************************end of tos_task_test**************************
        
        //to test
        // ************************start tos mmheap***************************
        // to do
        //*************************end of tos mmheap**************************
        
        // ************************start tos mmblk***************************
        // rust_test_tos_mmblk_pool_create();
        // rust_test_tos_mmblk_pool_destroy();
        // rust_test_tos_mmblk_alloc();
        // rust_test_tos_mmblk_free();
        //*************************end of tos mmblk**************************
        



        //end
        
        // ************************start tos_chr_fifo_test***********************
        // rust_test_tos_fifo_create();
        // rust_test_tos_fifo_destory();
        // rust_test_tos_fifo_char_push();
        // rust_test_tos_fifo_char_push_dyn();
        // rust_test_tos_fifo_stream_push();
        
        //*************************end of tos_chr_fifo_test**********************

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
static mut test_task_stack_00 : [k_stack_t;512] = [0;512];
static mut test_task_stack_01 : [k_stack_t;512] = [0;512];
static mut test_task_stack_02 : [k_stack_t;512] = [0;512];



// static mut count_delay_abort : i32 = 0;
unsafe extern "C" fn  test_task_entry(arg : *mut c_void) 
{
    // count_delay_abort = count_delay_abort + 1;
    // let delay_1 : k_time_t = u32::MAX - 1;
    // rust_tos_task_delay(rust_tos_millisec2tick(delay_1));
    rust_print("test_task_entry\r\n".as_ptr());
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
        // rust_print_num(err as u32);
        if(err != K_ERR_TASK_PRIO_INVALID){
            rust_print("RUST: rust_tos_task_create  test idle failed\r\n".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                    task_name, entry, 
                                    0 as *mut c_void /*K_NULL*/, 
                                    10 as k_prio_t /*K_TASK_PRIO_IDLE+1  10*/, 
                                    &mut test_task_stack_00[0], 
                                    512, 
                                    0);
        // rust_print_num(err as u32);                  
        if(err != K_ERR_TASK_PRIO_INVALID){
            rust_print("RUST: rust_tos_task_create out of prio failed\r\n".as_ptr());
            return ;
        }

        err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                    task_name, entry, 
                                    0 as *mut c_void /*K_NULL*/, 
                                    3 as k_prio_t /*K_TASK_PRIO_IDLE+1  10*/, 
                                    &mut test_task_stack_00[0], 
                                    512, 
                                    0);
        // rust_print_num(err as u32);                  
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy  failed\r\n".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_task_create pass\r\n".as_ptr());
    }
}

extern "C" {
    #[no_mangle]
    static  mut k_idle_task : k_task_t;
}
pub fn rust_test_tos_task_destroy(){
    unsafe{
        let mut err = rust_tos_task_destroy(&mut k_idle_task as *mut _);
        rust_print_num(err as u32);
        if(err != K_ERR_TASK_DESTROY_IDLE){
            rust_print("RUST: rust_tos_task_destroy  test idle failed\r\n".as_ptr());
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
        
        rust_print_num(err as u32);                  
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        rust_print_num(err as u32);  
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy test_task_00 failed\r\n".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_task_destroy pass\r\n".as_ptr()); 
    }
}


pub fn rust_test_tos_task_delay(){

    unsafe{
        // let mut count = 0;
        let mut begin: k_tick_t = 10;
        let mut end: k_tick_t = 10;
        let mut delay: k_tick_t = 2000;
        let mut deviation: k_tick_t = 10;
        
        begin = rust_tos_systick_get();
        let err = rust_tos_task_delay(delay);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_delay failed\r\n".as_ptr());
            return ;
        }
        end = rust_tos_systick_get();
        rust_print_num(begin);
        rust_print_num(end);
        rust_print("RUST: rust_test_tos_task_delay pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_task_delay_abort(){
    unsafe{
        let mut delay: k_tick_t = u32::MAX - 1;
        // delay_abort_count = 0;
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry);
       
        

        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
                                                task_name, entry, 
                                                delay as *mut c_void , 
                                                4 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
                                                &mut test_task_stack_00[0], 
                                                512, 
                                                0);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create  test delay abort failed\r\n".as_ptr());
            return ;
        }

        
        // if(delay_abort_count != 0){
        //     rust_print("RUST: delay_abort_count set 0 failed\r\n".as_ptr());
        //     return ;
        // }

        let delay_1 : k_time_t = 1000;
        err =rust_tos_task_delay(rust_tos_millisec2tick(delay_1));
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_delay 100 failed\r\n".as_ptr());
            return ;
        }
        

        // rust_print_num(count);
        // if(delay_abort_count != 0){
        //     rust_print("RUST: rust_tos_task_delay abort failed\r\n".as_ptr());
        //     return ;   
        // }

        err = rust_tos_task_delay_abort(&mut test_task_00 as *mut _);
        
        err =rust_tos_task_delay(rust_tos_millisec2tick(delay_1));
        // rust_print_num(err as u32);
        // if(delay_abort_count != 1){
        //     rust_print("RUST: rust_tos_task_delay abort count + 1 failed\r\n".as_ptr());
        //     return ; 
        // }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_delay_abort pass\r\n".as_ptr());

    }
}

pub fn rust_test_tos_task_suspend_resume(){
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
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_create  test delay abort failed\r\n".as_ptr());
            return ;
        }

        // rust_tos_task_delay(500);

        err = rust_tos_task_suspend(&mut test_task_00 as *mut _);
        // rust_print("RUST: rust_tos_task_suspend \r\n".as_ptr());
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_suspend failed\r\n".as_ptr());
            return ;
        }

        let delay_1 : k_time_t = 1000;
        rust_tos_task_delay(rust_tos_millisec2tick(delay_1));
        rust_print("RUST: rust_tos_task_suspend \r\n".as_ptr());

        err = rust_tos_task_resume(&mut test_task_00 as *mut _);
        // rust_print("RUST: rust_tos_task_resume \r\n".as_ptr());
        // if(err != K_ERR_NONE){
        //     rust_print("RUST: rust_tos_task_resume failed\r\n".as_ptr());
        //     return ;
        // }
        
        // rust_tos_task_delay_abort(&mut test_task_00 as *mut _);
        rust_tos_task_delay(rust_tos_millisec2tick(delay_1));

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_suspend_resume pass\r\n".as_ptr());
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
            rust_print("RUST: rust_tos_task_prio_change to idle  test failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_task_prio_change(&mut test_task_00 as *mut _,  4 as k_prio_t);
        if(test_task_00.prio != (4 as k_prio_t)){
            rust_print("RUST: rust_tos_task_prio_change  failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_task_prio_change(&mut test_task_00 as *mut _,  2 as k_prio_t);
        if(test_task_00.prio != (2 as k_prio_t)){
            rust_print("RUST: rust_tos_task_prio_change  failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_prio_change pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_task_yeild(){
    //to do
    unsafe{
        let mut test_task_00 = k_task_t::default();
       
        let  task_name =   "test_task".as_ptr() as *mut c_char ;
        let  mut entry  : k_task_entry_t = Some(test_task_entry);
    
        let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, 
            task_name, entry, 
            0 as *mut c_void , 
            3 as k_prio_t /*K_TASK_PRIO_IDLE  9*/, 
            &mut test_task_stack_00[0], 
            512, 
            0);  

        rust_print("RUST: curr task  not yeild\r\n".as_ptr());
        
        while (true) {
            rust_tos_task_yield();
        }
        
        err = rust_tos_task_destroy(&mut test_task_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_task_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_task_yeild pass\r\n".as_ptr());
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
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_01 as *mut _, &mut mmblk_pool_buffer_01 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_02 as *mut _, &mut mmblk_pool_buffer_02 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_01);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_02);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, 0 as *mut c_void, 5, 32);
        if(err != K_ERR_OBJ_PTR_NULL){
            rust_print("RUST: rust_tos_mmblk_pool_create 0 failed\r\n".as_ptr());
            return ;
        }


        // err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, ( (mmblk_pool_buffer_01[0] as u32 ) + 1) as *mut c_void, 5, 32);
        // if(err != K_ERR_MMBLK_INVALID_POOL_ADDR){
        //     rust_print("RUST: rust_tos_mmblk_pool_create invalid failed\r\n".as_ptr());
        //     return ;
        // }

        err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_01 as *mut _, &mut mmblk_pool_buffer_01 as *mut _ as *mut c_void, 5, 33);
        if(err != K_ERR_MMBLK_INVALID_BLK_SIZE){
            rust_print("RUST: rust_tos_mmblk_pool_create size failed\r\n".as_ptr());
            return ;
        }

        rust_print("RUST: rust_test_tos_mmblk_pool_create pass\r\n".as_ptr());
    }

}

pub fn rust_test_tos_mmblk_pool_destroy(){
    let mut test_mmblk_pool_00 = k_mmblk_pool_t::default();
    unsafe{
        let mut err = rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00 as *mut _);
        if (  err != K_ERR_OBJ_INVALID_ALLOC_TYPE ) {
            rust_print("RUST: rust_tos_mmblk_pool_destroy  failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_pool_destroy pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_mmblk_alloc(){
    unsafe{
        let mut test_mmblk_pool_00 = k_mmblk_pool_t::default();
        let mut err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _, &mut mmblk_pool_buffer_00 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n".as_ptr());
            return ;
        }
        let mut blk = test_mmblk_pool_00.free_list;
        let pblk = &mut blk as *mut *mut c_void;
        for i in 1..6  {
            err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, pblk);
            if(err != K_ERR_NONE){
                rust_print("RUST: rust_tos_mmblk_alloc failed\r\n".as_ptr());
                return ;
            }
            if(pblk == (0 as * mut  *mut c_void)){
                rust_print("RUST: blk ==null failed\r\n".as_ptr());
                return ;
            }
            
        }
        err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, 0 as * mut  *mut c_void);
        if(err != K_ERR_MMBLK_POOL_EMPTY){
            rust_print("RUST: rust_tos_mmblk_alloc empty\r\n".as_ptr());
            return ;
        }

        err =rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_alloc pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_mmblk_free(){
    unsafe{
        let mut test_mmblk_pool_00 = k_mmblk_pool_t::default();
        let mut err = rust_tos_mmblk_pool_create(&mut test_mmblk_pool_00 as *mut _,&mut  mmblk_pool_buffer_00 as *mut _ as *mut c_void, 5, 32);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_create failed\r\n".as_ptr());
            return ;
        }

        let mut blk = test_mmblk_pool_00.free_list;
        let pblk = &mut blk as *mut *mut c_void;
        err = rust_tos_mmblk_alloc(&mut test_mmblk_pool_00 as *mut _, pblk);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_alloc failed\r\n".as_ptr());
            return ;
        }
        if(pblk == (0 as * mut  *mut c_void)){
            rust_print("RUST: blk ==null failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_free(&mut test_mmblk_pool_00 as *mut _, blk);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_alloc failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_mmblk_free(&mut test_mmblk_pool_00 as *mut _, blk);
        if(err != K_ERR_MMBLK_POOL_FULL){
            rust_print("RUST: rust_tos_mmblk_alloc free failed\r\n".as_ptr());
            return ;
        }

        err =rust_tos_mmblk_pool_destroy(&mut test_mmblk_pool_00 as *mut _);
        if(err != K_ERR_NONE){
            rust_print("RUST: rust_tos_mmblk_pool_destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_mmblk_free pass\r\n".as_ptr());
    }
}

//***************************end of  tos mmblk test****************************

//****************************tos mutex test********************************
//to do
//****************************end  of tos mutex test************************

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
//to do
//****************************end  of tos mes_q test************************

//****************************tos mail_q test********************************
//to do
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
//to do
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
            rust_print("RUST: fifo_00 create failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_create(&mut test_fifo_01 as *mut _,fifo_buffer_01_ptr , 5);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_01 create failed\r\n".as_ptr());
            return ;
        }
        
        err = rust_tos_chr_fifo_create(&mut test_fifo_02 as *mut _,fifo_buffer_02_ptr , 5);
        if (err != K_ERR_NONE){
            rust_print("RUST: fifo_02 create failed\r\n".as_ptr());
            return ;
        }
        
        err = rust_tos_chr_fifo_destroy(&mut test_fifo_00  as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_00 destroy failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_destroy(&mut test_fifo_01  as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_01 destroy failed\r\n".as_ptr());
            return ;
        }

        err = rust_tos_chr_fifo_destroy(&mut test_fifo_02 as *mut _);
        if (err != K_ERR_NONE) {
            rust_print("RUST: fifo_02 destroy failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_fifo_create pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_fifo_destory(){
    let mut test_fifo_00 = k_chr_fifo_t::default();
    unsafe{
        let mut err = rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _);
        if (  err != K_ERR_OBJ_INVALID_ALLOC_TYPE && err != K_ERR_NONE ) {
            rust_print("RUST: rust_test_tos_fifo_destory  failed\r\n".as_ptr());
            return ;
        }
        rust_print("RUST: rust_test_tos_fifo_destory pass\r\n".as_ptr());
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
            rust_print("RUST: tos_chr_fifo_create failed\r\n".as_ptr());
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
                    rust_print_char(&push_char);
                    rust_print("RUST: rust_tos_chr_fifo_push failed\r\n".as_ptr());
                    return ;
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print("RUST: rust_tos_chr_fifo_push full failed\r\n".as_ptr());
                return ;
            }
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                if( err != K_ERR_NONE){
                    rust_print_char(&data);
                    rust_print("RUST: rust_tos_chr_fifo_pop failed\r\n".as_ptr());
                    return ;
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);

            if( err != K_ERR_RING_Q_EMPTY){
                rust_print("RUST: rust_tos_chr_fifo_pop empty failed\r\n".as_ptr());
                return ;
            }


            err = rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _);
            if( err != K_ERR_NONE){
                rust_print("RUST: rust_tos_chr_fifo_destroy_dyn  failed\r\n".as_ptr());
                return ;
            }
        }


        rust_print("RUST: rust_test_tos_fifo_char_push pass\r\n".as_ptr());
    }
}

pub fn rust_test_tos_fifo_char_push_dyn(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();

        let mut err; 
       
        err = rust_tos_chr_fifo_create_dyn(&mut test_fifo_00 as *mut _, 5);
        if(err != K_ERR_NONE){
           rust_print("RUST: rust_tos_chr_fifo_create_dyn failed\r\n ".as_ptr());
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
                    rust_print("RUST: rust_tos_chr_fifo_push failed\r\n".as_ptr());
                    return ;
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print("RUST: rust_tos_chr_fifo_push full failed\r\n".as_ptr());
                return ;
            }
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                if( err != K_ERR_NONE){
                    rust_print("RUST: rust_tos_chr_fifo_pop full failed\r\n".as_ptr());
                    return ;
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);


            if( err != K_ERR_RING_Q_EMPTY){
                rust_print("RUST: rust_tos_chr_fifo_pop empty failed\r\n".as_ptr());
                return ;
            }


            err = rust_tos_chr_fifo_destroy_dyn(&mut test_fifo_00 as *mut _);
            if( err != K_ERR_NONE){
                rust_print("RUST: rust_tos_chr_fifo_destroy_dyn failed\r\n".as_ptr());
            }
            
        }
        rust_print("RUST: rust_test_tos_fifo_char_push pass \r\n".as_ptr());
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
            rust_print("rust_tos_chr_fifo_create failed\r\n".as_ptr());
            return ;
        }
        
        let mut stream : [u8;5] = ['a' as c_char , 'b' as c_char, 'c' as c_char, 'd' as c_char, 'e' as c_char ] ;
        let mut stream_dummy : [u8;1] =['z' as c_char];
        let mut stream_pop: [u8;5]=['f' as c_char ;5];

        let mut count  = rust_tos_chr_fifo_push_stream(&mut test_fifo_00 as *mut _, &mut stream[0], 5);
        if(count != 5){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_push_stream &mut stream[0] failed\r\n".as_ptr());
            return ;
        }
        
        count = rust_tos_chr_fifo_push_stream(&mut test_fifo_00 as *mut _, &mut stream_dummy[0], 1);
        if(count != 0){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_push_stream &stream_dummy[0] failed\r\n".as_ptr());
            return ;
        }
        
        count = rust_tos_chr_fifo_is_full(&mut test_fifo_00 as *mut _);
        if(count != 1){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_is_full failed\r\n".as_ptr());
            return;
        }
        
        count = rust_tos_chr_fifo_pop_stream(&mut test_fifo_00 as *mut _, &mut stream_pop[0], 5);
        if(count != 5){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_pop_stream &mut stream_pop[0] failed\r\n".as_ptr());
            return ;
        }

        count = rust_tos_chr_fifo_pop_stream(&mut test_fifo_00 as *mut _, &mut stream_pop[0], 1);
        if(count != 0){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_pop_stream &mut stream_pop[0] failed\r\n".as_ptr());
            return ;
        }

        count = rust_tos_chr_fifo_is_empty(&mut test_fifo_00 as *mut _);
        if(count != 1){
            rust_print_i32(count);
            rust_print("rust_tos_chr_fifo_is_empty failed\r\n".as_ptr());
            return ;
        }

        rust_print("rust_test_tos_fifo_stream_push pass\r\n".as_ptr());
    }
}

//****************************end of tos_chr_fifo test************************