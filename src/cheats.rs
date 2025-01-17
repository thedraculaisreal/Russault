use crate::entities;
use crate::offsets;
use crate::math;

pub fn find_closest_target(local_player: entities::Player, player_list: Vec<entities::Player>, game: &proc_mem::Process) {
    for player in player_list {
        let target_angle = math::calculate_angle(local_player.pos.clone(), player.pos);
        // must add + 90.0 to yaw_value because game true north is 90.0
        let yaw_value = target_angle.y + 90.0; // y is the yaw in the vec3 of targ angle.// x is the pitch in the vec3 of targ angle.
        let pitch_value = target_angle.x; // x is the pitch in the vec3 of targ angle.
        game.write_mem(local_player.address + offsets::YAW, yaw_value);
        game.write_mem(local_player.address + offsets::PITCH, pitch_value);
    }
}
