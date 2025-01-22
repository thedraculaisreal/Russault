extern crate glium;
use glium::winit;
use crate::overlay::glium::Surface;
// my source for learning glium and glutin https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md

pub fn create_overlay() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    // making window always on top and transparent.
    window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
    window.set_transparent(true);
    // drawing
    let mut frame = display.draw();
    // all 0s including alpha to make window transparent.
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.finish().unwrap();
    // event_loop
    let _ = event_loop.run(move |event, window_target| {
	match event {
	    glium::winit::event::Event::WindowEvent { event, .. } => match event {
		glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
		_ => (),
	    },
	    _ => (),
	};
    });
}
