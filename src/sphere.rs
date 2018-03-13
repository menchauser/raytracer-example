use vec::*;
use ray::Ray;
use hitable::*;


pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let sol1 = (- b - f32::sqrt(discriminant)) / a;
            if sol1 < t_max && sol1 > t_min {
                let p = &r.point_at_parameter(sol1);
                let rec = HitRecord {
                    t: sol1,
                    p: *p,
                    normal: (p - &self.center) / self.radius,
                };
                return Some(rec)
            } 
            let sol2 = (- b + f32::sqrt(discriminant)) / a;
            if sol2 < t_max && sol2 > t_min {
                let p = &r.point_at_parameter(sol2);
                let rec = HitRecord {
                    t: sol1,
                    p: *p,
                    normal: (p - &self.center) / self.radius
                };
                return Some(rec)
            }
        } 
        return None
    }
}