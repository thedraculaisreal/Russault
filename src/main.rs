use std::{io, thread, time::Duration};
use proc_mem::Process;

mod entities;
mod offsets;
mod math;
//mod cheats;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    let black = game.clone();
    let entity_list_handle = thread::spawn(move || {
        entities::entity_list_loop(&black);
    });
    // aimbot for now
    unsafe { loop {
        for player in &entities::PLAYER_LIST {
            let target_angle = math::calculate_angle(entities::LOCAL_PLAYER.pos.clone(), player.pos.clone());
            let write_result: bool = game.write_mem(entities::LOCAL_PLAYER.address + offsets::YAW, target_angle.x);
            let write_result: bool = game.write_mem(entities::LOCAL_PLAYER.address + offsets::PITCH, target_angle.y);
        }
    } }
    Ok(())
}