use std::{thread, time::Duration};
use proc_mem::ProcMemError;

use crate::offsets;
use crate::math;

#[derive(Default , Clone)]
pub struct Player {
    pub address: usize,
    pub name: String,
    pub health: i32,
    pub pos: math::Vec3,
    pub origin: math::Vec3,
    pub view_angles: math::Vec3,
}

impl Player {
    pub fn new(address: &usize, game: &proc_mem::Process) -> Self {
        let health: i32 = game.read_mem::<i32>(address + offsets::HEALTH)
	    .expect("couldnt read health value ");
        let pos: math::Vec3 = game.read_mem::<math::Vec3>(address + offsets::POS)
	    .expect("couldnt read pos value ");
        let view_angles: math::Vec3 = game.read_mem::<math::Vec3>(address + offsets::VIEW_ANGLES)
	    .expect("couldnt read view angles");
	let origin: math::Vec3 = game.read_mem::<math::Vec3>(address + offsets::ORIGIN)
	    .expect("couldnt read origin");
        //let pitch: f32 = game.read_mem::<f32>(address.clone() + offsets::PITCH).expect("couldnt read pitch value ");
        let name = Self::read_name(address, game);
        Self {
            address: *address , name , health , pos, origin , view_angles ,
        }
    }
    pub fn print_values(&self) {
        println!("{:x}, {}, {}", self.address, self.health, self.name);
        println!("{}, {}, {}, {}, {}", self.pos.x, self.pos.y, self.pos.z, self.view_angles.x, self.view_angles.y);
    }
    fn read_name(offset: &usize, game: &proc_mem::Process) -> String {
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

pub fn entity_list_loop(game: &proc_mem::Process) -> Result<Vec<Player>, ProcMemError> {
    let mut player_list: Vec<Player> = Vec::new();
    let player_count_result = game.read_mem::<usize>(game.process_base_address + offsets::PLAYER_COUNT);
    let player_count = match player_count_result {
	Ok(player_count) => player_count,
	Err(e) => return Err(e),
    };
    if player_count <= 0 {
	// if not in game
	// we will handle this in the cheat loop, so that it keeps retrying to get the player_count, and doesnt go past
	thread::sleep(Duration::from_millis(100));
	return Err(ProcMemError::ReadMemoryError)
    }
    let entity_list_addr = game.read_mem::<usize>(offsets::ENTITY_LIST)?;
    for i in 1..=player_count {
	let player_address = game.read_mem::<usize>(entity_list_addr + (0x4 * i))?;
	let player = Player::new(&player_address, game);
	player_list.push(player);
	thread::sleep(Duration::from_millis(1));
    }
    Ok(player_list)
}

pub fn get_local_player(game: &proc_mem::Process) -> Player {
    let local_player_addr = game.read_mem::<usize>(game.process_base_address + offsets::LOCAL_PLAYER)
        .expect("failed to read local_player_addr");
    let local_player = Player::new(&local_player_addr, game);
    return local_player
}
