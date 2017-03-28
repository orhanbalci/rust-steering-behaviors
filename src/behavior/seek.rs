use super::super::SteeringBehavior;
use super::super::{SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

/// Seek behavior calculates the maximum linear valocity to reach the target location
#[derive(Builder)]
pub struct Seek<'a, T>
    where T: 'a + Real
{
    /// common steering behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// steering target
    pub target: &'a Steerable<T>,
}

impl<'a, T: Real> SteeringAccelerationCalculator<T> for Seek<'a, T> {
    fn calculate_real_steering<'b>(&self,
                                   steering_acceleration: &'b mut SteeringAcceleration<T>,
                                   owner: &'b Steerable<T>)
                                   -> &'b mut SteeringAcceleration<T> {
        steering_acceleration.linear = (*self.target.get_position() - *owner.get_position())
                                           .normalize()
                                           .multiply_by(match self.behavior
                                                                  .limiter {
                                               Some(a) => (*a).get_max_linear_acceleration(),
                                               None => T::one(),
                                           });
        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(&self) -> bool {
        self.behavior.enabled
    }
}
