use crate::offsets;

pub struct Player {
    pub address: usize,
    pub name: String,
    pub health: i32,
}

impl Player {
    pub fn new(offset: usize, game: &proc_mem::Process) -> Self {
        let mut address = game.process_base_address + offset;
        address = game.read_mem::<usize>(address.clone()).expect("couldnt find player address");
        let health: i32 = game.read_mem::<i32>(address.clone() + offsets::HEALTH_OFFSET).expect("couldnt read health value ");
        let name = Self::read_name(address.clone(), game);
        Self {
            address,
            name,
            health,
        }
    }
    pub fn print_values(&self) {
        println!("{:x}, {}, {}", self.address, self.health, self.name);
    }
    fn read_name(offset: usize, game: &proc_mem::Process) -> String {
        let rsize = 8;
        let mut bytes_buffer: Vec<u8> = vec![0u8;rsize];
        // we read 8 bytes, and store them within are string.
        game.read_bytes(offset + offsets::NAME_OFFSET, bytes_buffer.as_mut_ptr(), rsize);
        let mut name = String::new();
        for byte in bytes_buffer {
            name.push(byte as u8 as char);
        }
        return name
    }
}   
