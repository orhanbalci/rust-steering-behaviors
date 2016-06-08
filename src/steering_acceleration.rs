use nalgebra::{Vector3, Repeat, Point3, FloatPoint, BaseFloat, ApproxEq};
use num::Zero;

/// Represents result of a steering behaviour computation. User can aggregate
/// more than one behaviour result into single acceleration struct.
#[derive(Debug)]
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

impl<T: BaseFloat + ApproxEq<T>> PartialEq for SteeringAcceleration<T> {
    fn eq(&self, other: &Self) -> bool {
        self.linear == other.linear && self.angular == other.angular
    }
    fn ne(&self, other: &Self) -> bool {
        self.linear != other.linear || self.angular != other.angular
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

#[cfg(test)]
mod test {
    use super::SteeringAcceleration;
    use nalgebra::Vector3;
    #[test]
    fn is_zero_positive() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 2.0, 3.0), 5.0f32);
        acceleration.set_zero();
        assert!(acceleration.is_zero());
    }

    #[test]
    fn is_zero_negative() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 2.0, 3.0), 5.0f32);
        assert_eq!(acceleration.is_zero(), false);
    }

    #[test]
    fn add() {
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0f32);
        let acceleration2 = SteeringAcceleration::new(Vector3::new(1.0f32, 1.0, 1.0), 1.0f32);
        acceleration.add(acceleration2);
        assert_eq!(SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0f32), acceleration);
    }

    #[test]
    fn scl(){
        let mut acceleration = SteeringAcceleration::new(Vector3::new(1.0f32,1.0,1.0), 1.0f32);
        acceleration.scl(2.0f32);
        assert_eq!(SteeringAcceleration::new(Vector3::new(2.0f32, 2.0, 2.0), 2.0), acceleration);
    }

    #[test]
    fn calculate_square_magnitude(){
        let mut acceleration = SteeringAcceleration::new(Vector3::new(2.0f32,2.0,2.0), 2.0f32);
        assert_eq!(16f32, acceleration.calculate_square_magnitude());
    }
}
