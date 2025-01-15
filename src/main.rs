use std::{io, thread, time::Duration};
use proc_mem::Process;

mod entities;
mod offsets;
mod math;
//mod cheats;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    let entity_list_handle = thread::spawn(move || {
        entities::entity_list_loop(&game);
    });
    unsafe { loop {
        for player in &entities::PLAYER_LIST {
            player.print_values();
            thread::sleep(Duration::from_millis(1000));
        }
    } }
    Ok(())
}