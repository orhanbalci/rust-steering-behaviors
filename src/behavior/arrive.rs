use super::super::{Limiter, SteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator};
use nalgebra::{distance, Point3};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

/// This behavior is the oposite of Seek behavior. It produces linear steering acceleration
/// to go away from target
#[derive(Builder)]
pub struct Arrive<'a, T>
    where T: 'a + Real
{
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// Target to go away from 
    pub target: &'a Steerable<T>,
    pub tolerance: T,
    pub deceleration_radius: T,
    pub time_to_target: T,
}


impl<'a, T: 'a + Real> SteeringAccelerationCalculator<T> for Arrive<'a, T> {
    fn calculate_real_steering<'b>(&self,
                                   steering_acceleration: &'b mut SteeringAcceleration<T>,
                                   owner: &'b Steerable<T>)
                                   -> &'b mut SteeringAcceleration<T> {

        steering_acceleration.linear = *self.target.get_position() - *owner.get_position();

        let to_target = distance(&Point3::from_coordinates(steering_acceleration.linear),
                                 &Point3::origin());

        if to_target <= self.tolerance {
            steering_acceleration.set_zero();
        }
        let mut target_speed = match self.behavior.limiter {
            Some(lim) => lim.get_max_linear_speed(),
            None => T::one(),
        };
        if to_target <= self.deceleration_radius {
            target_speed *= (to_target / self.deceleration_radius);
        }
        steering_acceleration.linear = steering_acceleration.linear
                                                            .multiply_by(target_speed / to_target);
        steering_acceleration.linear -= *owner.get_linear_velocity();
        steering_acceleration.linear  = steering_acceleration.linear.multiply_by(T::one()/self.time_to_target);
        steering_acceleration.angular = T::zero();
        steering_acceleration
    }

    fn is_enabled(self: &Self) -> bool {
        self.behavior.enabled
    }
}
