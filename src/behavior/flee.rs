use nalgebra::Vector3;
use super::super::{SteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;

/// This behavior is the oposite of Seek behavior. It produces linear steering acceleration
/// to go away from target
pub struct Flee<'a, T: 'a + Real> {
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// Target to go away from 
    pub target: Vector3<T>,
}


impl<'a, T: 'a + Real> SteeringAccelerationCalculator<T> for Flee<'a, T> {
    fn calculate_real_steering<'b>(self: &mut Self,
                                   steering_acceleration: &'b mut SteeringAcceleration<T>)
                                   -> &'b mut SteeringAcceleration<T> {

        steering_acceleration.linear = (*self.behavior.owner.get_position() - self.target)
                                           .normalize()
                                           .multiply_by(self.behavior
                                                            .limiter
                                                            .get_max_linear_acceleration());

        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(self: &Self) -> bool {
        self.behavior.enabled
    }
}
