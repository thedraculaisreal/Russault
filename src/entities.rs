use std::{thread, time::Duration};
use proc_mem::ProcMemError;
use rustbot::*;

use crate::offsets;

pub enum MemoryError {
    NotInGame,
    AddressInvalid,
    FailedToRead,
}

#[derive(Default , Clone)]
pub struct Player {
    pub address: usize,
    pub name: String,
    pub health: i32,
    pub pos: Vec3,
    pub origin: Vec3,
    pub view_angles: Vec3,
}

impl Player {
    pub fn new(address: &usize, game: &proc_mem::Process) -> Self {
        let health: i32 = game.read_mem::<i32>(address + offsets::HEALTH)
	    .expect("couldnt read health value ");
        let pos: Vec3 = game.read_mem::<Vec3>(address + offsets::POS)
	    .expect("couldnt read pos value ");
        let view_angles: Vec3 = game.read_mem::<Vec3>(address + offsets::VIEW_ANGLES)
	    .expect("couldnt read view angles");
	let origin: Vec3 = game.read_mem::<Vec3>(address + offsets::ORIGIN)
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

pub fn entity_list_loop(game: &proc_mem::Process) -> Result<Vec<Player>, MemoryError> {
    let mut player_list: Vec<Player> = Vec::new();
    let player_count_result = game.read_mem::<usize>(game.process_base_address + offsets::PLAYER_COUNT);
    let player_count = match player_count_result {
	Ok(player_count) => player_count,
	Err(e) => match Err::<MemoryError, ProcMemError>(e) {
	    Err(ProcMemError::ReadMemoryError) => {
		println!("Failed to read value at player_count_address: 0x{:x}", (game.process_base_address + offsets::PLAYER_COUNT));
		return Err(MemoryError::FailedToRead)
	    },
	    _ => {
		println!("player_count_address invalid: 0x{:x}", (game.process_base_address + offsets::PLAYER_COUNT));
		return Err(MemoryError::AddressInvalid)
	    },
	},
    };
    if player_count <= 0 {
	// if not in game
	// we will handle this in the cheat loop, so that it keeps retrying to get the player_count, and doesnt go past
	thread::sleep(Duration::from_millis(100));
	return Err(MemoryError::NotInGame)
    }
    let entity_list_addr_result = game.read_mem::<usize>(offsets::ENTITY_LIST);
    let entity_list_addr = match entity_list_addr_result {
	Ok(entity_list_addr) => entity_list_addr,
	Err(e) => match Err::<MemoryError, ProcMemError>(e) {
	    Err(ProcMemError::ReadMemoryError) => {
		println!("Failed to read value at entity_list_address: 0x{:x}", offsets::ENTITY_LIST);
		return Err(MemoryError::FailedToRead)
	    },
	    _ => {
		println!("entity_list_address invalid: 0x{:x}", offsets::ENTITY_LIST);
		return Err(MemoryError::AddressInvalid)
	    },
	},
    };
    for i in 1..=player_count {
	let player_address_result = game.read_mem::<usize>(entity_list_addr + (0x4 * i));
	let player_address = match player_address_result {
	    Ok(player_address) => player_address,
	    Err(e) => match Err::<MemoryError, ProcMemError>(e) {
		Err(ProcMemError::ReadMemoryError) => {
		    println!("Failed to read value at player_address: 0x{:x} at index:{}", (entity_list_addr + (0x4 * i)) ,i);
		    return Err(MemoryError::FailedToRead)
		},
		_ => {
		    println!("player_address: 0x{:x} invalid at index {}", (entity_list_addr + (0x4 * i)) , i);
		    return Err(MemoryError::AddressInvalid)
		},
	    },
	};
	let player = Player::new(&player_address, game);
	// skips dead players
	if player.health <= 0 || player.health > 100 {
	    continue;
	}
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
