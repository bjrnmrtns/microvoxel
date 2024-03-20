const ROTATION_SPEED: f32 = 5.0;
const FREE_LOOK_MOVEMENT_SPEED: f32 = 20.0;

pub fn camera() {
    let time_elapsed_sec = 0.01;
    let pitch = 0.0;
    let yaw = 0.0;
    let forward = 0.0;
    let right = 0.0;
    let direction_in = glam::Vec3::new(1.0, 0.0, 0.0);
    let position_in = glam::Vec3::new(1.0, 0.0, 0.0);
    let direction = 
            freelook_rotate(
                    direction_in,
                    pitch * time_elapsed_sec * ROTATION_SPEED,
                    yaw * time_elapsed_sec * ROTATION_SPEED,
                );
    let position = freelook_move(
                    position_in,
                    direction_in,
                    forward * time_elapsed_sec * FREE_LOOK_MOVEMENT_SPEED,
                    right * time_elapsed_sec * FREE_LOOK_MOVEMENT_SPEED,
                );
}

pub fn freelook_move(position: glam::Vec3, direction: glam::Vec3, forward: f32, right: f32) -> glam::Vec3 {
    let right_vector = direction.cross(glam::Vec3::new(0.0, 1.0, 0.0));
    position + direction * forward + right_vector * right
}

pub fn freelook_rotate(direction: glam::Vec3, updown: f32, around: f32) -> glam::Vec3 {
    let left_vector = glam::Vec3::new(0.0, 1.0, 0.0).cross(direction);
    let rotation_y = glam::Quat::from_axis_angle(glam::Vec3::new(0.0, 1.0, 0.0), around);
    let rotation_left = glam::Quat::from_axis_angle(left_vector, updown);
    let temp_direction = (rotation_y * direction).normalize();
    (rotation_left * temp_direction).normalize()
}
