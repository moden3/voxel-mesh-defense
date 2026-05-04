use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    render::render_resource::PrimitiveTopology,
};
use bevy_mesh::Indices;
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};

pub fn generate_chunk_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Chunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut chunk) in query.iter_mut() {
        if !chunk.is_dirty {
            continue;
        }

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let mut index_offset = 0;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let voxel = chunk.get_voxel(x as i32, y as i32, z as i32);
                    if voxel == VoxelType::Empty {
                        continue;
                    }

                    // ... simple meshing logic ...
                    // (To keep it brief but correct, we will add a simple cube meshing here)
                    let fx = x as f32;
                    let fy = y as f32;
                    let fz = z as f32;

                    let is_empty = |cx: i32, cy: i32, cz: i32| -> bool {
                        if cx < 0 || cx >= CHUNK_SIZE as i32 ||
                           cy < 0 || cy >= CHUNK_SIZE as i32 ||
                           cz < 0 || cz >= CHUNK_SIZE as i32 {
                            return true;
                        }
                        chunk.get_voxel(cx, cy, cz) == VoxelType::Empty
                    };

                    let add_face = |
                        positions: &mut Vec<[f32; 3]>,
                        normals: &mut Vec<[f32; 3]>,
                        uvs: &mut Vec<[f32; 2]>,
                        indices: &mut Vec<u32>,
                        idx_offset: &mut u32,
                        face_positions: &[[f32; 3]; 4],
                        normal: [f32; 3]
                    | {
                        for pos in face_positions.iter() {
                            positions.push([pos[0] + fx, pos[1] + fy, pos[2] + fz]);
                            normals.push(normal);
                            uvs.push([0.0, 0.0]); // 簡易UV
                        }
                        indices.push(*idx_offset);
                        indices.push(*idx_offset + 1);
                        indices.push(*idx_offset + 2);
                        indices.push(*idx_offset + 2);
                        indices.push(*idx_offset + 3);
                        indices.push(*idx_offset);
                        *idx_offset += 4;
                    };

                    // Top
                    if is_empty(x as i32, y as i32 + 1, z as i32) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[0.0, 1.0, 0.0], [0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 0.0]], [0.0, 1.0, 0.0]);
                    }
                    // Bottom
                    if is_empty(x as i32, y as i32 - 1, z as i32) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]], [0.0, -1.0, 0.0]);
                    }
                    // Right
                    if is_empty(x as i32 + 1, y as i32, z as i32) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0, 1.0]], [1.0, 0.0, 0.0]);
                    }
                    // Left
                    if is_empty(x as i32 - 1, y as i32, z as i32) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[0.0, 0.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]], [-1.0, 0.0, 0.0]);
                    }
                    // Front
                    if is_empty(x as i32, y as i32, z as i32 + 1) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]], [0.0, 0.0, 1.0]);
                    }
                    // Back
                    if is_empty(x as i32, y as i32, z as i32 - 1) {
                        add_face(&mut positions, &mut normals, &mut uvs, &mut indices, &mut index_offset,
                            &[[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 0.0, 0.0]], [0.0, 0.0, -1.0]);
                    }
                }
            }
        }

        if positions.is_empty() {
            chunk.is_dirty = false;
            continue;
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        commands.entity(entity).insert((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.8, 0.2), // グリーンのブロック
                ..default()
            })),
        ));

        chunk.is_dirty = false;
    }
}
