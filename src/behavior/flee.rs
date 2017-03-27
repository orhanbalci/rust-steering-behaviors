use super::super::{SteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

/// This behavior is the oposite of Seek behavior. It produces linear steering acceleration
/// to go away from target
#[derive(Builder)]
pub struct Flee<'a, T>
    where T: 'a + Real
{
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// Target to go away from 
    pub target: &'a Steerable<T>,
}


impl<'a, T: 'a + Real> SteeringAccelerationCalculator<T> for Flee<'a, T> {
    fn calculate_real_steering<'b>(self: &mut Self,
                                   steering_acceleration: &'b mut SteeringAcceleration<T>,
                                   owner: &'b Steerable<T>)
                                   -> &'b mut SteeringAcceleration<T> {

        steering_acceleration.linear = (*owner.get_position() - *self.target.get_position())
                                           .normalize()
                                           .multiply_by(match self.behavior
                                                                  .limiter {
                                               Some(l) => (*l).get_max_linear_acceleration(),
                                               None => T::one(),
                                           });

        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(self: &Self) -> bool {
        self.behavior.enabled
    }
}
