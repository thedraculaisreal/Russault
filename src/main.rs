use proc_mem::Process;

mod entities;
mod offsets;
mod math;
mod cheats;
mod overlay;
#[allow(deprecated)]
fn main() {
    let game = Process::with_name("ac_client.exe")
	.expect("Failed to find game");
    loop {
	cheat_loop(&game);
    }
}

fn cheat_loop(game: &proc_mem::Process) {
    let (player_list, local_player) = entities::entity_list_loop(game);
    cheats::run_aimbot(game, &player_list, &local_player);
    let view_matrix: [f32; 16] = game.read_mem::<[f32; 16]>(offsets::VIEW_MATRIX)
	.expect("couldnt find view_matrix");
}
