const ROTATION_SPEED: f32 = 0.01;
const FREE_LOOK_MOVEMENT_SPEED: f32 = 1.0;

pub struct Camera {
    position: glam::Vec3,
    direction: glam::Vec3,
    up: glam::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: glam::Vec3::new(64.0, 40.0, 64.0),
            direction: glam::Vec3::new(0.0, -1.0, 0.0),
            up: glam::Vec3::new(0.0, 0.0, 1.0),
        }
    }
    pub fn update(&mut self, forward: f32, right: f32, yaw: f32, pitch: f32) {
        self.direction = 
            freelook_rotate(
                    self.direction,
                    pitch * ROTATION_SPEED,
                    yaw * ROTATION_SPEED,
                );
        self.position = freelook_move(
                    self.position,
                    self.direction,
                    forward * FREE_LOOK_MOVEMENT_SPEED,
                    right * FREE_LOOK_MOVEMENT_SPEED,
                );
    }
    pub fn view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_to_rh(self.position, self.direction, self.up)
    }
}

pub fn freelook_move(position: glam::Vec3, direction: glam::Vec3, forward: f32, right: f32) -> glam::Vec3 {
    let right_vector = direction.cross(glam::Vec3::new(0.0, 0.0, 1.0));
    position + direction * forward + right_vector * right
}

pub fn freelook_rotate(direction: glam::Vec3, updown: f32, around: f32) -> glam::Vec3 {
    let left_vector = glam::Vec3::new(0.0, 0.0, 1.0).cross(direction);
    let rotation_z = glam::Quat::from_axis_angle(glam::Vec3::new(0.0, 0.0, 1.0), around);
    let rotation_left = glam::Quat::from_axis_angle(left_vector, updown);
    let temp_direction = (rotation_z * direction).normalize();
    (rotation_left * temp_direction).normalize()
}

