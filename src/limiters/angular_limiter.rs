use alga::general::Real;
use super::super::Limiter;

#[allow(dead_code)]
pub struct AngularLimiter<T: Real> {
    max_angular_acceleration: T,
    max_angular_speed: T,
}

impl<T: Real> Limiter<T> for AngularLimiter<T> {
    fn get_zero_linear_speed_threshold(&self) -> T {
        unreachable!("get_zero_linear_speed_threshold is not implemented for AngularLimiter");
    }

    #[allow(unused)]
    fn set_zero_linear_speed_threshold(&mut self, speed_threshold: T) {
        unreachable!("set_zero_linear_speed_threshold is not implemented for AngularLimiter");
    }

    fn get_max_linear_speed(&self) -> T {
        unreachable!("get_max_linear_speed is not implmented for AngularLimiter");
    }

    #[allow(unused)]
    fn set_max_linear_speed(&mut self, linear_speed: T) {
        unreachable!("set_max_linear_speed is not implemented for AngularLimiter");
    }

    fn get_max_linear_acceleration(&self) -> T {
        unreachable!("get_max_linear_acceleration is not implemented for AngularLimiter");
    }

    #[allow(unused)]
    fn set_max_linear_acceleration(&mut self, linear_acceleration: T) {
        unreachable!("set_max_linear_acceleration is not implemented for AngularLimiter");
    }

    fn get_max_angular_speed(&self) -> T {
        self.max_angular_speed
    }

    fn set_max_angular_speed(&mut self, angular_speed: T) {
        self.max_angular_speed = angular_speed;
    }

    fn get_max_angular_acceleration(&self) -> T {
        self.max_angular_acceleration
    }

    fn set_max_angular_acceleration(&mut self, angular_acceleration: T) {
        self.max_angular_acceleration = angular_acceleration;
    }
}
