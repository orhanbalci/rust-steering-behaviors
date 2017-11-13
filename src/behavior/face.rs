use super::super::{HasSteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator,
                   SteeringBehavior};
use nalgebra::{angle, distance, Point3, Vector3};
use alga::general::Real;
use std::f32::MAX;
use num_traits::identities::Zero;
use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;

#[builder(pattern = "immutable")]
#[derive(Builder)]
pub struct Face<T>
where
    T: Real,
{
    /// Common behavior attributes
    pub behavior: RefCell<SteeringBehavior<T>>,
    pub allign_tolerance: T,
    pub deceleration_radius: T,
    pub time_to_target: T,
}


impl<T: Real> HasSteeringBehavior<T> for Face<T> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>> {
        self.behavior.borrow_mut()
    }
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

impl<T: Real> Face<T> {
    fn reach_orientation(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
        target_orientation: T,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow();
        let rotation = wrap_angle_around_zero(
            target_orientation - behavior.owner.borrow().get_orientation(),
        );
        let abs_rotation = Real::abs(rotation);
        if abs_rotation <= self.allign_tolerance {
            steering_acceleration.borrow_mut().set_zero();
        }

        let mut target_rotation = match self.behavior.borrow().limiter {
            Some(ref lim) => lim.borrow().get_max_angular_speed(),
            None => T::from_f32(MAX).unwrap(),
        };
        if abs_rotation < self.deceleration_radius {
            target_rotation *= abs_rotation / self.deceleration_radius;
        }

        target_rotation *= rotation / abs_rotation;

        steering_acceleration.borrow_mut().angular =
            (target_rotation - behavior.owner.borrow().get_angular_velocity()) /
                self.time_to_target;
        let angular_acceleration = Real::abs(steering_acceleration.borrow().angular);
        if let Some(ref lim) = self.behavior.borrow().limiter {
            if angular_acceleration > lim.borrow().get_max_angular_speed() {
                steering_acceleration.borrow_mut().angular *= lim.borrow().get_max_angular_speed() /
                    angular_acceleration;
            }
        }
        steering_acceleration.borrow_mut().linear = Vector3::zero();
        steering_acceleration
    }
}

impl<T: Real> SteeringAccelerationCalculator<T> for Face<T> {
    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow();
        steering_acceleration.borrow_mut().linear = behavior.target.borrow().get_position() -
            *behavior.owner.borrow().get_position();
        let to_target = distance(
            &Point3::from_coordinates(steering_acceleration.borrow().linear),
            &Point3::origin(),
        );

        if let Some(ref lim) = self.behavior.borrow().limiter {
            if to_target.powi(2) < lim.borrow().get_zero_linear_speed_threshold() {
                steering_acceleration.borrow_mut().set_zero();
            }
        }

        let target_orientation = angle(
            &steering_acceleration.borrow().linear,
            &Vector3::new(T::zero(), T::one(), T::zero()),
        );
        self.reach_orientation(steering_acceleration, target_orientation)
    }
}
