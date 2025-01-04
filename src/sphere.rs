use crate::interval::Interval;
use crate::{hittable::Hittable, vec3::Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        return Sphere {
            center,
            radius: f64::max(0.0, radius),
        };
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: crate::ray::Ray,
        ray_t: Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();

        let h = r.direction().dot(oc);

        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
