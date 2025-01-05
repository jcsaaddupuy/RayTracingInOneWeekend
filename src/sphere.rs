use crate::hittable::HitRecord;
use crate::interval::Interval;
use crate::material::Material;
use crate::vec3;
use crate::{hittable::Hittable, vec3::Point3};

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    pub mat: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Sphere {
            center,
            radius: f64::max(0.0, radius),
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();

        let h = r.direction().dot(oc);

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: vec3::Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: &self.mat,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
