#![no_std]

extern crate cortex_m;

mod bridge;

use crate::bridge::*;
use cty::*;

// #[repr(C)]
// pub struct knl_obj {
//     alloc_type : knl_obj_alloc_type,
// }

// pub struct k_mail_q {
//     knl_rustobj : knl_obj,
//     pend_rustobj: pend_obj,
//     ring_q: k_ring_q,
// }

// let list = k_list{}
// let knl_rustobj = knl_obj{ alloc_type : KNL_OBJ_ALLOC_TYPE_STATIC}
// let pend_rustobj = pend_obj{}



#[no_mangle]
pub extern "C" fn application_entry_rust() -> c_void {
    unsafe {
        rust_print(b"[+] Welcome to the RUST-WORLD in TencentOS :)".as_ptr());
        rust_oled_init();
        rust_oled_clear();
        
        let sys_tick = rust_osKernelSysTick();
        rust_print_num(sys_tick);
        let k_running = rust_osKernelRunning();
        if(k_running == 1){
            rust_oled_print(0,0,b"TencentOS RUST\0".as_ptr());
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
