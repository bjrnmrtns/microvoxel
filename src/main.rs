use image::{RgbImage, ImageBuffer, Rgb};
use cgmath::Vector3;
use cgmath::InnerSpace;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

const WORLD: [[u8; 24]; 24] = 
[
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max,
        }
    }
    pub fn length(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }
    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }
    pub fn clamp(&self, value: f64) -> f64 {
        value.clamp(self.min, self.max)
    }
    const EMPTY : Interval = Interval::new(f64::MAX, f64::MIN);
    const UNIVERSE : Interval = Interval::new(f64::MIN, f64::MAX);
}

struct Ray {
    pub origin: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, dir: Vector3<f64>) -> Self {
        Self {
            origin,
            dir,
        }
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.dir * t
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0; 
    const IMAGE_WIDTH: u32 = 400;
    const CALCULATED_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = if CALCULATED_HEIGHT < 1 { 1 } else { CALCULATED_HEIGHT  };

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    const CAMERA_CENTER: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

    let viewport_u: Vector3<f64> = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v: Vector3<f64> = Vector3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left = CAMERA_CENTER - Vector3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    
    let mut buffer : RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let pixel_sample = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);

        let ray = Ray::new(CAMERA_CENTER, pixel_sample - CAMERA_CENTER);
        let mut map_x = ray.origin.x as i32;
        let mut map_y = ray.origin.y as i32;
        let mut map_z = ray.origin.z as i32;

        let delta_dist_x = if ray.dir.x == 0.0 { std::f64::MAX } else { (1.0 / ray.dir.x).abs() };
        let delta_dist_y = if ray.dir.y == 0.0 { std::f64::MAX } else { (1.0 / ray.dir.y).abs() };
        let delta_dist_z = if ray.dir.z == 0.0 { std::f64::MAX } else { (1.0 / ray.dir.z).abs() };

        let (step_x, mut side_dist_x) = if ray.dir.x < 0.0 
            { (-1, (ray.origin.x - map_x as f64) * delta_dist_x ) } 
        else
            {(1, (map_x as f64 + 1.0 - ray.origin.x) * delta_dist_x )};

        let (step_y, mut side_dist_y) = if ray.dir.y < 0.0 
            { (-1, (ray.origin.y - map_y as f64) * delta_dist_y ) } 
        else
            {(1, (map_y as f64 + 1.0 - ray.origin.y) * delta_dist_y )};

        let (step_z, mut side_dist_z) = if ray.dir.z < 0.0 
            { (-1, (ray.origin.z - map_z as f64) * delta_dist_z ) } 
        else
            {(1, (map_z as f64 + 1.0 - ray.origin.z) * delta_dist_z )};

        let mut hit = false;
        let mut hit_nothing = false;
        let mut side : i32 = 0;
        while !hit && !hit_nothing {
            if side_dist_x < side_dist_y {
                if side_dist_x < side_dist_z {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_z += delta_dist_z;
                    map_z += step_z;
                    side = 2;
                }
            } else {
                if side_dist_y < side_dist_z {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                } else {
                    side_dist_z += delta_dist_z;
                    map_z += step_z;
                    side = 2;
                }
            }
/*                if map_x < 24 && map_y < 24 && map_x >= 0 && map_y >= 0 {
                if map_z < WORLD[map_x as usize][map_y as usize] as i32 && map_z >= 0 {
                    hit = true;
                }
            }
            */
            if map_x == 0 && map_y == 0 && map_z == -4 {
                hit = true;
            }
            if map_x < -100 || map_x > 100 || map_y < -100 || map_y > 100 || map_z < -100 || map_z > 100   {
                hit_nothing = true;
            }
        }
        let color: Vector3<f64> = if hit {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };

        const INTENSITY : Interval = Interval::new(0.000, 0.999);
        let ir = (256.0 * INTENSITY.clamp(color.x)) as u8;
        let ig = (256.0 * INTENSITY.clamp(color.y)) as u8;
        let ib = (256.0 * INTENSITY.clamp(color.z)) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }
    buffer.save("render.png").unwrap();
}
