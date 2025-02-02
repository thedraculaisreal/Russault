#![allow(dead_code)]
#[macro_use]
extern crate glium;
extern crate glyph_brush;
use proc_mem::Process;
use winit::window::WindowAttributes;
use std::fs::read;

mod entities;
mod offsets;
mod math;
mod cheats;
mod overlay;
#[allow(deprecated)]
fn main() {
    let game = Process::with_name("ac_client.exe")
	.expect("Failed to find game");
    type Vertex = ();
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let window_builder = WindowAttributes::new()
	.with_transparent(true)
	.with_title("Russault overlay")
	.with_window_level(winit::window::WindowLevel::AlwaysOnTop)
	.with_active(false)
	.with_inner_size(winit::dpi::LogicalSize::new(overlay::WINDOW_WIDTH, overlay::WINDOW_HEIGHT))
	.with_position(winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(345.0, 150.0)))
	.with_decorations(false);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    let font = glyph_brush::ab_glyph::FontArc::try_from_slice(include_bytes!("/Users/black/projects/rust/Russault/fonts/SIXTY.TTF")).unwrap();
    let mut glyph_bulder: glyph_brush::GlyphBrush<Vertex> = glyph_brush::GlyphBrushBuilder::using_font(font).build();
    // setting cursor passthrough
    let _ = window.set_cursor_hittest(false);
    // event_loop
    let _ = event_loop.run(move |event, window_target| {
	match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
		glium::winit::event::WindowEvent::RedrawRequested => {
		    cheat_loop(&display, &game, &glyph_bulder);
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
}

fn cheat_loop(display: &glium::backend::glutin::Display<glutin::surface::WindowSurface>, game: &proc_mem::Process, glyph_brush: glyph_brush::GlyphBrush<Vertex>) {
    let (player_list, local_player) = entities::entity_list_loop(game);
    cheats::run_aimbot(game, &player_list, &local_player);
    let view_matrix: [f32; 16] = game.read_mem::<[f32; 16]>(offsets::VIEW_MATRIX)
	.expect("couldnt find view_matrix");
    overlay::draw_to_screen(display, view_matrix, &player_list, glyph_brush);
}
