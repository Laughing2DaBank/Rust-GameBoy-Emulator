mod memory;
mod cpu;

use memory::MemoryBus; 
use cpu::CPU;

fn main(){
    let mut bus = MemoryBus::new();
   let mut cpu = CPU::new(); 
    let program = [
        0x3E, 0x42,
        0x06, 0x99,
        0x00,
    ];

    for(i,&byte) in program.iter().enumerate(){

        bus.write(cpu.pc + i as u16,byte);
    }
    
    for _ in 0..3{
        cpu.step(&bus);
    }


 println!("Final State -> A: 0x{:02X}, B: 0x{:02X}", cpu.a, cpu.b);
    



}
