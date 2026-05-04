mod camera;
mod voxel;
mod voxel_render;

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use camera::{fly_camera_system, FlyCamera};
use voxel::{Chunk, VoxelChangedEvent, VoxelType, CHUNK_SIZE};
use voxel_render::generate_chunk_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // カスタムイベントの登録
        .add_event::<VoxelChangedEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (fly_camera_system, generate_chunk_system))
        .run();
}

fn setup(mut commands: Commands) {
    // ライトの設定 (太陽光と環境光の疑似)
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1000000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(
            CHUNK_SIZE as f32 / 2.0,
            CHUNK_SIZE as f32 + 10.0,
            CHUNK_SIZE as f32 / 2.0,
        ),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // プレイヤーカメラの設定
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, CHUNK_SIZE as f32, -10.0)
                .looking_at(Vec3::new(16.0, 16.0, 16.0), Vec3::Y),
            ..default()
        },
        FlyCamera::default(),
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
                    if y < height - 3 {
                        chunk.set_voxel(x, y, z, VoxelType::Stone);
                    } else {
                        chunk.set_voxel(x, y, z, VoxelType::Dirt);
                    }
                }
            }
        }
    }

    commands.spawn(chunk);
}
