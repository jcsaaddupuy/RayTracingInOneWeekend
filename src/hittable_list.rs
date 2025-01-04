use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
// #[derive(Copy, Clone, Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add<'a, T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in self.objects.iter() {
            if obj.hit(r, Interval::new(ray_t.min, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
