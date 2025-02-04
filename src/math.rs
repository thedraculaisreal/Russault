use libm;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Default ,Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Default ,Copy, Clone)]
pub struct Vec4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
    pub const fn new_const(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
    fn calc_length(&mut self) -> f32{
        libm::sqrtf(self.x * self.x + self.y * self.y + self.z * self.z)
    }
    pub fn calc_distance(&mut self, target: Vec3) -> f32 {
        self.x -= target.x;
        self.y -= target.y;
        self.z -= target.z;
        self.calc_length()
    }
    pub fn absf(&mut self) {
	self.x = libm::fabsf(self.x);
	self.y = libm::fabsf(self.y);
	self.z = libm::fabsf(self.z);
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }        
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;   
    }
}
impl Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }        
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;   
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }        
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;    
    }
}
const PI: f32 = 3.14159;
fn radians_to_degrees(radians_angle: f32) -> f32 {
    radians_angle * (180.0 / PI)
}
pub fn calculate_angle(mut origin: Vec3, target: Vec3) -> Vec3 {
    let mut results: Vec3 = Vec3::new(0.0,0.0,0.0);
    results.x = radians_to_degrees(-(libm::atan2f(origin.x - target.x , origin.y - target.y)));
    results.y = radians_to_degrees(libm::asinf((target.z - origin.z) / origin.calc_distance(target.clone())));
    return results
}
pub fn world_to_screen(pos: Vec3, matrix: [f32; 16]) -> Vec3 {
    let mut camera_coords: Vec4 = Vec4 {
	x: 0.0,
	y: 0.0,
	z: 0.0,
	w: 0.0,
    };
    camera_coords.x = pos.x * matrix[0] + pos.y * matrix[4] + pos.z * matrix[8] + matrix[12];
    camera_coords.y = pos.x * matrix[1] + pos.y * matrix[5] + pos.z * matrix[9] + matrix[13];
    camera_coords.z = pos.x * matrix[2] + pos.y * matrix[6] + pos.z * matrix[10] + matrix[14];
    camera_coords.w = pos.x * matrix[3] + pos.y * matrix[7] + pos.z * matrix[11] + matrix[15];
    // if off screen return blank vec
    if camera_coords.w < 0.1 {
	return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut ndc = Vec3::new(0.0, 0.0, 0.0);
    ndc.x = camera_coords.x / camera_coords.w;
    ndc.y = camera_coords.y / camera_coords.w;
    ndc.z = camera_coords.z / camera_coords.w;
    
    return ndc 
}
