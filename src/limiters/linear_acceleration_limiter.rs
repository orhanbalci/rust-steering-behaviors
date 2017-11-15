use alga::general::Real;
use super::super::Limiter;

#[allow(dead_code)]
pub struct LinearAccelerationLimiter<T: Real> {
    max_linear_acceleration: T,
}

impl<T: Real> Limiter<T> for LinearAccelerationLimiter<T> {
    fn get_zero_linear_speed_threshold(&self) -> T {
        unreachable!("get_zero_linear_threshold is not implemented for LinearAccelerationLimiter");
    }

    #[allow(unused)]
    fn set_zero_linear_speed_threshold(&mut self, threshold: T) {
        unreachable!("set_zero_linear_threshold is not implemented for LinearAccelerationLimiter");
    }

    fn get_max_linear_speed(&self) -> T {
        unreachable!("get_max_linear_speed is not implemented for LinearAccelerationLimiter");
    }

    #[allow(unused)]
    fn set_max_linear_speed(&mut self, linear_speed: T) {
        unreachable!("set_max_linear_speed is not implemented for LinearAccelerationLimiter");
    }

    fn get_max_linear_acceleration(&self) -> T {
        self.max_linear_acceleration
    }

    fn set_max_linear_acceleration(&mut self, linear_acceleration: T) {
        self.max_linear_acceleration = linear_acceleration;
    }

    fn get_max_angular_speed(&self) -> T {
        unreachable!("get_max_angular_speed is not implemented for LinearAccelerationLimiter");
    }

    #[allow(unused)]
    fn set_max_angular_speed(&mut self, angular_speed: T) {
        unreachable!("set_max_angular_speed is not implemented for LinearAccelerationLimiter");
    }

    fn get_max_angular_acceleration(&self) -> T {
        unreachable!(
            "get_max_angular_acceleration is not implemented for LinearAccelerationLimiter"
        );
    }

    #[allow(unused)]
    fn set_max_angular_acceleration(&mut self, angular_acceleration: T) {
        unreachable!(
            "set_max_angular_acceleration is not implmented for LinearAccelerationLimiter"
        );
    }
}
