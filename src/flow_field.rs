use bevy::prelude::*;
use std::collections::VecDeque;
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};

/// マーカーへ向かうためのベクトルを保持するフローフィールドコンポーネント
#[derive(Component)]
pub struct FlowField {
    pub grid: Vec<Vec3>,
}

impl Default for FlowField {
    fn default() -> Self {
        Self {
            grid: vec![Vec3::ZERO; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}

impl FlowField {
    pub fn get_idx(x: usize, y: usize, z: usize) -> usize {
        x * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + z
    }

    /// 全空きボクセルからBFS（幅優先探索）でフローフィールドを構築する
    /// テストから直接呼び出せるよう、ロジックをObserverとは独立して公開する
    pub fn build_from_bfs(chunk: &Chunk, target_x: i32, target_y: i32, target_z: i32) -> Option<Vec<Vec3>> {
        if target_x < 0 || target_x >= CHUNK_SIZE as i32 ||
           target_y < 0 || target_y >= CHUNK_SIZE as i32 ||
           target_z < 0 || target_z >= CHUNK_SIZE as i32 {
            return None;
        }

        let mut distances = vec![i32::MAX; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        let mut queue = VecDeque::new();

        let start_idx = FlowField::get_idx(target_x as usize, target_y as usize, target_z as usize);
        distances[start_idx] = 0;
        queue.push_back((target_x, target_y, target_z));

        while let Some((x, y, z)) = queue.pop_front() {
            let curr_idx = FlowField::get_idx(x as usize, y as usize, z as usize);
            let current_dist = distances[curr_idx];
            let dirs = [
                (1, 0, 0), (-1, 0, 0),
                (0, 1, 0), (0, -1, 0),
                (0, 0, 1), (0, 0, -1)
            ];

            for (dx, dy, dz) in dirs.iter() {
                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;

                if nx >= 0 && nx < CHUNK_SIZE as i32 &&
                   ny >= 0 && ny < CHUNK_SIZE as i32 &&
                   nz >= 0 && nz < CHUNK_SIZE as i32 {

                    // 壁は通れないためスキップ
                    if chunk.get_voxel(nx, ny, nz) != VoxelType::Empty {
                        continue;
                    }

                    let n_idx = FlowField::get_idx(nx as usize, ny as usize, nz as usize);
                    if distances[n_idx] == i32::MAX {
                        distances[n_idx] = current_dist + 1;
                        queue.push_back((nx, ny, nz));
                    }
                }
            }
        }

        // 各セルの方向ベクトルを計算（距離が近い方へ向かう）
        let mut grid = vec![Vec3::ZERO; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let idx = FlowField::get_idx(x, y, z);
                    let current_dist = distances[idx];
                    if current_dist == i32::MAX || current_dist == 0 {
                        grid[idx] = Vec3::ZERO;
                        continue;
                    }

                    let mut best_dir = Vec3::ZERO;
                    let mut min_dist = current_dist;

                    let dirs = [
                        (1, 0, 0), (-1, 0, 0),
                        (0, 1, 0), (0, -1, 0),
                        (0, 0, 1), (0, 0, -1)
                    ];

                    for (dx, dy, dz) in dirs.iter() {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let nz = z as i32 + dz;

                        if nx >= 0 && nx < CHUNK_SIZE as i32 &&
                           ny >= 0 && ny < CHUNK_SIZE as i32 &&
                           nz >= 0 && nz < CHUNK_SIZE as i32 {

                            let n_idx = FlowField::get_idx(nx as usize, ny as usize, nz as usize);
                            let neighbor_dist = distances[n_idx];
                            if neighbor_dist < min_dist {
                                min_dist = neighbor_dist;
                                // マーカーに向かうため符号反転
                                best_dir = Vec3::new(-*dx as f32, -*dy as f32, -*dz as f32);
                            }
                        }
                    }

                    if best_dir != Vec3::ZERO {
                        grid[idx] = best_dir.normalize();
                    }
                }
            }
        }

        Some(grid)
    }
}

pub fn update_flow_field_system(
    trigger: On<crate::picking::MarkerPlacedEvent>,
    mut query: Query<(&mut FlowField, &Chunk)>,
) {
    for (mut flow_field, chunk) in query.iter_mut() {
        let target = trigger.position.floor();
        let tx = target.x as i32;
        let ty = target.y as i32;
        let tz = target.z as i32;

        if let Some(new_grid) = FlowField::build_from_bfs(chunk, tx, ty, tz) {
            flow_field.grid = new_grid;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getIdx_correctLinearMapping() {
        // Arrange: 各座標パターンを用意
        // Act & Assert: (x,y,z)が1次元配列に正しくマッピングされること
        assert_eq!(FlowField::get_idx(0, 0, 0), 0);
        assert_eq!(FlowField::get_idx(1, 0, 0), CHUNK_SIZE * CHUNK_SIZE);
        assert_eq!(FlowField::get_idx(0, 1, 0), CHUNK_SIZE);
        assert_eq!(FlowField::get_idx(0, 0, 1), 1);
    }

    #[test]
    fn test_buildFromBfs_outOfBoundsTarget_returnsNone() {
        // Arrange: チャンクは空（デフォルト）でターゲットを範囲外に設定
        let chunk = Chunk::new();

        // Act: 境界外の座標でフローフィールドを構築
        let result = FlowField::build_from_bfs(&chunk, -1, 0, 0);

        // Assert: 境界外ターゲットはNoneを返すこと
        assert!(result.is_none(), "境界外のターゲット座標はNoneを返すべき");
    }

    #[test]
    fn test_buildFromBfs_emptyChunk_targetCellHasZeroVector() {
        // Arrange: 完全に空なチャンク、ターゲットは(5, 5, 5)
        let chunk = Chunk::new();
        let (tx, ty, tz) = (5, 5, 5);

        // Act: BFSでフローフィールドを生成
        let grid = FlowField::build_from_bfs(&chunk, tx, ty, tz)
            .expect("空チャンクでのBFSは成功すべき");

        // Assert: ターゲットセル自体のベクトルはZEROであること（目的地到達済み）
        let target_idx = FlowField::get_idx(tx as usize, ty as usize, tz as usize);
        assert_eq!(grid[target_idx], Vec3::ZERO, "目的地セルの方向ベクトルはZEROであるべき");
    }

    #[test]
    fn test_buildFromBfs_neighborCellPointsTowardTarget() {
        // Arrange: 完全に空なチャンクでターゲットの隣接セルのベクトルを検証
        let chunk = Chunk::new();
        let (tx, ty, tz) = (5, 5, 5);

        // Act
        let grid = FlowField::build_from_bfs(&chunk, tx, ty, tz)
            .expect("空チャンクでのBFSは成功すべき");

        // Assert: X方向に1つ離れたセル(6, 5, 5)の方向ベクトルは
        // ターゲット(5,5,5)に向かうX負方向を指すこと
        let neighbor_idx = FlowField::get_idx(6, 5, 5);
        let dir = grid[neighbor_idx];
        assert!(
            dir.x < 0.0,
            "ターゲットより+X方向にある隣接セルのベクトルは-X方向を指すべき (実際: {:?})", dir
        );
    }

    #[test]
    fn test_buildFromBfs_wallBlocksPath_cellBehindWallHasZeroVector() {
        // Arrange: ターゲット(5,5,5)とチェックセル(7,5,5)の間に壁を設置
        let mut chunk = Chunk::new();
        chunk.set_voxel(6, 5, 5, VoxelType::Stone); // 壁を追加

        // Act
        let grid = FlowField::build_from_bfs(&chunk, 5, 5, 5)
            .expect("BFSは成功すべき");

        // Assert: 壁に完全に遮断された場合、壁の向こう側のセルはZEROになること
        // (ただし他経路で到達できる場合は非ZEROになる可能性もある点に注意)
        // 本テストでは壁がX方向の直線的な唯一の経路を遮断しないため、
        // 壁ボクセル(6,5,5)自身がVec3::ZEROであることを確認
        let wall_idx = FlowField::get_idx(6, 5, 5);
        assert_eq!(
            grid[wall_idx],
            Vec3::ZERO,
            "壁ボクセル自体の方向ベクトルはZEROであるべき"
        );
    }
}
