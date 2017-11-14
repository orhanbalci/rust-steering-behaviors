use alga::general::Real;
use super::super::Limiter;

#[allow(dead_code)]
pub struct FullLimiter<T: Real> {
    max_linear_acceleration: T,
    max_linear_speed: T,
    max_angular_acceleration: T,
    max_angular_speed: T,
    zero_linear_speed_threshold: T,
}

impl<T: Real> Limiter<T> for FullLimiter<T> {
    fn get_zero_linear_speed_threshold(&self) -> T {
        self.zero_linear_speed_threshold
    }

    fn set_zero_linear_speed_threshold(&mut self, threshold: T) {
        self.zero_linear_speed_threshold = threshold;
    }

    fn get_max_linear_speed(&self) -> T {
        self.max_linear_speed
    }

    fn set_max_linear_speed(&mut self, linear_speed: T) {
        self.max_linear_speed = linear_speed;
    }

    fn get_max_linear_acceleration(&self) -> T {
        self.max_linear_acceleration
    }

    fn set_max_linear_acceleration(&mut self, linear_acceleration: T) {
        self.max_linear_acceleration = linear_acceleration;
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
