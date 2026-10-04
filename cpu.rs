use crate::memory::MemoryBus; 


pub struct CPU {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8, //0-3 is nothing , 4-Zero Flag,5-Subtract,6-HalfCarry,7-Carry 

}


impl CPU {

    pub fn new()-> Self {
        CPU{
            pc:0x0100,
            sp: 0xFFFE,
            a: 0x01, 
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            f: 0xB0,
        }


    }
pub fn step(&mut self, bus : &MemoryBus){

    let opcode = bus.read(self.pc);

    match opcode {
    //NOP
    0x00 => {
        println!("0x{:40X}: NOP", self.pc);
        self.pc = self.pc.wrapping_add(1);
    
    }
    //LD B, d8
    0x06 => {
        let value = bus.read(self.pc.wrapping_add(1));
        self.b = value;
        println!("0x{:04X}: LD B, 0x{:02X}",self.pc , value); 
        self.pc = self.pc.wrapping_add(2);
    }
    //LD C, d8
    0x0E => {
        let value = bus.read(self.pc.wrapping_add(1));
        self.c = value;
        println!("0x{:04X}: LD C, 0x{:02X}",self.pc , value); 
        self.pc = self.pc.wrapping_add(2);
    }
    //LD A, d8 
    0x3E => {
        let value = bus.read(self.pc.wrapping_add(1));
        self.a = value;
        println!("0x{:04X}: LD A, 0x{:02X}",self.pc , value); 
        self.pc = self.pc.wrapping_add(2);
    }
    //LD HL 
    0x21 => {
        let low = bus.read(self.pc.wrapping_add(1)) as u16; 
        let high = bus.read(self.pc.wrapping_add(2)) as u16;
        let val = (high << 8) | low;
        self.set_hl(val);
        println!("0x{:04X}: LD HL , ox{:04X}",self.pc , val); 
        self.pc = self.pc.wrapping_add(3);
    }
    //INC B (Increment register B by 1) 
    




    _ => {

    panic!("Unimplemented opcode: 0x{:02X} at address 0x{:04X}", opcode, self.pc);

    }
}
}

}


