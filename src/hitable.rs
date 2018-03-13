use std::option::Option;
use vec::Vec3;
use ray::Ray;


#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,       // ray position parameter
    pub p: Vec3,      // hit point at parameter t
    pub normal: Vec3, // surface normal at hit point
}


pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}


pub struct HitableList<'a> {
    pub list: Vec<&'a Hitable>,
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for h in self.list.iter() {
            let x = h.hit(r, t_min, closest_so_far);
            match x {
                Some(rec) => {
                   closest_so_far = rec.t;
                   temp_rec = Some(rec)
                },
                None => {} 
            }
        }

        temp_rec
    }

}