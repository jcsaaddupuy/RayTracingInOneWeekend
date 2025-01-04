use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     // #[test]
//     // fn test_new() {
//     //     let hc = HitRecord::default();
//     //     hc.front_face()

//     // }
// }
