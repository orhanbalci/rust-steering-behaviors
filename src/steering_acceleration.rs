use nalgebra::{Vector3, Repeat};
use num::num_traits::Num;
use num::Zero;
use std::ops::AddAssign;
use std::ops::MulAssign;

/// Represents result of a steering behaviour computation. User can aggregate
/// more than one behaviour result into single acceleration struct.
pub struct SteeringAcceleration<T: Num + AddAssign + MulAssign + Copy> {
    pub linear: Vector3<T>,
    pub angular: T,
}


impl<T: Num + AddAssign + MulAssign + Copy> SteeringAcceleration<T> {
    /// Creates a steering acceleration struct using given linear and angular components
    fn new(linear_acceleration: Vector3<T>, angular_acceleration: T) -> SteeringAcceleration<T> {
        SteeringAcceleration {
            linear: linear_acceleration,
            angular: angular_acceleration,
        }
    }

    /// Tests whether both linear and angular acceleration compenents are zero
    fn is_zero(self: &Self) -> bool {
        self.angular.is_zero() && self.linear.is_zero()
    }

    /// Sets both compononents to zero
    fn set_zero(self: &mut Self) {
        self.angular = T::zero();
        self.linear = Vector3::zero();
    }

    /// 
    fn add(self: &mut Self, other: SteeringAcceleration<T>) -> &mut Self {
        self.angular = self.angular + other.angular;
        self.linear += other.linear;
        self
    }

    fn scl(self : &mut Self, scale : T) -> &mut Self{
        self.angular = self.angular * scale;
        self.linear *= Vector3::repeat(scale);        
        self
    }
}
