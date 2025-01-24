use crate::entities;
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

pub fn find_closest_target(game: &proc_mem::Process) {
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
            for player in entities::PLAYER_LIST.clone() {
		let target_angle = math::calculate_angle(entities::LOCAL_PLAYER.pos, player.pos);
		let mut delta_angle = target_angle - entities::LOCAL_PLAYER.view_angles;
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
		delta_angle += entities::LOCAL_PLAYER.view_angles;
		game.write_mem(entities::LOCAL_PLAYER.address + offsets::VIEW_ANGLES, delta_angle);
		thread::sleep(Duration::from_millis(1));
            }
	}
    }
}

pub fn run_aimbot(game: &proc_mem::Process) {
    thread::sleep(Duration::from_millis(1000));
    loop {
	find_closest_target(game);
    }
}
