use core::f64;

use indicatif::ProgressBar;

use crate::color;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

use crate::rtweekend::random_f64;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel

    image_height: i32,        // Rendered image height
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples

    pub max_depth: i32, // Maximum number of ray bounces into scene
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            max_depth: 50,
        }
    }
    pub fn ray_color<T: Hittable>(&self, r: Ray, world: &T, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec: HitRecord = HitRecord::default();

        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit();
            return self.ray_color(Ray::new(rec.p, direction), world, depth - 1) * 0.1;
        }

        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let bar = ProgressBar::new((self.image_width * self.image_height) as u64);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(r, world, self.max_depth);
                }
                color::write_color(self.pixel_samples_scale * pixel_color);

                bar.inc(1);
            }
        }
        bar.finish();
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}
