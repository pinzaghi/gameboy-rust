use crate::gameboy::cpu::*;
use crate::gameboy::cpu::ppu::*;

use super::rom::*;

const MEM_SIZE: usize = 0xFFFF;

pub(super) struct MemoryBus {
    memory: [u8; MEM_SIZE],
    is_boot_rom_mapped: bool,
    bootrom: ROM,
    gpu: PPU
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        let data = [0; MEM_SIZE];

        MemoryBus { memory: data, is_boot_rom_mapped: true, gpu: ppu::PPU::new(), bootrom: ROM::dmg() }
    }

    pub(super) fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_BEGIN ..= BOOT_END => {
                self.bootrom.read_byte(address)
            },
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram(address - VRAM_BEGIN)
            },
            _ => self.memory[address as usize]
        }
    }

    pub(super) fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(address - VRAM_BEGIN, value)
            },
            addr => self.memory[addr as usize] = value
        }
    }
}

impl CPU {

    pub(super) fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc+1)
    }

    pub(super) fn read_next_word(&self) -> u16 {
        ((self.bus.read_byte(self.pc+2) as u16) << 8) | self.bus.read_byte(self.pc+1) as u16
    }

    pub(super) fn load(&mut self, load_type: LoadType) -> ProgramCounter {
        match load_type {
        LoadType::Byte(target, source) => {
            let source_value = match source {
            LoadByteSource::A => self.regs.a,
            LoadByteSource::B => self.regs.b,
            LoadByteSource::C => self.regs.c,
            LoadByteSource::D => self.regs.d,
            LoadByteSource::E => self.regs.e,
            LoadByteSource::H => self.regs.h,
            LoadByteSource::L => self.regs.l,
            LoadByteSource::D8 => self.read_next_byte(),
            LoadByteSource::HLI => self.bus.read_byte(self.regs.get_hl())
            };
            match target {
            LoadByteTarget::A => self.regs.a = source_value,
            LoadByteTarget::B => self.regs.b = source_value,
            LoadByteTarget::C => self.regs.c = source_value,
            LoadByteTarget::D => self.regs.d = source_value,
            LoadByteTarget::E => self.regs.e = source_value,
            LoadByteTarget::H => self.regs.h = source_value,
            LoadByteTarget::L => self.regs.l = source_value,
            LoadByteTarget::HLI => self.bus.write_byte(self.regs.get_hl(), source_value)
            };
            match source {
            LoadByteSource::D8  => self.pc.wrapping_add(2),
            _                   => self.pc.wrapping_add(1),
            }
        },
        LoadType::Word(target) => {
            match target {
                WordRegister::BC => {
                    self.regs.set_bc(self.read_next_word());
                },
                WordRegister::DE => {
                    self.regs.set_de(self.read_next_word());
                },
                WordRegister::HL => {
                    self.regs.set_hl(self.read_next_word());
                },
                WordRegister::SP => {
                    self.sp = self.read_next_word();
                }
            }
            self.pc.wrapping_add(3)
        },
        LoadType::IndirectFromA(target) => {
            match target {
                AFromIndirectSource::BC => {
                    let addr = self.regs.get_bc();
                    self.regs.a = self.bus.read_byte(addr);
                },
                AFromIndirectSource::DE => {
                    let addr = self.regs.get_de();
                    self.regs.a = self.bus.read_byte(addr);
                },
                AFromIndirectSource::HLInc => {
                    let addr = self.regs.get_hl();
                    self.regs.a = self.bus.read_byte(addr);
                    let new_value = self.regs.get_hl().wrapping_add(1);
                    self.regs.set_hl(new_value);
                },
                AFromIndirectSource::HLDec => {
                    let addr = self.regs.get_hl();
                    self.regs.a = self.bus.read_byte(addr);
                    let new_value = self.regs.get_hl().wrapping_sub(1);
                    self.regs.set_hl(new_value);
                }
            }
            self.pc.wrapping_add(1)
        },
        _ => { todo!("todo") }
        }
    }

    pub(super) fn push(&mut self, target: StackTarget) -> ProgramCounter {
        let value = match target {
            StackTarget::BC => self.regs.get_bc(),
            StackTarget::DE => self.regs.get_de(),
            StackTarget::HL => self.regs.get_hl(),
            StackTarget::AF => self.regs.get_af(),
        };
        self.push_value(value);
        self.pc.wrapping_add(1)
    }

    pub(super) fn pop(&mut self, target: StackTarget) -> ProgramCounter {
        let result = self.pop_value();
        match target {
            StackTarget::BC => self.regs.set_bc(result),
            StackTarget::DE => self.regs.set_de(result),
            StackTarget::HL => self.regs.set_hl(result),
            StackTarget::AF => self.regs.set_af(result),
        };
        self.pc.wrapping_add(1)
    }

    pub(super) fn push_value(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    pub(super) fn pop_value(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

}