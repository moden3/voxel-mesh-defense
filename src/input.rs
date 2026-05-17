use macroquad::prelude::*;
use crate::game::GameState;
use crate::renderer::TILE_PX;
use crate::voxel::CHUNK_SIZE;

/// キーボード・マウス入力を処理してゲーム状態を更新する。
pub fn process_input(state: &mut GameState) {
    // レイヤー切替：上下矢印キー
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::RightBracket) {
        state.layer_up();
    }
    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::LeftBracket) {
        state.layer_down();
    }

    // マウスクリックでボクセルを選択（左クリック：掘削＋マーカー）
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        // タイルグリッドの左上オフセットを計算して座標変換
        let offset_x = (screen_width()  - CHUNK_SIZE as f32 * TILE_PX) / 2.0;
        let offset_z = (screen_height() - CHUNK_SIZE as f32 * TILE_PX - 60.0) / 2.0;
        let gx = ((mx - offset_x) / TILE_PX) as i32;
        let gz = ((my - offset_z) / TILE_PX) as i32;

        // グリッド内のクリックのみ処理
        if gx >= 0 && gx < CHUNK_SIZE as i32 && gz >= 0 && gz < CHUNK_SIZE as i32 {
            state.dig_voxel(gx as usize, gz as usize);
        }
    }
}
