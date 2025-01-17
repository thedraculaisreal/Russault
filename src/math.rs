use libm;

#[derive(Default)]
#[derive(Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub fn add(mut self, angle: Vec3) -> Self {
        self.x += angle.x;
        self.y += angle.y;
        self.z += angle.z;
        self
    }
    pub fn subtract(mut self, angle: Vec3) -> Self {
        self.x -= angle.x;
        self.y -= angle.y;
        self.z -= angle.z;
        self
    }
    pub fn multiply_f32(mut self, value: f32) -> Self {
        self.x *= value;
        self.y *= value;
        self.z *= value;
        self
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
}  

const PI: f32 = 3.14159;

fn radians_to_degrees(radians_angle: f32) -> f32 {
    radians_angle * (180.0 / PI)
}

pub fn calculate_angle(mut origin: Vec3, target: Vec3) -> Vec3 {
    let mut results: Vec3 = Vec3::new(0.0,0.0,0.0);
    results.x = radians_to_degrees(-(libm::atan2f(target.x - origin.x, target.y - origin.y)));
    if results.x <= 90.0 {
        results.x += 360.0;
    }
    results.x -= 180.0; // usually 270, for any other game but 180 for assualt cube cuz of true north compensation.
    results.y = radians_to_degrees(libm::asinf((target.z - origin.z) / origin.calc_distance(target.clone())));
    return results
}