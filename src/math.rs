use impl_ops::*;
use std::f32::consts::PI;
use std::ops;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

// Use a macro to implement operations
// https://docs.rs/crate/impl_ops/0.1.1

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 {
        Vector3{
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
});

impl_op_ex!(-|a: &Vector3, b: &Vector3| -> Vector3 {
    Vector3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(*|a: &Vector3, mul: &f32| -> Vector3 {
    Vector3 {
        x: a.x * mul,
        y: a.y * mul,
        z: a.z * mul,
    }
});

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    // Rounds to the nearest whole number
    pub fn round(&self) -> Self {
        Self {
            x: self.x as i32 as f32,
            y: self.y as i32 as f32,
        }
    }
}

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(n: (f32, f32)) -> Self {
        Self { x: n.0, y: n.1 }
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from(n: [f32; 2]) -> Self {
        Self { x: n[0], y: n[1] }
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from(n: (i32, i32)) -> Self {
        Self {
            x: n.0 as _,
            y: n.1 as _,
        }
    }
}

// Use a macro to implement operations
// https://docs.rs/crate/impl_ops/0.1.1

impl_op_ex!(+ |a: &Vector2, b: &Vector2| -> Vector2 {
        Vector2{
            x: a.x + b.x,
            y: a.y + b.y,
        }
});

impl_op_ex!(-|a: &Vector2, b: &Vector2| -> Vector2 {
    Vector2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
/// Represent pitch / yaw view angles
/// Pitch: +down / -up
/// Yaw: +right / -left
pub struct Angles2 {
    pub pitch: f32,
    pub yaw: f32,
}

impl Angles2 {
    /// Creates a new Angles2 struct using a pitch and yaw and clamps it
    pub fn new(pitch: f32, yaw: f32) -> Self {
        let mut new_angles = Self { pitch, yaw };
        new_angles.clamp();
        new_angles
    }

    pub fn is_zero(&self) -> bool {
        self.pitch == 0.0 && self.yaw == 0.0
    }

    /// Clamps the angles between:
    /// Pitch: [-90, 90]
    /// Yaw: [-180, 180]
    pub fn clamp(&mut self) {
        while self.pitch > 90.0 {
            self.pitch -= 90.0
        }
        while self.pitch < -90.0 {
            self.pitch += 90.0
        }

        while self.yaw > 180.0 {
            self.yaw -= 180.0
        }
        while self.yaw < -180.0 {
            self.yaw += 180.0
        }
    }

    pub fn length(&self) -> f32 {
        f32::hypot(self.pitch, self.yaw)
    }

    pub fn normalize(&mut self) {
        if self.pitch > 180.0 {
            self.pitch -= 360.0;
        }
        if self.pitch < -180.0 {
            self.pitch += 360.0
        }

        if self.yaw > 180.0 {
            self.yaw -= 360.0
        }
        if self.yaw < -180.0 {
            self.yaw += 360.0
        }
    }
}

impl_op_ex!(-|a: &Angles2, b: &Angles2| -> Angles2 {
    Angles2 {
        pitch: a.pitch - b.pitch,
        yaw: a.yaw - b.yaw,
    }
});
impl_op_ex!(+ |a: &Angles2, b: &Angles2| -> Angles2 {
        Angles2{
            pitch: a.pitch + b.pitch,
            yaw: a.yaw + b.yaw,
        }
});
impl_op_ex!(/ |a: &Angles2, div: &f32| -> Angles2 {
    Angles2{
        pitch: a.pitch / div,
        yaw: a.yaw / div
    }
});
impl_op_ex!(* |a: &Angles2, mul: &f32| -> Angles2 {
    Angles2{
        pitch: a.pitch * mul,
        yaw: a.yaw * mul
    }
});

/// Calculates the angle between `source` & `dest` relative to the current `angles`
pub fn calculate_relative_angles(source: &Vector3, dest: &Vector3, angles: &Angles2) -> Angles2 {
    let delta = dest - source;
    let mut relative_angles = Angles2 {
        pitch: radians_to_deg(f32::atan2(-delta.z, f32::hypot(delta.x, delta.y))) - angles.pitch,
        yaw: radians_to_deg(f32::atan2(delta.y, delta.x)) - angles.yaw,
    };
    relative_angles.normalize();

    relative_angles
}

pub fn deg_to_radians(degrees: f32) -> f32 {
    degrees * (PI / 180.0)
}

pub fn radians_to_deg(radians: f32) -> f32 {
    radians * (180.0 / PI)
}
