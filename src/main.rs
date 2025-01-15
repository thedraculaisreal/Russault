use std::{io, thread, time::Duration};
use proc_mem::Process;

mod entities;
mod offsets;
mod math;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    entity_list_loop(&game);
    Ok(())
}

fn entity_list_loop(game: &proc_mem::Process) {
    loop {
        let entity_list_addr = game.read_mem::<usize>(offsets::ENTITY_LIST).expect("couldnt find entity_list address");
        let local_player_addr = game.read_mem::<usize>(game.process_base_address + offsets::LOCAL_PLAYER).expect("couldnt find entity_list address");
        let player = entities::Player::new(local_player_addr, game);
        player.print_values();
        thread::sleep(Duration::from_millis(1000));
        for i in 1..4 {
            let player_address = game.read_mem::<usize>(entity_list_addr + (0x4 * i)).expect("couldnt find entity_list address");
            let player = entities::Player::new(player_address, game);
            player.print_values();
            thread::sleep(Duration::from_millis(1000));
        }
    }   
}
