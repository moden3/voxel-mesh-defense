use noise::{NoiseFn, Perlin};
use crate::voxel::{Chunk, VoxelType, CHUNK_SIZE};
use crate::flow_field::FlowField;
use crate::swarm::Worker;

/// ゲーム全体の状態を一元管理する構造体。
pub struct GameState {
    pub chunk: Chunk,
    pub flow_field: FlowField,
    pub workers: Vec<Worker>,
    /// 現在表示中のYレイヤー（断面図の深度）
    pub current_layer: usize,
    /// マーカー配置済み座標（x, y, z）
    pub marker: Option<(usize, usize, usize)>,
}

impl GameState {
    pub fn new() -> Self {
        let mut chunk = Chunk::new();
        // Perlin Noiseで初期地形を生成する
        let perlin = Perlin::new(42);
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let nx = x as f64 * 0.1;
                let nz = z as f64 * 0.1;
                let height = (perlin.get([nx, nz]) * 8.0 + 14.0) as usize;
                for y in 0..CHUNK_SIZE {
                    if y < height {
                        let voxel = if y + 3 < height {
                            VoxelType::Stone
                        } else {
                            VoxelType::Dirt
                        };
                        chunk.set_voxel(x, y, z, voxel);
                    }
                }
            }
        }
        // スタート地点（表面付近）を探索済みにする
        let start_layer = 12;
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.explore(x, start_layer, z);
            }
        }

        Self {
            chunk,
            flow_field: FlowField::default(),
            workers: Vec::new(),
            current_layer: start_layer,
            marker: None,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let flow_field = &self.flow_field;
        for worker in &mut self.workers {
            worker.update(flow_field, delta);
        }
    }

    /// クリックされたボクセルを掘削し、フローフィールドを更新する。
    pub fn dig_voxel(&mut self, x: usize, z: usize) {
        let y = self.current_layer;
        if self.chunk.get_voxel(x as i32, y as i32, z as i32) != VoxelType::Empty {
            self.chunk.set_voxel(x, y, z, VoxelType::Empty);
            self.chunk.explore(x, y, z);
        }
        // マーカーを配置してフローフィールドを再計算する
        self.chunk.set_voxel(x, y, z, VoxelType::Marker);
        self.marker = Some((x, y, z));
        if let Some(grid) = FlowField::build_from_bfs(
            &self.chunk,
            x as i32,
            y as i32,
            z as i32,
        ) {
            self.flow_field.grid = grid;
        }
        // ワーカーをスポーン（最大8体）
        if self.workers.len() < 8 {
            self.workers.push(Worker::new(
                (CHUNK_SIZE / 2) as f32,
                y,
                (CHUNK_SIZE / 2) as f32,
            ));
        }
    }

    pub fn layer_up(&mut self) {
        if self.current_layer + 1 < CHUNK_SIZE {
            self.current_layer += 1;
        }
    }

    pub fn layer_down(&mut self) {
        if self.current_layer > 0 {
            self.current_layer -= 1;
        }
    }
}
