use image::{RgbImage, ImageBuffer, Rgb};
use cgmath::Vector3;
use cgmath::InnerSpace;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

struct Random {
    rng : Box<Pcg64Mcg>,
}

impl Random {
    pub fn new() -> Self {
        let rng = Box::new(Pcg64Mcg::new(42));
        Self {
            rng,
        }
    }
    pub fn random_f64(&mut self) -> f64 {
        self.rng.gen_range(0.0..1.0)
    }
    pub fn random_f64_min_max(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.random_f64()
    }
    pub fn sample_square(&mut self) -> Vector3<f64> {
        Vector3::new(self.random_f64() - 0.5, self.random_f64() - 0.5, 0.0)
    }
    pub fn random_vector3_min_max(&mut self, min: f64, max: f64) -> Vector3<f64> {
        Vector3::new(self.random_f64_min_max(min, max), self.random_f64_min_max(min, max), self.random_f64_min_max(min, max))
    }
    pub fn random_in_unit_sphere(&mut self) -> Vector3<f64> {
        loop {
            let p = self.random_vector3_min_max(-1.0, 1.0);
            if p.magnitude() < 1.0 {
                return p;
            }
        }
    }
    pub fn random_unit_vector(&mut self) -> Vector3<f64> {
        self.random_in_unit_sphere().normalize()
    }
    pub fn random_on_hemisphere(&mut self, normal: Vector3<f64>) -> Vector3<f64> {
        let on_unit_sphere = self.random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

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

fn ray_color(ray: &Ray) -> Vector3<f64> {
    Vector3::new(1.0, 0.0, 0.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0; 
    const IMAGE_WIDTH: u32 = 400;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH : u32 = 50;
    const PIXEL_SAMPLE_SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
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

    let mut random = Random::new();

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut color = Vector3::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let offset = random.sample_square();
            let pixel_sample = pixel00_loc + ((x as f64 + offset.x) * pixel_delta_u) + ((y as f64 + offset.y) * pixel_delta_v);

            let ray = Ray::new(CAMERA_CENTER, pixel_sample - CAMERA_CENTER);
            color = color + ray_color(&ray);
        }
        color = PIXEL_SAMPLE_SCALE * color;

        const INTENSITY : Interval = Interval::new(0.000, 0.999);
        let ir = (256.0 * INTENSITY.clamp(color.x)) as u8;
        let ig = (256.0 * INTENSITY.clamp(color.y)) as u8;
        let ib = (256.0 * INTENSITY.clamp(color.z)) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }
    buffer.save("render.png").unwrap();
}
