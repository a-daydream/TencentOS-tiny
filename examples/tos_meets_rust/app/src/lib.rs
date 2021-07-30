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
        // rust_print_num(mail_struct.ring_q.total);
        // let  mut pool:[i32;4] = [-1;4];
        // let pool_ptr : *mut c_void = &mut pool as *mut _ as *mut c_void;

        // rust_tos_mail_q_create(&mut mail_struct as *mut _,pool_ptr,3,4);


        // let mut event =  k_event_t::default();
        // let mut flag : k_event_flag_t = 0;
        // if(rust_tos_event_create(&mut event as *mut _,flag) == K_ERR_NONE){
        //     if(rust_tos_event_destroy(&mut event as *mut _) == K_ERR_NONE){
        //         rust_oled_print(0,0,b"rust_tos_event_destroy xxxx\0".as_ptr());
        //     }
        // }
        let time_forever : k_tick_t = u32::MAX;
        let time_nowait : k_tick_t = 0u32;
        let mut sem_test = k_sem_t::default();
        let mut sem_count : k_sem_cnt_t = 1;
        if(rust_tos_sem_create(&mut sem_test as *mut _,sem_count) == K_ERR_NONE){
            rust_oled_print(0,0,b"RUST: sem_test\0".as_ptr());
            if(rust_tos_sem_pend(&mut sem_test as *mut _,time_nowait) == K_ERR_NONE){
                rust_oled_print(0,0,b"RUST: sem_test get sem sucessful\0".as_ptr());
            }else{
                rust_oled_print(0,0,b"RUST: sem_test test failed\0".as_ptr());
            }
        }


        rust_sleep(5000u32);
        rust_mqtt_daemon();
        
    }

    loop {
        unsafe {
            rust_print(b"[+] This is a mqtt demo!".as_ptr());
            rust_sleep(5000u32);
        }
    }
}
