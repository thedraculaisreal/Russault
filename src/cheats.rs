use crate::entities;
use crate::offsets;
use crate::math;

static smooth_value: f32 = 0.10;

pub fn find_closest_target(local_player: entities::Player, player_list: Vec<entities::Player>, game: &proc_mem::Process) {
    for player in player_list {
        let target_angle = math::calculate_angle(local_player.pos.clone(), player.pos);
        /*let mut delta_angle = target_angle.subtract(local_player.view_angles.clone());
        delta_angle = delta_angle.multiply_f32(smooth_value);
        delta_angle = delta_angle.add(local_player.view_angles.clone());*/ 
        game.write_mem(local_player.address + offsets::VIEW_ANGLES, target_angle);
    }
}
