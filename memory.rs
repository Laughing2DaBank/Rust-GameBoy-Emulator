pub struct MemoryBus {
    memory: [u8; 65536],
    
}
impl MemoryBus {
    pub fn new() -> Self {
        Self { 
            memory: [0;65536],
        }
    }
    pub fn read(&self , address: u16) -> u8 {

        self.memory[address as usize ] 
    }

    pub fn write(&mut self , address:u16 , value: u8 ) {

        self.memory[address as usize] = value;
    }
}
