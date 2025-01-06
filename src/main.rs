use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use log::info;
use material::{Dielectric, Lambertian, Metal};
use rtweekend::{random_f64, random_f64_bounded};
use sphere::Sphere;
use vec3::{Point3, Vec3};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    colog::init();

    //World
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.add(Sphere::<Lambertian>::new(
        Point3::new(0., -1000., 0.),
        1000.0,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();

                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    let fuzz = random_f64_bounded(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, -0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_depth,
        vfov,
        0.6,
        10.0,
        lookfrom,
        lookat,
        vup,
    );

    camera.render(&world);
    info!("Done");
}
