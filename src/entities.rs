use std::{thread, time::Duration};

use crate::offsets;
use crate::math;
use crate::cheats;

// not gonna use global vars anymore they are cancer.

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
            address , name , health , pos , yaw , pitch ,
        }
    }
    pub fn print_values(&self) {
        println!("{:x}, {}, {}", self.address, self.health, self.name);
        println!("{}, {}, {}, {}, {}", self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch);
    }
    fn read_name(offset: usize, game: &proc_mem::Process) -> String {
        let rsize = 16;
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

pub fn entity_list_loop(game: &proc_mem::Process) {
    loop {
        let mut player_list: Vec<Player> = Vec::new();
        let player_count: usize = game.read_mem::<usize>(game.process_base_address + offsets::PLAYER_COUNT).expect("couldnt find player_count");
        let entity_list_addr = game.read_mem::<usize>(offsets::ENTITY_LIST).expect("couldnt find entity_list address");
        let local_player_addr = game.read_mem::<usize>(game.process_base_address + offsets::LOCAL_PLAYER).expect("couldnt find entity_list address");
        let local_player: Player = Player::new(local_player_addr, game);
        for i in 1..=player_count {
            let player_address = game.read_mem::<usize>(entity_list_addr + (0x4 * i)).expect("couldnt find entity_list address");
            let player = Player::new(player_address, game);
            player_list.push(player);
            thread::sleep(Duration::from_millis(1));
        }
        cheats::find_closest_target(local_player, player_list, game);
    } 
}