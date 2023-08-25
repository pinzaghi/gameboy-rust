use core::fmt;

use pretty_hex::*;

use crate::gameboy::{mmu::{Address, IO_SIZE, IO_BEGIN, IO_END}};

use super::interrupts::{Interruption, InterruptsRegister};

pub(crate) const JOYPAD_INPUT_ADDRESS: Address = 0xFF00;
pub(crate) const SERIAL_DATA_ADDRESS: Address = 0xFF01;
pub(crate) const SERIAL_CONTROL_ADDRESS: Address = 0xFF02;

pub(crate) const DIV_ADDRESS: Address = 0xFF04;
pub(crate) const TIMA_ADDRESS: Address = 0xFF05;
pub(crate) const TMA_ADDRESS: Address = 0xFF06;
pub(crate) const TAC_ADDRESS: Address = 0xFF07;

pub(crate) const LCD_CONTROL_BEGIN: Address = 0xFF40;
pub(crate) const LCD_CONTROL_END: Address = 0xFF4B;
pub(crate) const BOOT_SWITCH_ADDRESS: Address = 0xFF50;

pub(crate) const INTERRUPT_FLAG_ADDRESS: Address = 0xFF0F;
pub(crate) const INTERRUPT_ENABLE_ADDRESS: Address = 0xFFFF;

pub(crate) struct IO {
    pub(crate) interrupts: InterruptsRegister,
    data: [u8; IO_SIZE]
}

#[derive(Debug)]
pub(crate) enum IOEvent {
    BootSwitched(bool),
}

impl IO {
    pub(crate) fn new() -> Self {
        Self{ interrupts: InterruptsRegister::new(), data:[0; IO_SIZE] }
    }

    pub(crate) fn read_byte(&self, address: Address) -> u8 {
        match address {
            0xFF44 => 0x90,
            INTERRUPT_FLAG_ADDRESS => self.interrupts.read_flag(),
            // TODO: Map the rest
            _ => self.data[(address - IO_BEGIN) as usize]
        }
    }

    pub(crate) fn write_byte(&mut self, address: Address, value: u8) -> Option<IOEvent> {
        self.data[(address - IO_BEGIN) as usize] = value;
        match address {
            // ROM
            BOOT_SWITCH_ADDRESS => Some(IOEvent::BootSwitched(value == 0)),
            INTERRUPT_FLAG_ADDRESS => {
                self.interrupts.write_flag(value);
                None
            },
            _ => None
        }
    }

    pub(crate) fn serial_control_clear(&mut self) {
        // Turn off bit 7
        self.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] = self.data[(SERIAL_CONTROL_ADDRESS - IO_BEGIN) as usize] & 0b01111111;
    }

}