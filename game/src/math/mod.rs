use nalgebra as na;
use std::ops;


pub type V3 = na::Vector3::<f32>;

pub const PI : f32 = std::f32::consts::PI;
pub const PI2 : f32 = std::f32::consts::PI * 2.0;

/// A 2d Rotation around
#[derive(Default, Debug, Clone, Copy)]
pub struct Rotation2 {
    pub radians: f32 // in radians
}

impl Rotation2 {

    pub fn diff(self, other: Rotation2) -> Rotation2 {
        let diff = (self.radians - other.radians + PI) % (PI2) - PI;
        if diff < -PI {
            Rotation2{ radians: - (diff + PI2)}
        } else {
            Rotation2 { radians: - diff}
        }
    }

    pub fn to_homogeneous(self) -> na::Matrix4::<f32> {
        na::Rotation3::from_euler_angles(0.0, 0.0, self.radians).to_homogeneous()
    }

    pub fn interpolate(self, dt: f32, speed: f32) -> Rotation2 {
        let sign = self.radians.signum();

        // if radians.abs < dt*speed use radians, otherwise use dt * speed as max
        let change = self.radians.abs().min(dt*speed);

        let radians = normalize(sign * change);

        Rotation2 { radians }
    }
}


fn normalize(radians: f32) -> f32 {
    (radians  + PI2) % PI2
}

impl ops::Add<Rotation2> for Rotation2 {
    type Output = Self;

    /// Return final radians in [0;PI2]
    fn add(self, rhs: Rotation2) -> Rotation2 {
        let radians = normalize(self.radians + rhs.radians);
        Self { radians }
    }
}

impl ops::AddAssign for Rotation2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}


// assumes that dir.mag > 0
pub fn update_rotation_z(dir: V3) -> f32 {
    dir.y.atan2(dir.x)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn rot_diff() {

        let r1 = Rotation2 {radians: PI / 2.0};
        let r2 = Rotation2 {radians: PI2};

        let diff = r2.diff(r1);

        println!("{:?}", diff);

        assert!(true);
    }

    #[test]
    fn rot_add() {

        let r1 = Rotation2 {radians: PI / 2.0};
        let r2 = Rotation2 {radians: PI / 2.0};

        let r = r1 + r2;

        assert_eq!(r.radians, PI);

    }

    #[test]
    fn diff_1() {

        let r1 = Rotation2 {radians: PI / 2.0};
        let r2 = Rotation2 {radians: PI / 2.0};

        let r = r1.diff(r2);

        assert_eq!(r.radians, 0.0);

    }

}
