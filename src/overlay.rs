extern crate glium;
use glium::winit;
use crate::overlay::glium::Surface;
use crate::overlay::winit::window::WindowAttributes;
// my source for learning glium and glutin https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md

pub fn create_overlay() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let window_builder = WindowAttributes::new()
	.with_transparent(true)
	.with_title("Russault overlay")
	.with_window_level(winit::window::WindowLevel::AlwaysOnTop)
	.with_active(false);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    // drawing
    let mut frame = display.draw();
    // all 0s including alpha to make window transparent.
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
    frame.finish().unwrap();
    // event_loop
    let _ = event_loop.run(move |event, window_target| {	    
	if let glium::winit::event::Event::WindowEvent { event: glium::winit::event::WindowEvent::RedrawRequested, .. } = event {
	    let mut frame = display.draw();
            frame.clear_color(0.0, 0.0, 0.0, 0.0);
            frame.finish().unwrap();
	}
	if let glium::winit::event::Event::WindowEvent { event: glium::winit::event::WindowEvent::CloseRequested, .. } = event {
	    window_target.exit()
	}
    });
}

/*
// disable interaction with window
glium::winit::event::WindowEvent::KeyboardInput {..} => (),
glium::winit::event::WindowEvent::MouseInput {..} => (),
glium::winit::event::WindowEvent::CursorMoved {..} => (),
*/		
