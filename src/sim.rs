#![allow(unused)]
use crate::read_modify_write;
use core::ptr;
use modify_derive::Modify;

pub struct SIM {
    registers: *mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    system_options2: u32,
    res_1: u32,
    system_options4: u32,
    system_options5: u32,
    res_2: u32,
    system_options7: u32,
    res_3: [u32; 2],
    system_device_identification: u32,
}

#[derive(Modify)]
#[modify(width = 2, position = 26, register_type = u32)]
pub enum UartClockSource {
    Disabled,
    MCGFLLClock,
    OSCERClock,
    MCGIRClock,
}

impl SIM {
    pub const fn access() -> Self {
        SIM {
            registers: 0x4004_8004 as *mut RegisterBlock,
        }
    }
    read_modify_write!(set_uart_clock_source, system_options2, UartClockSource);
}
