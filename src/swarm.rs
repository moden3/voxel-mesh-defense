use crate::flow_field::FlowField;
use crate::voxel::CHUNK_SIZE;

#[derive(Clone, Debug, PartialEq)]
pub enum WorkerState {
    Idle,
    MovingToTarget,
    Working,
}

/// 自律型ワーカー（ドローン）。
/// Bevy Transform 非依存のグリッド座標で管理する。
#[derive(Clone, Debug)]
pub struct Worker {
    /// 現在のグリッド座標（浮動小数点で滑らかな移動を実現）
    pub x: f32,
    pub y: usize,
    pub z: f32,
    pub speed: f32,
    pub state: WorkerState,
}

impl Worker {
    pub fn new(x: f32, y: usize, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            speed: 4.0,
            state: WorkerState::MovingToTarget,
        }
    }

    /// フローフィールドを参照して移動する（キネマティック移動）。
    pub fn update(&mut self, flow_field: &FlowField, delta: f32) {
        let xi = self.x.floor() as i32;
        let zi = self.z.floor() as i32;
        if xi < 0 || xi >= CHUNK_SIZE as i32 || zi < 0 || zi >= CHUNK_SIZE as i32 {
            return;
        }
        let idx = FlowField::get_idx(xi as usize, self.y, zi as usize);
        let dir = flow_field.grid[idx];
        if dir[0] == 0.0 && dir[1] == 0.0 {
            self.state = WorkerState::Working;
            return;
        }
        self.x += dir[0] * self.speed * delta;
        self.z += dir[1] * self.speed * delta;
        // グリッド境界内にクランプ
        self.x = self.x.clamp(0.0, (CHUNK_SIZE - 1) as f32);
        self.z = self.z.clamp(0.0, (CHUNK_SIZE - 1) as f32);
    }
}
