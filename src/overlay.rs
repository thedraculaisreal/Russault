extern crate glium;
use crate::offsets;
use crate::entities;
use crate::math;
use glium::winit;
use crate::overlay::glium::Surface;
use crate::overlay::winit::window::WindowAttributes;
use proc_mem::Process;
use std::thread;
use std::time::Duration;
// my source for learning glium and glutin https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

static WINDOW_WIDTH: u32 = 1000;
static WINDOW_HEIGHT: u32 = 700; 

fn create_shapes(display: &glium::backend::glutin::Display<glutin::surface::WindowSurface>, view_matrix: [f32; 16]) {
    unsafe {
	let mut all_shapes = Vec::new();
	for player in entities::PLAYER_LIST.clone() {
	    let pos: math::Vec3 = math::world_to_screen(player.pos, view_matrix, 1000.0, 700.0);
	    let multiple: f32 = 1.0;
	    
	    all_shapes.push(Vertex { position: [pos.x, pos.y + 100.0 * multiple] });
            all_shapes.push(Vertex { position: [pos.x + 50.0 * multiple, pos.y + 100.0 * multiple] });

            all_shapes.push(Vertex { position: [pos.x + 50.0 * multiple, pos.y + 100.0 * multiple] });
            all_shapes.push(Vertex { position: [pos.x + 50.0 * multiple, pos.y] });

            all_shapes.push(Vertex { position: [pos.x + 50.0 * multiple, pos.y] });
            all_shapes.push(Vertex { position: [pos.x, pos.y] });

            all_shapes.push(Vertex { position: [pos.x, pos.y] }); 
            all_shapes.push(Vertex { position: [pos.x, pos.y + 100.0 * multiple] });
	}
	if all_shapes.is_empty() {
	    return
	}
	let vertex_buffer = glium::VertexBuffer::new(display, &all_shapes).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

	let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 transform;

            void main() {
            gl_Position = transform * vec4(position, 0.0, 1.0);
            }
            "#;

	let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
            "#;

	let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

	let (width, height) = display.get_framebuffer_dimensions();
	let scale_x = 2.0 / width as f32;
	let scale_y = 2.0 / height as f32;
	let transform = [
	    [scale_x, 0.0, 0.0, 0.0],
	    [0.0, scale_y, 0.0, 0.0],
	    [0.0, 0.0, 1.0, 0.0],
	    [0.0, 0.0, 0.0, 1.0f32],
	];
	let uniforms = uniform! {
	    transform: transform
	};
	let mut target = display.draw();
	target.clear_color(0.0, 0.0, 0.0, 0.0);
	target.draw(&vertex_buffer, &indices, &program, &uniforms,
		    &Default::default()).unwrap();
	target.finish().unwrap();
    }
}

#[allow(deprecated)]
pub fn create_overlay() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let window_builder = WindowAttributes::new()
	.with_transparent(true)
	.with_title("Russault overlay")
	.with_window_level(winit::window::WindowLevel::AlwaysOnTop)
	.with_active(false)
	.with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
	.with_position(winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(345.0, 150.0)))
	.with_decorations(false);
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    let _ = window.set_cursor_hittest(false);
    // event_loop
    let game = Process::with_name("ac_client.exe").expect("Failed to find game");
    let _ = event_loop.run(move |event, window_target| {
	match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
		glium::winit::event::WindowEvent::RedrawRequested => {
		    // drawing to screen.
		    let view_matrix: [f32; 16] = game.read_mem::<[f32; 16]>(offsets::VIEW_MATRIX)
			.expect("couldnt find view_matrix");
		    create_shapes(&display, view_matrix);
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

/*
// disable interaction with window
glium::winit::event::WindowEvent::KeyboardInput {..} => (),
glium::winit::event::WindowEvent::MouseInput {..} => (),
glium::winit::event::WindowEvent::CursorMoved {..} => (),
*/		
