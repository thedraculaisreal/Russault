#![allow(dead_code)]
use std::io;
use proc_mem::Process;

mod entities;
mod offsets;
mod math;
mod cheats;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    entities::entity_list_loop(&game);
    Ok(())
}
