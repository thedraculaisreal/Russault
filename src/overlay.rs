extern crate glium;
use glium::winit;
use crate::overlay::glium::Surface;
use crate::overlay::winit::window::WindowAttributes;
// my source for learning glium and glutin https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md

#[allow(deprecated)]
pub fn create_overlay() {
    let width: u32 = 1000;
    let height: u32 = 700;
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let window_builder = WindowAttributes::new()
	.with_transparent(true)
	.with_title("Russault overlay")
	.with_window_level(winit::window::WindowLevel::AlwaysOnTop)
	.with_active(false)
	.with_inner_size(winit::dpi::LogicalSize::new(width, height))
	.with_position(winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(345.0, 150.0)))
	.with_decorations(false);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    let _ = window.set_cursor_hittest(false);
    // drawing
    let mut frame = display.draw();
    // all 0s including alpha to make window transparent.
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.finish().unwrap();
    // event_loop
    let _ = event_loop.run(move |event, window_target| {
	match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
		glium::winit::event::WindowEvent::RedrawRequested => {
		    let mut frame = display.draw();
		    frame.clear_color(0.0, 0.0, 0.0, 0.0);
		    frame.finish().unwrap();
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

/*
// disable interaction with window
glium::winit::event::WindowEvent::KeyboardInput {..} => (),
glium::winit::event::WindowEvent::MouseInput {..} => (),
glium::winit::event::WindowEvent::CursorMoved {..} => (),
*/		
