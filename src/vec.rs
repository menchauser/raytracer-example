use std::ops;
use std::f32;

#[derive(Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
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

    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        let length = self.length();
        self / length
    }
}


#[inline]
pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}


impl<'a> ops::Add for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ]
        }
    }
}


impl<'a> ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        self + &other
    }
}


impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        &self + &other
    }
}


impl<'a> ops::Sub for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]
            ]
        }
    }
}


impl<'a> ops::Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [ self.e[0] * t, self.e[1] * t, self.e[2] * t ]
        }
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        &self * t
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        &v * self
    }
}

impl<'a> ops::Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        &self / t
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.e[0] = self.e[0] / t;
        self.e[1] = self.e[1] / t;
        self.e[2] = self.e[2] / t;
    }
}