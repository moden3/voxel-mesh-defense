use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};

pub fn generate_chunk_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut Chunk)>,
) {
    for (entity, mut chunk) in query.iter_mut() {
        if !chunk.is_dirty {
            continue;
        }

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        // 6面の定義: (法線ベクトル, 頂点座標4つ)
        let offsets = [
            (Vec3::new(0.0, 0.0, 1.0), [ // Front
                [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]
            ]),
            (Vec3::new(0.0, 0.0, -1.0), [ // Back
                [0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]
            ]),
            (Vec3::new(-1.0, 0.0, 0.0), [ // Left
                [-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]
            ]),
            (Vec3::new(1.0, 0.0, 0.0), [ // Right
                [0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]
            ]),
            (Vec3::new(0.0, 1.0, 0.0), [ // Top
                [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]
            ]),
            (Vec3::new(0.0, -1.0, 0.0), [ // Bottom
                [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]
            ]),
        ];

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let voxel = chunk.get_voxel(x as i32, y as i32, z as i32);
                    if voxel == VoxelType::Empty {
                        continue;
                    }

                    let pos = Vec3::new(x as f32, y as f32, z as f32);

                    // 隣接ボクセルをチェックして露出している面のみ描画（カリング最適化）
                    let dirs = [
                        (0, 0, 1), (0, 0, -1), (-1, 0, 0), (1, 0, 0), (0, 1, 0), (0, -1, 0)
                    ];

                    for (i, (dx, dy, dz)) in dirs.iter().enumerate() {
                        let neighbor = chunk.get_voxel(x as i32 + dx, y as i32 + dy, z as i32 + dz);
                        if neighbor == VoxelType::Empty {
                            let start_idx = positions.len() as u32;
                            let (normal, vertices) = &offsets[i];

                            for v in vertices {
                                positions.push([v[0] + pos.x, v[1] + pos.y, v[2] + pos.z]);
                                normals.push([normal.x, normal.y, normal.z]);
                                uvs.push([0.0, 0.0]); // 単純なUV
                            }

                            // 2つの三角形で1つの四角形面を表現
                            indices.push(start_idx);
                            indices.push(start_idx + 1);
                            indices.push(start_idx + 2);
                            indices.push(start_idx);
                            indices.push(start_idx + 2);
                            indices.push(start_idx + 3);
                        }
                    }
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.8, 0.4), // 仮の草ブロック色
            ..default()
        });

        commands.entity(entity).insert(PbrBundle {
            mesh: meshes.add(mesh),
            material,
            ..default()
        });

        chunk.is_dirty = false;
    }
}
