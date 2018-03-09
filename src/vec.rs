
pub struct Vec3 {
    e: [f32; 3],
}


impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    #[inline]
    pub fn x(&self) -> f32 { self.e[0] }

    #[inline]
    pub fn y(&self) -> f32 { self.e[1] }

    #[inline]
    pub fn z(&self) -> f32 { self.e[2] }

    #[inline]
    pub fn r(&self) -> f32 { self.e[0] }

    #[inline]
    pub fn g(&self) -> f32 { self.e[1] }

    #[inline]
    pub fn b(&self) -> f32 { self.e[2] }
}