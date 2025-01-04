use crate::hittable::{HitRecord, Hittable};

// #[derive(Copy, Clone, Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList {
            objects: Vec::new(),
        };
    }
    pub fn add<'a, T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
    pub fn clear(mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit<'a>(
        &self,
        r: crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &'a mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for obj in self.objects.iter() {
            if obj.hit(r, ray_tmin, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }
}
