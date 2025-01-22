extern crate glium;
use glium::winit;

pub fn create_overlay() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
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
