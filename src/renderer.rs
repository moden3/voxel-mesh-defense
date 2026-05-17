use macroquad::prelude::*;
use crate::game::GameState;
use crate::voxel::{VoxelType, CHUNK_SIZE};
use crate::swarm::WorkerState;

/// 1タイルのピクセルサイズ
pub const TILE_PX: f32 = 18.0;

/// ボクセル種別ごとの描画色。
fn voxel_color(voxel: VoxelType) -> Color {
    match voxel {
        VoxelType::Empty  => Color::from_rgba(17, 17, 17, 255),
        VoxelType::Stone  => Color::from_rgba(102, 102, 102, 255),
        VoxelType::Dirt   => Color::from_rgba(139, 69, 19, 255),
        VoxelType::Pipe   => Color::from_rgba(0, 255, 255, 255),
        VoxelType::Marker => Color::from_rgba(255, 255, 0, 255),
    }
}

/// ゲーム全体を2Dタイルとして描画する。
/// 現在レイヤー（Y断面）のみを描画し、未探索は霧として表示する。
pub fn render(state: &GameState) {
    clear_background(Color::from_rgba(10, 10, 20, 255));

    let layer = state.current_layer;
    let offset_x = (screen_width()  - CHUNK_SIZE as f32 * TILE_PX) / 2.0;
    let offset_z = (screen_height() - CHUNK_SIZE as f32 * TILE_PX - 60.0) / 2.0;

    // タイルグリッドを描画
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let px = offset_x + x as f32 * TILE_PX;
            let pz = offset_z + z as f32 * TILE_PX;

            if !state.chunk.is_explored(x, layer, z) {
                // 未探索：霧（濃い紺）
                draw_rectangle(px, pz, TILE_PX - 1.0, TILE_PX - 1.0,
                    Color::from_rgba(20, 20, 46, 255));
                continue;
            }

            let voxel = state.chunk.get_voxel(x as i32, layer as i32, z as i32);
            let color = voxel_color(voxel);
            draw_rectangle(px, pz, TILE_PX - 1.0, TILE_PX - 1.0, color);
        }
    }

    // ワーカーを描画（オレンジの点）
    for worker in &state.workers {
        if worker.y == layer {
            let px = offset_x + worker.x * TILE_PX + TILE_PX / 2.0;
            let pz = offset_z + worker.z * TILE_PX + TILE_PX / 2.0;
            let color = match worker.state {
                WorkerState::Working => Color::from_rgba(0, 255, 0, 255),
                _ => Color::from_rgba(255, 140, 0, 255),
            };
            draw_circle(px, pz, TILE_PX / 3.0, color);
        }
    }

    // HUD：レイヤー情報・操作説明
    let hud_y = offset_z + CHUNK_SIZE as f32 * TILE_PX + 8.0;
    draw_text(
        &format!("Layer Y={:>2}  [↑]/[↓] で切替  クリックで掘削/マーカー  Workers:{}", layer, state.workers.len()),
        offset_x, hud_y + 20.0, 18.0, WHITE,
    );
}
