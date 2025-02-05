use crate::entities::*;
use crate::offsets;
use rustbot::*;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

static SMOOTH_VALUE: Vec3 = Vec3 {
    x: 0.01,
    y: 0.01,
    z: 0.01,
};

static mut TOGGLE: bool = false;
static mut LAST_STATE: bool = false;
static mut SMOOTHING: bool = false;

pub fn run_aimbot(game: &proc_mem::Process, player_list: &Vec<Player>, local_player: &Player ) {
    unsafe {
	let device_state = DeviceState::new();
	let keys = device_state.get_keys();
	if keys.contains(&Keycode::F) {
	    if !LAST_STATE {  
		TOGGLE = !TOGGLE;  
		LAST_STATE = true;  
	    }
	}
	else {
	    LAST_STATE = false;  
	}
	if TOGGLE {
	    let mut closest_target = Vec3::new(0.0,0.0,0.0);
	    let mut closest_angle: f32 = 999999.9;
            for player in player_list {
		let target_angle = calculate_angle(local_player.pos, player.pos);
		let mut delta_angle = target_angle - local_player.view_angles;
		if delta_angle.x > 180.0
		{
		    delta_angle.x -= 360.0;
		}
		if delta_angle.x < -180.0
		{
		    delta_angle.x += 360.0;
		}
		if !is_in_fov(&mut delta_angle.clone()) {
		    continue;
		}
		let angle_magnitude = (libm::sqrtf(delta_angle.x * delta_angle.x)) + (libm::sqrtf(delta_angle.y * delta_angle.y));
		if SMOOTHING {
		    //delta_angle.x = libm::fabsf(delta_angle.x);
		    delta_angle *= SMOOTH_VALUE;
		    delta_angle += local_player.view_angles;
		}
		if angle_magnitude < closest_angle {
		    closest_angle = angle_magnitude;
		    closest_target = target_angle;
		}
		thread::sleep(Duration::from_millis(1));
            }
	    if closest_angle == 999999.9 {
		return
	    }
	    game.write_mem(local_player.address + offsets::VIEW_ANGLES, closest_target);
	}
    }
}

static FOV: f32 = 30.0;

fn is_in_fov(delta_angle: &mut Vec3) -> bool {
    delta_angle.absf();
    if delta_angle.x > FOV || delta_angle.y > FOV {
	return false
    }
    return true
}
