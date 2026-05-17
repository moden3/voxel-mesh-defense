/// ボクセルの種別を定義する。
/// ゲーム内部の3Dグリッドデータとして使用し、描画は2D断面図で行う。
pub const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VoxelType {
    Empty,   // 空洞・通路
    Stone,   // 岩盤（掘削可能）
    Dirt,    // 土（掘削可能・軟弱）
    Pipe,    // エネルギーパイプ
    Marker,  // 掘削指示マーカー
}

/// 32x32x32 のボクセルデータを保持するチャンク。
/// スタックオーバーフロー防止のため Vec（ヒープ）で確保する。
pub struct Chunk {
    voxels: Vec<VoxelType>,
    /// 探索済みフラグ（falseは霧として表示される）
    pub explored: Vec<bool>,
    pub is_dirty: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: vec![VoxelType::Empty; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            explored: vec![false; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            is_dirty: true,
        }
    }

    /// (x, y, z) を1次元インデックスに変換する。
    pub fn get_idx(x: usize, y: usize, z: usize) -> usize {
        x * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + z
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: VoxelType) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            let idx = Self::get_idx(x, y, z);
            self.voxels[idx] = voxel;
            self.is_dirty = true;
        }
    }

    /// 境界外アクセスはパニックを避けて Empty を返す。
    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> VoxelType {
        if x < 0 || y < 0 || z < 0
            || x >= CHUNK_SIZE as i32
            || y >= CHUNK_SIZE as i32
            || z >= CHUNK_SIZE as i32
        {
            return VoxelType::Empty;
        }
        let idx = Self::get_idx(x as usize, y as usize, z as usize);
        self.voxels[idx]
    }

    pub fn explore(&mut self, x: usize, y: usize, z: usize) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            let idx = Self::get_idx(x, y, z);
            self.explored[idx] = true;
        }
    }

    pub fn is_explored(&self, x: usize, y: usize, z: usize) -> bool {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.explored[Self::get_idx(x, y, z)]
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_getVoxel_outOfBounds_returnsEmpty() {
        let chunk = Chunk::new();
        assert_eq!(chunk.get_voxel(-1, 0, 0), VoxelType::Empty);
        assert_eq!(chunk.get_voxel(CHUNK_SIZE as i32, 0, 0), VoxelType::Empty);
    }

    #[test]
    fn test_chunk_setVoxel_validCoordinates_updatesVoxelAndSetsDirtyFlag() {
        let mut chunk = Chunk::new();
        chunk.is_dirty = false;
        chunk.set_voxel(5, 5, 5, VoxelType::Stone);
        assert_eq!(chunk.get_voxel(5, 5, 5), VoxelType::Stone);
        assert!(chunk.is_dirty);
    }

    #[test]
    fn test_chunk_setVoxel_outOfBounds_doesNotPanic() {
        let mut chunk = Chunk::new();
        chunk.set_voxel(CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE, VoxelType::Stone);
        assert_eq!(chunk.get_voxel(0, 0, 0), VoxelType::Empty);
    }

    #[test]
    fn test_explore_marksExplored() {
        let mut chunk = Chunk::new();
        assert!(!chunk.is_explored(1, 1, 1));
        chunk.explore(1, 1, 1);
        assert!(chunk.is_explored(1, 1, 1));
    }
}
