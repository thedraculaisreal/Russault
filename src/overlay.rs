extern crate glium;
use crate::entities::Player;
use crate::math;
use crate::overlay::glium::Surface;


// my source for learning glium and glutin https://github.com/glium/glium/blob/master/book/tuto-01-getting-started.md

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub static WINDOW_WIDTH: u32 = 1000;
pub static WINDOW_HEIGHT: u32 = 700; 

pub fn draw_to_screen(display: &glium::backend::glutin::Display<glutin::surface::WindowSurface>, view_matrix: [f32; 16], player_list: &Vec<Player> ){
    let esp_boxes = draw_esp(view_matrix, player_list);
    // no players to draw clear opengl draw buffer
    if esp_boxes.is_empty() {
	let mut target = display.draw();
	target.clear_color(0.0, 0.0, 0.0, 0.0);
	target.finish().unwrap();
	return
    }
    let vertex_buffer = glium::VertexBuffer::new(display, &esp_boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform mat4 transform;
        void main() {
        gl_Position = vec4(position, 0.0, 1.0);
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

    /*let (width, height) = display.get_framebuffer_dimensions();
    let scale_x = width as f32;
    let scale_y = height as f32;
    let transform = [
    [scale_x, 0.0, 0.0, 0.0],
    [0.0, scale_y, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0f32],
    ];
    let uniforms = uniform! {
    transform: transform
    };*/
    
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
		&Default::default()).unwrap();
    target.finish().unwrap();
}
// my implementation of bounding boxes, pretty suspect tho.
fn draw_esp(view_matrix: [f32; 16], player_list: &Vec<Player>) -> Vec<Vertex> {
    let mut esp_boxes = Vec::new();
    for player in player_list {
	let mut feet: math::Vec3 = math::world_to_screen(player.pos, view_matrix);
	let head: math::Vec3 = math::world_to_screen(player.origin, view_matrix);
	let difference = head.y - feet.y;
	if feet.x == 0.0 && feet.y == 0.0 && feet.z == 0.0 {
	    continue;
	}
	feet.z -= 0.90;
	feet.y = feet.y - feet.z;
	let x_diff = 0.38 - feet.z * 3.6;
	// top line segment
	esp_boxes.push(Vertex { position: [feet.x , feet.y + difference ] });
        esp_boxes.push(Vertex { position: [feet.x + x_diff , feet.y + difference ] });
	// right line segment
        esp_boxes.push(Vertex { position: [feet.x + x_diff , feet.y + difference ] });
        esp_boxes.push(Vertex { position: [feet.x + x_diff, feet.y ] });
	// bottom line segment
        esp_boxes.push(Vertex { position: [feet.x + x_diff , feet.y ] });
        esp_boxes.push(Vertex { position: [feet.x , feet.y ] });
	// left line segment
        esp_boxes.push(Vertex { position: [feet.x , feet.y ] }); 
        esp_boxes.push(Vertex { position: [feet.x , feet.y + difference ] });
    }
    return esp_boxes
}
/*
// disable interaction with window
glium::winit::event::WindowEvent::KeyboardInput {..} => (),
glium::winit::event::WindowEvent::MouseInput {..} => (),
glium::winit::event::WindowEvent::CursorMoved {..} => (),
*/		
