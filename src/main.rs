use image::{RgbImage, ImageBuffer, Rgb};
use cgmath::Vector3;
use cgmath::InnerSpace;
use cgmath::VectorSpace;

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

fn hit_sphere(ray: &Ray, center: Vector3<f64>, radius: f64) -> Option<f64> {
    let oc = center - ray.origin;
    let a = ray.dir.dot(ray.dir);
    let b = -2.0 * ray.dir.dot(oc);
    let c = oc.dot(oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    if discriminant >= 0.0 {
        Some((-b - discriminant.sqrt()) / 2.0 * a)
    } else {
        None
    }
}

fn ray_color(ray: &Ray) -> Vector3<f64> {
    let t = hit_sphere(ray, Vector3::new(0.0, 0.0, -1.0), 0.5);

    if let Some(t) = t {
        let n = (ray.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        0.5 * (n + Vector3::new(1.0, 1.0, 1.0))
    } else {
        let normalized_y = 0.5 * (ray.dir.normalize().y + 1.0);
        Vector3::new(1.0, 1.0, 1.0).lerp(Vector3::new(0.5, 0.7, 1.0), normalized_y)
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0; 
    const IMAGE_WIDTH: u32 = 400;
    const CALCULATED_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = if CALCULATED_HEIGHT > 0 { CALCULATED_HEIGHT } else { 1 };

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
        let pixel_center = pixel00_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
        let ray_direction = pixel_center - CAMERA_CENTER;
        let ray = Ray::new(CAMERA_CENTER, ray_direction);
            
        let color = ray_color(&ray); 
        let ir = (255.99 * color.x) as u8;
        let ig = (255.99 * color.y) as u8;
        let ib = (255.99 * color.z) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }
    buffer.save("render.png").unwrap();

}
