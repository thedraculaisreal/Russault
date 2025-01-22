use crate::entities;
use crate::offsets;
use crate::math;
use device_query::{DeviceQuery, DeviceState, Keycode};

static SMOOTH_VALUE: math::Vec3 = math::Vec3 {
    x: 0.01,
    y: 0.01,
    z: 0.01,
};
static mut TOGGLE: bool = false;
static mut LAST_STATE: bool = false;

pub fn find_closest_target(local_player: entities::Player, player_list: Vec<entities::Player>, game: &proc_mem::Process) {
    unsafe {
	let device_state = DeviceState::new();
	let keys = device_state.get_keys();
	if keys.contains(&Keycode::F) {
	    if !LAST_STATE {  
		TOGGLE = !TOGGLE;  
		LAST_STATE = true;  
	    }
	} else {
	    LAST_STATE = false;  
	}
	if TOGGLE {
            for player in player_list {
            let target_angle = math::calculate_angle(local_player.pos, player.pos);
            /*let mut delta_angle = target_angle - local_player.view_angles;
            delta_angle *= SMOOTH_VALUE;
            delta_angle += local_player.view_angles;
            println!("{},{}", delta_angle.x, delta_angle.y);*/
            game.write_mem(local_player.address + offsets::VIEW_ANGLES, target_angle);
        }
    } }
}
