use std::collections::VecDeque;
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};

/// 各ボクセルから目標地点へ向かう方向ベクトル（XZ平面）を保持するフローフィールド。
/// BFSで計算し、ワーカーはこれを参照して移動する。
pub struct FlowField {
    /// 各セルの移動方向。[dx, dz] の正規化ベクトル。
    pub grid: Vec<[f32; 2]>,
}

impl Default for FlowField {
    fn default() -> Self {
        Self {
            grid: vec![[0.0, 0.0]; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}

impl FlowField {
    pub fn get_idx(x: usize, y: usize, z: usize) -> usize {
        x * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + z
    }

    /// 目標地点からBFSでフローフィールドを構築する。
    /// 2D断面表示に合わせてXZ平面のみで経路探索を行う。
    pub fn build_from_bfs(
        chunk: &Chunk,
        tx: i32,
        ty: i32,
        tz: i32,
    ) -> Option<Vec<[f32; 2]>> {
        if tx < 0 || tx >= CHUNK_SIZE as i32
            || ty < 0 || ty >= CHUNK_SIZE as i32
            || tz < 0 || tz >= CHUNK_SIZE as i32
        {
            return None;
        }

        let mut distances = vec![i32::MAX; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        let mut queue = VecDeque::new();
        let start_idx = FlowField::get_idx(tx as usize, ty as usize, tz as usize);
        distances[start_idx] = 0;
        queue.push_back((tx, ty, tz));

        // XZ平面のみで経路探索（同レイヤー内移動）
        let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((x, y, z)) = queue.pop_front() {
            let curr_idx = FlowField::get_idx(x as usize, y as usize, z as usize);
            let current_dist = distances[curr_idx];

            for (dx, dz) in &dirs {
                let nx = x + dx;
                let nz = z + dz;
                if nx < 0 || nx >= CHUNK_SIZE as i32 || nz < 0 || nz >= CHUNK_SIZE as i32 {
                    continue;
                }
                if chunk.get_voxel(nx, y, nz) != VoxelType::Empty {
                    continue;
                }
                let n_idx = FlowField::get_idx(nx as usize, y as usize, nz as usize);
                if distances[n_idx] == i32::MAX {
                    distances[n_idx] = current_dist + 1;
                    queue.push_back((nx, y, nz));
                }
            }
        }

        let mut grid = vec![[0.0f32; 2]; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let idx = FlowField::get_idx(x, y, z);
                    let current_dist = distances[idx];
                    if current_dist == i32::MAX || current_dist == 0 {
                        continue;
                    }
                    let mut best_dir = [0.0f32; 2];
                    let mut min_dist = current_dist;
                    for (dx, dz) in &dirs {
                        let nx = x as i32 + dx;
                        let nz = z as i32 + dz;
                        if nx < 0 || nx >= CHUNK_SIZE as i32 || nz < 0 || nz >= CHUNK_SIZE as i32 {
                            continue;
                        }
                        let n_idx = FlowField::get_idx(nx as usize, y, nz as usize);
                        if distances[n_idx] < min_dist {
                            min_dist = distances[n_idx];
                            // 距離が小さい隣接セルへ向かう方向がそのまま目標方向
                            best_dir = [*dx as f32, *dz as f32];
                        }
                    }
                    let len = (best_dir[0] * best_dir[0] + best_dir[1] * best_dir[1]).sqrt();
                    if len > 0.0 {
                        grid[idx] = [best_dir[0] / len, best_dir[1] / len];
                    }
                }
            }
        }
        Some(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getIdx_correctLinearMapping() {
        assert_eq!(FlowField::get_idx(0, 0, 0), 0);
        assert_eq!(FlowField::get_idx(1, 0, 0), CHUNK_SIZE * CHUNK_SIZE);
        assert_eq!(FlowField::get_idx(0, 1, 0), CHUNK_SIZE);
        assert_eq!(FlowField::get_idx(0, 0, 1), 1);
    }

    #[test]
    fn test_buildFromBfs_outOfBoundsTarget_returnsNone() {
        let chunk = Chunk::new();
        assert!(FlowField::build_from_bfs(&chunk, -1, 0, 0).is_none());
    }

    #[test]
    fn test_buildFromBfs_emptyChunk_targetCellHasZeroVector() {
        let chunk = Chunk::new();
        let grid = FlowField::build_from_bfs(&chunk, 5, 5, 5).unwrap();
        let idx = FlowField::get_idx(5, 5, 5);
        assert_eq!(grid[idx], [0.0, 0.0]);
    }

    #[test]
    fn test_buildFromBfs_neighborCellPointsTowardTarget() {
        let chunk = Chunk::new();
        let grid = FlowField::build_from_bfs(&chunk, 5, 5, 5).unwrap();
        let neighbor_idx = FlowField::get_idx(6, 5, 5);
        let dir = grid[neighbor_idx];
        assert!(dir[0] < 0.0, "隣接セルは-X方向を指すべき: {:?}", dir);
    }
}
