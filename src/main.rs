use std::{io, thread, time::Duration};
use proc_mem::Process;

mod entities;
mod offsets;
mod math;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    for i in 0..100 {
        let player = entities::Player::new(offsets::LOCAL_PLAYER, &game);
        player.print_values();
        thread::sleep(Duration::from_millis(1000));
        std::mem::drop(player);
    }
    Ok(())
}
