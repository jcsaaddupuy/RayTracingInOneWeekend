use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = r_in.direction().reflect(rec.normal);

        reflected = reflected.unit() + (Vec3::random_unit() * self.fuzz);
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }
        return None;
    }
}

pub struct Dielectric {
    refraction_index: f64,
}
impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let mut ri = self.refraction_index;
        if rec.front_face {
            ri = (1.0 / self.refraction_index);
        };

        let unit_direction = r_in.direction().unit();

        let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let mut direction = unit_direction.refract(rec.normal, ri);

        if cannot_refract {
            direction = unit_direction.reflect(rec.normal);
        }

        let scattered: Ray = Ray::new(rec.p, direction);
        return Some((scattered, attenuation));
    }
}
