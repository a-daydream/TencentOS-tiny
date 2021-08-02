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
        
        // ************************tos_chr_fifo_test***********************
        // rust_test_tos_fifo_create();
        // rust_test_tos_fifo_destory();
        // rust_test_tos_fifo_char_push();
        // rust_test_tos_fifo_char_push_dyn();
        // rust_sleep(5000u32);
        // rust_mqtt_daemon();
    }

    loop {
        unsafe {
            rust_print(b"[+] This is a mqtt demo!".as_ptr());
            rust_sleep(5000u32);
        }
    }


}

    
pub fn  rust_test_tos_fifo_create(){
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
        if (rust_tos_chr_fifo_create(&mut test_fifo_00 as *mut _,fifo_buffer_00_ptr , 5) == K_ERR_NONE) {
            rust_print("RUST: fifo_00 create sucessful".as_ptr());
        }
        
        if (rust_tos_chr_fifo_create(&mut test_fifo_01 as *mut _,fifo_buffer_01_ptr , 5) == K_ERR_NONE) {
            rust_print(b"RUST: fifo_01 create sucessful".as_ptr());
        }
        
        if (rust_tos_chr_fifo_create(&mut test_fifo_02 as *mut _,fifo_buffer_02_ptr , 5) == K_ERR_NONE){
            rust_print(b"RUST: fifo_02 create sucessful".as_ptr());
        }
        
        if (rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _) == K_ERR_NONE) {
            rust_print("RUST: fifo_00 destroy sucessful".as_ptr());
        }

        if (rust_tos_chr_fifo_destroy(&mut test_fifo_01 as *mut _) == K_ERR_NONE) {
            rust_print("RUST: fifo_01 destroy sucessful".as_ptr());
        }

        if (rust_tos_chr_fifo_destroy(&mut test_fifo_02 as *mut _) == K_ERR_NONE) {
            rust_print("RUST: fifo_02 destroy sucessful".as_ptr());
        }
    }
}

pub fn rust_test_tos_fifo_destory(){
    let mut test_fifo_00 = k_chr_fifo_t::default();
    unsafe{
        if ( rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _) == K_ERR_NONE ) {
            rust_print("RUST: tos_fifo_destory K_ERR_NONE".as_ptr());
        }else if(rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _) ==  K_ERR_OBJ_INVALID_ALLOC_TYPE  ){
            rust_print("RUST: tos_fifo_destory K_ERR_OBJ_INVALID_ALLOC_TYPE".as_ptr());
        }else if(rust_tos_chr_fifo_destroy(&mut test_fifo_00 as *mut _) ==  K_ERR_OBJ_PTR_NULL ){
            rust_print(b"RUST: tos_fifo_destory K_ERR_OBJ_PTR_NULL".as_ptr());
        }else{
            rust_print(b"RUST: tos_fifo_destory failed".as_ptr());
        }
    }
}

pub fn rust_test_tos_fifo_char_push(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();
        let mut fifo_buffer_00 : [u8;5] = [0;5];
        let fifo_buffer_00_ptr : *mut c_void = &mut fifo_buffer_00 as *mut _ as *mut c_void;
        let mut err; 
        err = rust_tos_chr_fifo_create(&mut test_fifo_00 as *mut _,fifo_buffer_00_ptr , 5);
        rust_print_num(err as u32); 
        if( err == K_ERR_NONE){
            rust_print("RUST: tos_chr_fifo_create successful".as_ptr());
            let mut push_char : c_char = 'a' as c_char;

            let  mut data :  c_char = 'b' as c_char;
            
            let c : char = 'a';
            // push 
            for c in b'a'..=b'e'{
                push_char = c as c_char;
                err  = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char);
                if(err != K_ERR_NONE){
                    rust_print(" tos_chr_fifo_push failed".as_ptr());
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print(" K_ERR_RING_Q_FULL failed".as_ptr());
            }
            rust_print_num(err as u32);
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                if( err == K_ERR_NONE){
                    rust_print_char(&data);
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
            rust_print_num(err as u32);


            if( err == K_ERR_RING_Q_EMPTY){
                rust_print(" K_ERR_RING_Q_EMPTY".as_ptr());
            }


            err = rust_tos_chr_fifo_destroy_dyn(&mut test_fifo_00 as *mut _);
            rust_print_num(err as u32); 
            if( err != K_ERR_NONE){
                rust_print("rust_test_tos_fifo_char_push pass failed".as_ptr());
            }
            
        }else{
            rust_print("RUST: tos_chr_fifo_create failed".as_ptr());
        }

        rust_print("rust_test_tos_fifo_char_push pass ".as_ptr());
    }
}

pub fn rust_test_tos_fifo_char_push_dyn(){
    unsafe{
        let mut test_fifo_00 = k_chr_fifo_t::default();

        let mut err; 
       
        err = rust_tos_chr_fifo_create_dyn(&mut test_fifo_00 as *mut _, 5);
        rust_print_num(err as u32); 
        if( err == K_ERR_NONE){
            rust_print("RUST: tos_chr_fifo_create successful".as_ptr());
            let mut push_char : c_char = 'a' as c_char;

            let  mut data :  c_char = 'b' as c_char;
            
            let c : char = 'a';
            // push 
            for c in b'a'..=b'e'{
                push_char = c as c_char;
                err  = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char);
                if(err != K_ERR_NONE){
                    rust_print(" tos_chr_fifo_push failed".as_ptr());
                }
            }

            push_char ='z' as c_char;

            err = rust_tos_chr_fifo_push(&mut test_fifo_00 as *mut _, push_char); 
            if( err != K_ERR_RING_Q_FULL){
                rust_print(" K_ERR_RING_Q_FULL failed".as_ptr());
            }
            rust_print_num(err as u32);
 
            // pop
            for n in 1..6 {
                err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
                if( err == K_ERR_NONE){
                    rust_print_char(&data);
                }
            }
            err = rust_tos_chr_fifo_pop(&mut test_fifo_00 as *mut _, &mut data);
            rust_print_num(err as u32);


            if( err == K_ERR_RING_Q_EMPTY){
                rust_print(" K_ERR_RING_Q_EMPTY".as_ptr());
            }


            err = rust_tos_chr_fifo_destroy_dyn(&mut test_fifo_00 as *mut _);
            rust_print_num(err as u32); 
            if( err != K_ERR_NONE){
                rust_print("rust_test_tos_fifo_char_push pass failed".as_ptr());
            }
            
        }
        rust_print("rust_test_tos_fifo_char_push pass ".as_ptr());
    }
}