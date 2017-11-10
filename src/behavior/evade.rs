use nalgebra::{distance_squared, Point3};
use super::super::{HasSteeringBehavior, Steerable, SteeringAcceleration,
                   SteeringAccelerationCalculator, SteeringBehavior};
use alga::general::Real;
use alga::general::AbstractModule;
use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Builder)]
pub struct Evade<T>
where
    T: Real,
{
    /// Common behavior attributes
    pub behavior: RefCell<SteeringBehavior<T>>,

    pub max_prediction_time: T,
}

impl<T: Real> HasSteeringBehavior<T> for Evade<T> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>> {
        self.behavior.borrow_mut()
    }
}

impl<T: Real> SteeringAccelerationCalculator<T> for Evade<T> {
    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow();
        let square_distance = distance_squared(
            &Point3::from_coordinates(
                *behavior.target.borrow().get_position() - *behavior.owner.borrow().get_position(),
            ),
            &Point3::origin(),
        );
        let square_speed = distance_squared(
            &Point3::from_coordinates(*behavior.owner.borrow().get_linear_velocity()),
            &Point3::origin(),
        );
        let mut prediction_time = self.max_prediction_time;
        if square_speed > T::zero() {
            let square_prediction_time = square_distance / square_speed;
            if square_prediction_time < self.max_prediction_time * self.max_prediction_time {
                prediction_time = square_prediction_time.sqrt();
            }
        }

        steering_acceleration.borrow_mut().linear = *behavior.target.borrow().get_position();
        steering_acceleration.borrow_mut().mul_add(
            SteeringAcceleration::new(*behavior.target.borrow().get_linear_velocity(), T::zero()),
            prediction_time,
        );
        steering_acceleration.borrow_mut().linear -= *behavior.owner.borrow().get_position();
        steering_acceleration.borrow_mut().linear =
            steering_acceleration.borrow().linear.normalize();
        steering_acceleration.borrow_mut().linear = steering_acceleration
            .borrow()
            .linear
            .multiply_by(match self.behavior.borrow().limiter {
                Some(ref a) => -(*a).borrow().get_max_linear_acceleration(),
                None => T::one(),
            });
        steering_acceleration.borrow_mut().angular = T::zero();
        steering_acceleration
    }
}
