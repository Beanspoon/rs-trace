#![allow(unused)]
use rs_arm_core::{default_interrupt_handler, Vector};

#[link_section = ".vector_table.interrupts"]
pub static INTERRUPTS: [Vector; 32] = [
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // FTFA
    },
    Vector {
        handler: default_interrupt_handler, // PMC
    },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // I2C0
    },
    Vector {
        handler: default_interrupt_handler, // I2C1
    },
    Vector {
        handler: default_interrupt_handler, // SPI0
    },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // UART0
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // ADC0
    },
    Vector {
        handler: default_interrupt_handler, // CMP0
    },
    Vector {
        handler: default_interrupt_handler, // TPM0
    },
    Vector {
        handler: default_interrupt_handler, // TPM1
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // MCG
    },
    Vector {
        handler: default_interrupt_handler, // LPTMP0
    },
    Vector { reserved: 0 },
    Vector {
        handler: default_interrupt_handler, // Port A pin detect
    },
    Vector {
        handler: default_interrupt_handler, // Port B pin detect
    },
];
