use vec::Vec3;

#[derive(Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

    pub fn origin<'a>(&'a self) -> &'a Vec3 {
        &self.a
    }

    pub fn direction<'a>(&'a self) -> &'a Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        &self.a + t * &self.b
    }
}
