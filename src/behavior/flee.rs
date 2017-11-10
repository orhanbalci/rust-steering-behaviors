use super::super::{HasSteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator,
                   SteeringBehavior};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;
/// This behavior is the oposite of Seek behavior. It produces linear steering acceleration
/// to go away from target
#[derive(Builder)]
pub struct Flee<T>
where
    T: Real,
{
    /// Common behavior attributes
    pub behavior: RefCell<SteeringBehavior<T>>,
}


impl<T: Real> HasSteeringBehavior<T> for Flee<T> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>> {
        self.behavior.borrow_mut()
    }
}

impl<T: Real> SteeringAccelerationCalculator<T> for Flee<T> {
    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow();
        steering_acceleration.borrow_mut().linear = (*behavior.owner.borrow().get_position()
            - *behavior.target.borrow().get_position())
            .normalize()
            .multiply_by(match self.behavior.borrow().limiter {
                Some(ref l) => (*l).borrow().get_max_linear_acceleration(),
                None => T::one(),
            });

        steering_acceleration.borrow_mut().angular = T::zero();
        steering_acceleration
    }
}
