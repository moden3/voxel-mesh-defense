use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VoxelType {
    Empty,
    Stone,
    Dirt,
}

/// ボクセルの変更を通知するイベント
/// UIやメッシュ再生成システムがこれを購読して更新処理を行う
#[derive(Event)]
pub struct VoxelChangedEvent {
    pub chunk_entity: Entity,
}

#[derive(Component)]
pub struct Chunk {
    voxels: [[[VoxelType; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub is_dirty: bool, // メッシュの再生成が必要かどうかを判定するフラグ
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [[[VoxelType::Empty; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            is_dirty: true, // 生成直後はメッシュ化が必要なためtrue
        }
    }

    pub fn set_voxel(&mut self, x: usize, y: usize, z: usize, voxel: VoxelType) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE {
            self.voxels[x][y][z] = voxel;
            self.is_dirty = true;
        }
    }

    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> VoxelType {
        // パニックを防ぐため、範囲外アクセスはEmptyとして安全に処理する
        if x < 0 || y < 0 || z < 0 || x >= CHUNK_SIZE as i32 || y >= CHUNK_SIZE as i32 || z >= CHUNK_SIZE as i32 {
            return VoxelType::Empty;
        }
        self.voxels[x as usize][y as usize][z as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_getVoxel_outOfBounds_returnsEmpty() {
        // Arrange
        let chunk = Chunk::new();

        // Act
        let voxel_negative = chunk.get_voxel(-1, 0, 0);
        let voxel_too_large = chunk.get_voxel(CHUNK_SIZE as i32, 0, 0);

        // Assert
        assert_eq!(voxel_negative, VoxelType::Empty, "境界外への負のインデックスアクセスはEmptyを返却すべき");
        assert_eq!(voxel_too_large, VoxelType::Empty, "境界外への過大インデックスアクセスはEmptyを返却すべき");
    }

    #[test]
    fn test_chunk_setVoxel_validCoordinates_updatesVoxelAndSetsDirtyFlag() {
        // Arrange
        let mut chunk = Chunk::new();
        chunk.is_dirty = false; // フラグの更新をテストするためにリセット

        // Act
        chunk.set_voxel(5, 5, 5, VoxelType::Stone);

        // Assert
        assert_eq!(chunk.get_voxel(5, 5, 5), VoxelType::Stone, "指定座標のボクセルがStoneに変更されていること");
        assert!(chunk.is_dirty, "ボクセル変更時にis_dirtyフラグがtrueに設定されること");
    }
}
