#![allow(dead_code)]
#[macro_use]
extern crate glium;
use std::io;
use proc_mem::Process;
use std::thread;

mod entities;
mod offsets;
mod math;
mod cheats;
mod overlay;

fn main() -> io::Result<()> {
    thread::spawn(move || {
	let game = Process::with_name("ac_client.exe").expect("Failed to find game");
	entities::entity_list_loop(&game);
    });
    thread::spawn(move || {
	let game = Process::with_name("ac_client.exe").expect("Failed to find game");
	cheats::run_aimbot(&game);
    });
    overlay::create_overlay();
    Ok(())
}
