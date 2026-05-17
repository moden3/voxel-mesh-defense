mod voxel;
mod flow_field;
mod swarm;
mod game;
mod renderer;
mod input;

use macroquad::prelude::*;
use game::GameState;

fn window_conf() -> Conf {
    Conf {
        window_title: "Voxel Mesh Defense".to_string(),
        window_width: 640,
        window_height: 680,
        // WASM時はブラウザのcanvasサイズに合わせる
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = GameState::new();

    loop {
        let delta = get_frame_time();

        // 入力処理
        input::process_input(&mut state);

        // ゲームロジック更新
        state.update(delta);

        // 2D描画
        renderer::render(&state);

        next_frame().await;
    }
}
