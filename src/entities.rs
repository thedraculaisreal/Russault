use crate::offsets;
use crate::math;

//pub const PLAYER_LIST: Vec<Player> = Vec::new();

pub struct Player {
    pub address: usize,
    pub name: String,
    pub health: i32,
    pub pos: math::Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Player {
    pub fn new(address: usize, game: &proc_mem::Process) -> Self {
        let health: i32 = game.read_mem::<i32>(address.clone() + offsets::HEALTH).expect("couldnt read health value ");
        let pos: math::Vec3 = game.read_mem::<math::Vec3>(address.clone() + offsets::POS).expect("couldnt read health value ");
        let yaw: f32 = game.read_mem::<f32>(address.clone() + offsets::YAW).expect("couldnt read yaw value ");
        let pitch: f32 = game.read_mem::<f32>(address.clone() + offsets::PITCH).expect("couldnt read pitch value ");
        let name = Self::read_name(address.clone(), game);
        Self {
            address, name , health , pos , yaw , pitch ,
        }
    }
    pub fn print_values(&self) {
        println!("{:x}, {}, {}", self.address, self.health, self.name);
        println!("{}, {}, {}, {}, {}", self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch);
    }
    fn read_name(offset: usize, game: &proc_mem::Process) -> String {
        let rsize = 8;
        let mut bytes_buffer: Vec<u8> = vec![0u8;rsize];
        // we read 8 bytes, and store them within are string.
        game.read_bytes(offset + offsets::NAME, bytes_buffer.as_mut_ptr(), rsize);
        let mut name = String::new();
        for byte in bytes_buffer {
            name.push(byte as u8 as char);
        }
        return name
    }
}   
