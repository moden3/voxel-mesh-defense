use bevy::prelude::*;
use crate::flow_field::FlowField;
use crate::voxel::CHUNK_SIZE;

/// ワーカー（自律型ドローン）コンポーネント
#[derive(Component)]
pub struct Worker {
    pub speed: f32,
}

/// ワーカーをスポーンさせるイベント
#[derive(Event)]
pub struct SpawnWorkerEvent {
    pub position: Vec3,
}

pub fn spawn_worker_system(
    trigger: On<SpawnWorkerEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3))), // Bevy 0.14+
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.5, 0.0), // ワーカーはオレンジ色で表現
            ..default()
        })),
        Transform::from_translation(trigger.position),
        Worker { speed: 5.0 },
    ));
}

pub fn worker_movement_system(
    mut query: Query<(&mut Transform, &Worker)>,
    flow_field_query: Query<&FlowField>,
    time: Res<Time>,
) {
    let flow_field = match flow_field_query.iter().next() {
        Some(f) => f,
        None => return,
    };

    for (mut transform, worker) in query.iter_mut() {
        let pos = transform.translation;
        let x = pos.x.floor() as i32;
        let y = pos.y.floor() as i32;
        let z = pos.z.floor() as i32;

        if x >= 0 && x < CHUNK_SIZE as i32 &&
           y >= 0 && y < CHUNK_SIZE as i32 &&
           z >= 0 && z < CHUNK_SIZE as i32 {
            let idx = FlowField::get_idx(x as usize, y as usize, z as usize);
            let dir = flow_field.grid[idx];
            if dir != Vec3::ZERO {
                // フローフィールドに従ってキネマティックに移動する
                transform.translation += dir * worker.speed * time.delta_secs();
            }
        } else {
            // 空間外にいる場合は重力で落下させる（簡易的なフェールセーフ）
            transform.translation.y -= worker.speed * time.delta_secs();
        }
    }
}
