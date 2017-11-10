use super::super::{HasSteeringBehavior, Limiter, SteeringAcceleration,
                   SteeringAccelerationCalculator, SteeringBehavior};
use nalgebra::{distance, Point3};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;
use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;

/// This behavior is the oposite of Seek behavior. It produces linear steering acceleration
/// to go away from target
#[derive(Builder)]
pub struct Arrive<T>
where
    T: Real,
{
    /// Common behavior attributes
    pub behavior: RefCell<SteeringBehavior<T>>,
    /// Target to go away from
    pub tolerance: T,
    pub deceleration_radius: T,
    pub time_to_target: T,
}


impl<T: Real> HasSteeringBehavior<T> for Arrive<T> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>> {
        self.behavior.borrow_mut()
    }
}

impl<T: Real> SteeringAccelerationCalculator<T> for Arrive<T> {
    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow();
        steering_acceleration.borrow_mut().linear =
            *behavior.target.borrow().get_position() - *behavior.owner.borrow().get_position();
        let to_target = distance(
            &Point3::from_coordinates(steering_acceleration.borrow().linear),
            &Point3::origin(),
        );

        if to_target <= self.tolerance {
            steering_acceleration.borrow_mut().set_zero();
        }
        let mut target_speed = match self.behavior.borrow().limiter {
            Some(ref lim) => lim.borrow().get_max_linear_speed(),
            None => T::one(),
        };
        if to_target <= self.deceleration_radius {
            target_speed *= to_target / self.deceleration_radius;
        }
        steering_acceleration.borrow_mut().linear = steering_acceleration
            .borrow()
            .linear
            .multiply_by(target_speed / to_target);
        steering_acceleration.borrow_mut().linear -= *behavior.owner.borrow().get_linear_velocity();
        steering_acceleration.borrow_mut().linear = steering_acceleration
            .borrow()
            .linear
            .multiply_by(T::one() / self.time_to_target);
        steering_acceleration.borrow_mut().angular = T::zero();
        steering_acceleration
    }
}
