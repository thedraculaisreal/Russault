#![allow(dead_code)]
#[macro_use]
extern crate glium;
use proc_mem::Process;
use winit::window::WindowAttributes;
use crate::glium::Surface;
use entities::MemoryError;

mod entities;
mod offsets;
mod cheats;
mod overlay;

#[allow(deprecated)]
fn main() -> Result<(), String> {
    let game = Process::with_name("ac_client.exe").unwrap();
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window_builder = WindowAttributes::new()
	.with_transparent(true)
	.with_title("Russault overlay")
	.with_window_level(winit::window::WindowLevel::AlwaysOnTop)
	.with_active(false)
	.with_inner_size(winit::dpi::LogicalSize::new(overlay::WINDOW_WIDTH, overlay::WINDOW_HEIGHT))
	.with_position(winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(345.0, 150.0)))
	.with_decorations(false);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    // setting cursor passthrough
    let _ = window.set_cursor_hittest(false);
    // event_loop
    let _ = event_loop.run(move |event, window_target| {
	match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
		glium::winit::event::WindowEvent::RedrawRequested => {
		    cheat_loop(&display, &game);
		    window.request_redraw();
		},
		glium::winit::event::WindowEvent::CloseRequested => {
		    window_target.exit()
		},
		_ => (),
	    }
	    _ => (),
	}
    });

    Ok(())
}

fn cheat_loop(display: &glium::backend::glutin::Display<glutin::surface::WindowSurface>, game: &proc_mem::Process) {
    let player_list_result = entities::entity_list_loop(game);
    let player_list = match player_list_result {
	Ok(player_list) => player_list,
	Err(e) => match e {
	    MemoryError::NotInGame => {
		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.0, 0.0);
		target.finish().unwrap();
		return
	    },
	    MemoryError::FailedToRead => {
		panic!("Failed to read memory check address in console log.")
	    },
	    MemoryError::AddressInvalid => {
		panic!("Invalid offset/address check console log")
	    },
	},
    };
    let local_player = entities::get_local_player(game);
    cheats::run_aimbot(game, &player_list, &local_player);
    let view_matrix: [f32; 16] = game.read_mem::<[f32; 16]>(offsets::VIEW_MATRIX)
	.expect("couldnt find view_matrix");
    overlay::draw_to_screen(display, view_matrix, &player_list);
}
