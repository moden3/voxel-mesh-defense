use bevy::prelude::*;
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};
use crate::camera::FlyCamera;
use crate::swarm::SpawnWorkerEvent;

/// プレイヤーがマーカーを配置した際に発火するイベント
#[derive(Event)]
pub struct MarkerPlacedEvent {
    pub position: Vec3,
}

pub fn mouse_picking_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<FlyCamera>>,
    mut chunk_query: Query<&mut Chunk>,
    mut commands: Commands,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = match windows.iter().next() {
        Some(w) => w,
        None => return,
    };

    let cursor_pos = match window.cursor_position() {
        Some(pos) => pos,
        None => return,
    };

    let (camera, camera_transform) = match camera_query.iter().next() {
        Some(c) => c,
        None => return,
    };

    let ray = match camera.viewport_to_world(camera_transform, cursor_pos) {
        Ok(ray) => ray,
        Err(_) => return,
    };

    let mut chunk = match chunk_query.iter_mut().next() {
        Some(c) => c,
        None => return,
    };

    // ボクセル特化の高速なDDA（Digital Differential Analyzer）アルゴリズム
    // 汎用の物理Raycastを用いず、3Dグリッド上をステップ実行して交差判定を行う
    let start = ray.origin;
    let dir = ray.direction.normalize();

    let mut t = 0.0;
    let max_dist = 50.0; // 最大探索距離

    let mut current_voxel = Vec3::new(start.x.floor(), start.y.floor(), start.z.floor());
    let step_x = dir.x.signum();
    let step_y = dir.y.signum();
    let step_z = dir.z.signum();

    let t_delta_x = if dir.x != 0.0 { (1.0_f32 / dir.x).abs() } else { f32::MAX };
    let t_delta_y = if dir.y != 0.0 { (1.0_f32 / dir.y).abs() } else { f32::MAX };
    let t_delta_z = if dir.z != 0.0 { (1.0_f32 / dir.z).abs() } else { f32::MAX };

    let mut t_max_x = if dir.x > 0.0 { (current_voxel.x + 1.0 - start.x) * t_delta_x } else { (start.x - current_voxel.x) * t_delta_x };
    let mut t_max_y = if dir.y > 0.0 { (current_voxel.y + 1.0 - start.y) * t_delta_y } else { (start.y - current_voxel.y) * t_delta_y };
    let mut t_max_z = if dir.z > 0.0 { (current_voxel.z + 1.0 - start.z) * t_delta_z } else { (start.z - current_voxel.z) * t_delta_z };

    let mut hit = false;
    let mut previous_voxel = current_voxel;

    while t < max_dist {
        let vx = current_voxel.x as i32;
        let vy = current_voxel.y as i32;
        let vz = current_voxel.z as i32;

        if vx >= 0 && vx < CHUNK_SIZE as i32 &&
           vy >= 0 && vy < CHUNK_SIZE as i32 &&
           vz >= 0 && vz < CHUNK_SIZE as i32 {
            if chunk.get_voxel(vx, vy, vz) != VoxelType::Empty {
                hit = true;
                break;
            }
        }

        previous_voxel = current_voxel;

        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                current_voxel.x += step_x;
                t = t_max_x;
                t_max_x += t_delta_x;
            } else {
                current_voxel.z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        } else {
            if t_max_y < t_max_z {
                current_voxel.y += step_y;
                t = t_max_y;
                t_max_y += t_delta_y;
            } else {
                current_voxel.z += step_z;
                t = t_max_z;
                t_max_z += t_delta_z;
            }
        }
    }

    if hit {
        // ヒットしたボクセル（壁）を破壊する
        let vx = current_voxel.x as usize;
        let vy = current_voxel.y as usize;
        let vz = current_voxel.z as usize;
        chunk.set_voxel(vx, vy, vz, VoxelType::Empty);
        
        let marker_pos = previous_voxel + Vec3::splat(0.5);
        
        // フローフィールドの再計算をトリガー
        commands.trigger(MarkerPlacedEvent {
            position: marker_pos,
        });

        // テスト用に上空からワーカーをスポーン
        commands.trigger(SpawnWorkerEvent {
            position: Vec3::new(16.0, CHUNK_SIZE as f32 + 5.0, 16.0),
        });
    }
}
