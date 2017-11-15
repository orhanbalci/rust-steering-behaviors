use alga::general::Real;
use super::super::Limiter;

#[allow(dead_code)]
pub struct LinearLimiter<T: Real> {
    max_linear_acceleration: T,
    max_linear_speed: T,
}

impl<T: Real> Limiter<T> for LinearLimiter<T> {
    fn get_zero_linear_speed_threshold(&self) -> T {
        unreachable!("get_zero_linear_speed_threshold is not implemented for LinearLimiter");
    }

    #[allow(unused)]
    fn set_zero_linear_speed_threshold(&mut self, speed_threshold: T) {
        unreachable!("set_zero_linear_speed_threshold is not implemented for LimearLimiter");
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
        unreachable!("get_max_angular_speed is not implemented for LinearLimiter");
    }

    #[allow(unused)]
    fn set_max_angular_speed(&mut self, angular_speed: T) {
        unreachable!("set_max_angular_speed is not implemented for LinearLimiter");
    }

    fn get_max_angular_acceleration(&self) -> T {
        unreachable!("get_max_angular_acceleration is not implemented for LinearLimiter");
    }

    #[allow(unused)]
    fn set_max_angular_acceleration(&mut self, angular_acceleration: T) {
        unreachable!("set_max_angular_acceleration is not implemented for LinearLimiter");
    }
}
