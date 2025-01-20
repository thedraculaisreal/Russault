#![allow(dead_code)]
use std::io;
use proc_mem::Process;
use std::thread;

mod entities;
mod offsets;
mod math;
mod cheats;
mod overlay;

fn main() -> io::Result<()> {
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    thread::spawn(move || {
	overlay::create_overlay();
	// message handling and render pipeline will go here.
    });
    entities::entity_list_loop(&game);
    Ok(())
}
