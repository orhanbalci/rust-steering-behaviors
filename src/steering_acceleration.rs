use nalgebra::{Vector3, Repeat, Point3, FloatPoint, BaseFloat, ApproxEq};
use num::Zero;

/// Represents result of a steering behaviour computation. User can aggregate
/// more than one behaviour result into single acceleration struct.
pub struct SteeringAcceleration<T: BaseFloat + ApproxEq<T>> {
    /// linear acceleration component
    pub linear: Vector3<T>,
    /// angular acceleration component
    pub angular: T,
}


impl<T: BaseFloat + ApproxEq<T>> SteeringAcceleration<T> {
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
    fn set_zero(self: &mut Self) -> &mut Self {
        self.angular = T::zero();
        self.linear = Vector3::zero();
        self
    }

    /// 
    fn add(self: &mut Self, other: SteeringAcceleration<T>) -> &mut Self {
        self.angular = self.angular + other.angular;
        self.linear += other.linear;
        self
    }

    ///
    fn scl(self: &mut Self, scale: T) -> &mut Self {
        self.angular = self.angular * scale;
        self.linear *= Vector3::repeat(scale);
        self
    }

    ///
    fn mulAdd(self: &mut Self, other: SteeringAcceleration<T>, scale: T) -> &mut Self {
        self.angular = self.angular + (other.angular * scale);
        self.linear += other.linear * Vector3::repeat(scale);
        self
    }

    ///
    fn calculate_square_magnitude(self: &Self) -> T {
        self.linear.as_point().distance_squared(&Vector3::zero().to_point()) +
        self.angular * self.angular
    }

    ///
    fn calculate_magnitude(self: &Self) -> T {
        self.calculate_square_magnitude().sqrt()
    }
}

pub trait SteeringAccelerationCalculator<T: BaseFloat + ApproxEq<T>> {
    fn calculate_steering<'a>(self: &mut Self,
                              steering_acceleration: &'a mut SteeringAcceleration<T>)
                              -> &'a mut SteeringAcceleration<T> {
        if self.is_enabled() {
            self.calculate_real_steering(steering_acceleration)
        } else {
            steering_acceleration.set_zero()
        }
    }
    fn calculate_real_steering<'a>(self: &mut Self,
                                   steering_acceleration: &'a mut SteeringAcceleration<T>)
                                   -> &'a mut SteeringAcceleration<T>;
    fn is_enabled(self: &Self) -> bool;
}
