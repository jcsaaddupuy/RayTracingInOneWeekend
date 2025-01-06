use core::f64;
use std::sync::Arc;

use indicatif::ProgressBar;

use crate::color;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;

use crate::rtweekend::{degrees_to_radians, random_f64};
use crate::vec3::{Point3, Vec3};
use log::info;
use std::thread;
use threadpool::ThreadPool;

pub struct Camera {
    pub image_width: i32,       // Rendered image width in pixel count
    pub samples_per_pixel: i32, // Count of random samples for each pixel

    image_height: i32,        // Rendered image height
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples

    pub max_depth: i32, // Maximum number of ray bounces into scene

    pub vfov: i32,

    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    //
    // u: Vec3,
    // v: Vec3,
    // w: Vec3,
    defocus_angle: f64, // Variation angle of rays through each pixel
    // focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

unsafe impl Sync for Camera {}
// unsafe impl Send for Camera {}

impl Camera {
    pub fn new(
        image_width: i32,
        aspect_ratio: f64,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: i32,
        defocus_angle: f64,
        focus_dist: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let mut image_height = (image_width as f64 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        let center = lookfrom;

        // Camera
        let focal_length = (lookfrom - lookat).length();

        let theta = degrees_to_radians(vfov as f64);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focus_dist;

        info!("viewport_height: {:?}", viewport_height);

        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(vup.cross(w));
        let v = w.cross(u);

        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist as f64 * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            samples_per_pixel,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            max_depth,
            vfov,

            lookfrom,
            lookat,
            vup,
            //
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub fn ray_color<T: Hittable>(&self, r: Ray, world: &T, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
                return attenuation * self.ray_color(scattered, world, depth - 1);
            }
        }

        let unit_direction = r.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        let bar = Arc::new(ProgressBar::new(
            (self.image_width * self.image_height) as u64,
        ));

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
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin;

        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }
}
