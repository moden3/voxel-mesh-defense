mod camera;
mod voxel;
mod voxel_render;
mod picking;
mod flow_field;
mod swarm;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use camera::{fly_camera_system, FlyCamera};
use voxel::{Chunk, VoxelType, CHUNK_SIZE};
use voxel_render::generate_chunk_system;
use picking::mouse_picking_system;
use flow_field::{update_flow_field_system, FlowField};
use swarm::{spawn_worker_system, worker_movement_system};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // ブラウザのキャンバス要素にウィンドウをフィットさせる
                fit_canvas_to_parent: true,
                // ブラウザのデフォルトキー操作（スクロール等）を無効化し、ゲームに全入力を渡す
                prevent_default_event_handling: false,
                title: "Voxel Mesh Defense".to_string(),
                ..default()
            }),
            ..default()
        }))
        // カスタムイベントのObserver登録
        .add_observer(update_flow_field_system)
        .add_observer(spawn_worker_system)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            fly_camera_system, 
            generate_chunk_system,
            mouse_picking_system,
            worker_movement_system
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // ライトの設定
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(16.0, 30.0, 16.0),
    ));
       commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 50.0, 10.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    // プレイヤーカメラの設定
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(16.0, 30.0, 30.0).looking_at(Vec3::new(16.0, 10.0, 16.0), Dir3::Y),
        FlyCamera { speed: 10.0, sensitivity: 0.002, pitch: 0.0, yaw: 0.0 },
    ));

    // Perlin Noiseによる初期地形生成
    let perlin = Perlin::new(1);
    let mut chunk = Chunk::new();

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let nx = x as f64 * 0.1;
            let nz = z as f64 * 0.1;
            // 高さのスケールを調整
            let height = (perlin.get([nx, nz]) * 8.0 + 10.0) as usize;

            for y in 0..CHUNK_SIZE {
                if y < height {
                    if y + 3 < height {
                        chunk.set_voxel(x, y, z, VoxelType::Stone);
                    } else {
                        chunk.set_voxel(x, y, z, VoxelType::Dirt);
                    }
                }
            }
        }
    }

    commands.spawn((chunk, FlowField::default()));
}
