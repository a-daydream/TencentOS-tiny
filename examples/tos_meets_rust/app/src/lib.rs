#![no_std]

extern crate cortex_m;


mod bridge;

use crate::k_err_en::*;

use crate::bridge::*;
use cty::*;



#[no_mangle]
pub extern "C" fn application_entry_rust() -> c_void {
    unsafe {
        rust_print(b"[+] Welcome to the RUST-WORLD in TencentOS :)".as_ptr());

        rust_oled_init();
        rust_oled_clear();
        rust_oled_print(0,0,b"TencentOS RUST\0".as_ptr());

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
        
        // ************************start tos_chr_fifo_test***********************
        // rust_test_tos_fifo_create();
        // rust_test_tos_fifo_destory();
        rust_test_tos_fifo_char_push();
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

// k_stack_t test_task_stack_00[TEST_TASK_STACK_SIZE_00];
// k_stack_t test_task_stack_01[TEST_TASK_STACK_SIZE_01];
// k_stack_t test_task_stack_02[TEST_TASK_STACK_SIZE_02];

// k_task_t test_task_00;
// k_task_t test_task_01;
// k_task_t test_task_02;

unsafe extern "C" fn  test_task_entry(arg : *mut c_void)
{
    rust_print("test_task_entry\r\n".as_ptr());
}

pub fn rust_test_tos_task_create(){
    let mut test_task_00 = k_task_t::default();
    let mut task_name = "test_task".as_mut_ptr();
    let  mut entry  : k_task_entry_t = Some(test_task_entry);

    let mut err  = rust_tos_task_create(&mut test_task_00 as *mut _, task_name , entry, 
                                                                    arg: *mut ::core::ffi::c_void, 
                                                                    prio: k_prio_t, 
                                                                    stk_base: *mut k_stack_t, 
                                                                    stk_size: c_ulong, 
                                                                    timeslice: k_timeslice_t)
}
// TEST test_tos_task_create(void) 
// {
//     k_err_t err;

//     err = tos_task_create(&test_task_00, "test_task", test_task_entry,
//                             K_NULL, K_TASK_PRIO_IDLE,
//                             test_task_stack_00, sizeof(test_task_stack_00),
//                             0);
//     ASSERT_EQ(err, K_ERR_TASK_PRIO_INVALID);

//     err = tos_task_create(&test_task_00, "test_task", test_task_entry,
//                             K_NULL, K_TASK_PRIO_IDLE + 1,
//                             test_task_stack_00, sizeof(test_task_stack_00),
//                             0);
//     ASSERT_EQ(err, K_ERR_TASK_PRIO_INVALID);

//     err = tos_task_create(&test_task_00, "test_task", test_task_entry,
//                             K_NULL, TEST_TASK_PRIO_00,
//                             test_task_stack_00, sizeof(test_task_stack_00),
//                             0);
//     ASSERT_EQ(err, K_ERR_NONE);

//     err = tos_task_destroy(&test_task_00);
//     ASSERT_EQ(err, K_ERR_NONE);

//     PASS();
// }




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
