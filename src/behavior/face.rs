use super::super::{Limiter, SteeringAcceleration, SteeringAccelerationCalculator, SteeringBehavior};
use nalgebra::{angle, distance, Point3, Vector3};
use alga::general::Real;
use alga::general::AbstractModule;
use std::f32::MAX;
use num_traits::identities::Zero;
use Steerable;

#[derive(Builder)]
pub struct Face<'a, T>
where
    T: 'a + Real,
{
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    pub target: &'a Steerable<T>,
    pub allign_tolerance: T,
    pub deceleration_radius: T,
    pub time_to_target: T,
}

fn wrap_angle_around_zero<T: Real>(inp: T) -> T {
    if inp >= T::zero() {
        let mut rotation = inp % T::two_pi();
        if rotation > T::pi() {
            rotation -= T::two_pi();
        }
        rotation
    } else {
        let mut rotation = -inp % T::two_pi();
        if rotation > T::pi() {
            rotation -= T::two_pi();
        }
        -rotation
    }
}

impl<'a, T: 'a + Real> Face<'a, T> {
    fn reach_orientation<'b>(
        &self,
        steering_acceleration: &'b mut SteeringAcceleration<T>,
        owner: &'b Steerable<T>,
        target_orientation: T,
    ) -> &'b mut SteeringAcceleration<T> {
        let rotation = wrap_angle_around_zero(target_orientation - owner.get_orientation());
        let abs_rotation = Real::abs(rotation);
        if abs_rotation <= self.allign_tolerance {
            steering_acceleration.set_zero();
        }

        let mut target_rotation = match self.behavior.limiter {
            Some(lim) => lim.get_max_angular_speed(),
            None => T::from_f32(MAX).unwrap(),
        };
        if abs_rotation < self.deceleration_radius {
            target_rotation *= abs_rotation / self.deceleration_radius;
        }

        target_rotation *= rotation / abs_rotation;

        steering_acceleration.angular =
            (target_rotation - owner.get_angular_velocity()) / self.time_to_target;
        let angular_acceleration = Real::abs(steering_acceleration.angular);
        if let Some(lim) = self.behavior.limiter {
            if angular_acceleration > lim.get_max_angular_speed() {
                steering_acceleration.angular *= lim.get_max_angular_speed() / angular_acceleration;
            }
        }
        steering_acceleration.linear = Vector3::zero();
        steering_acceleration
    }
}

impl<'a, T: 'a + Real> SteeringAccelerationCalculator<T> for Face<'a, T> {
    fn calculate_real_steering<'b>(
        &self,
        steering_acceleration: &'b mut SteeringAcceleration<T>,
        owner: &'b Steerable<T>,
    ) -> &'b mut SteeringAcceleration<T> {
        steering_acceleration.linear = *self.target.get_position() - *owner.get_position();
        let to_target = distance(
            &Point3::from_coordinates(steering_acceleration.linear),
            &Point3::origin(),
        );

        if let Some(lim) = self.behavior.limiter {
            if to_target.powi(2) < lim.get_zero_linear_speed_threshold() {
                steering_acceleration.set_zero();
            }
        }

        let target_orientation = angle(
            &steering_acceleration.linear,
            &Vector3::new(T::zero(), T::one(), T::zero()),
        );
        self.reach_orientation(steering_acceleration, owner, target_orientation)
    }

    fn is_enabled(self: &Self) -> bool {
        self.behavior.enabled
    }
}
