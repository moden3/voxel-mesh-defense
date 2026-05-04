use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct FlyCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            speed: 10.0,
            sensitivity: 0.002,
        }
    }
}

pub fn fly_camera_system(
    mut q_camera: Query<(&mut Transform, &mut FlyCamera)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let (mut transform, mut fly_cam) = match q_camera.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    let mut delta_mouse = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta_mouse += event.delta;
    }

    // カメラの回転を更新
    fly_cam.yaw -= delta_mouse.x * fly_cam.sensitivity;
    fly_cam.pitch -= delta_mouse.y * fly_cam.sensitivity;
    
    // ピッチが真上・真下を超えないように制限（ジンバルロック防止）
    use std::f32::consts::FRAC_PI_2;
    fly_cam.pitch = fly_cam.pitch.clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);

    transform.rotation = Quat::from_axis_angle(Vec3::Y, fly_cam.yaw) * Quat::from_axis_angle(Vec3::X, fly_cam.pitch);

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += transform.forward().into();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += transform.back().into();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += transform.left().into();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += transform.right().into();
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    // 移動方向の正規化と適用
    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * fly_cam.speed * time.delta_seconds();
    }
}
