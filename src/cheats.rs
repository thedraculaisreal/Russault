use crate::entities::*;
use crate::offsets;
use crate::math;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

static SMOOTH_VALUE: math::Vec3 = math::Vec3 {
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
            for player in player_list {
		let target_angle = math::calculate_angle(local_player.pos, player.pos);
		if SMOOTHING {
		    let mut delta_angle = target_angle - local_player.view_angles;
		    if delta_angle.x > 180.0
		    {
			delta_angle.x -= 360.0;
		    }
		    if delta_angle.x < -180.0
		    {
			delta_angle.x += 360.0;
		    }
		    //delta_angle.x = libm::fabsf(delta_angle.x);
		    delta_angle *= SMOOTH_VALUE;
		    delta_angle += local_player.view_angles;
		}
		game.write_mem(local_player.address + offsets::VIEW_ANGLES, target_angle);
		thread::sleep(Duration::from_millis(1));
            }
	}
    }
}
