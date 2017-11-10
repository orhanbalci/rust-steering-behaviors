use super::super::SteeringBehavior;
use super::super::HasSteeringBehavior;
use super::super::{SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

use std::cell::RefMut;
use std::cell::RefCell;
use std::rc::Rc;

/// Seek behavior calculates the maximum linear valocity to reach the target location
#[derive(Builder)]
pub struct Seek<T>
where
    T: Real,
{
    /// common steering behavior attributes
    pub behavior: RefCell<SteeringBehavior<T>>,
}

impl<T: Real> HasSteeringBehavior<T> for Seek<T> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>> {
        self.behavior.borrow_mut()
    }
}

impl<T: Real> SteeringAccelerationCalculator<T> for Seek<T> {
    fn calculate_real_steering(
        &self,
        steering_acceleration: Rc<RefCell<SteeringAcceleration<T>>>,
    ) -> Rc<RefCell<SteeringAcceleration<T>>> {
        let behavior = self.behavior.borrow().clone();
        let position_diff =
            behavior.target.borrow().get_position() - *behavior.owner.borrow().get_position();
        steering_acceleration.borrow_mut().linear = position_diff.normalize().multiply_by(
            match self.behavior.borrow().limiter {
                Some(ref a) => (*a).borrow().get_max_linear_acceleration(),
                None => T::one(),
            },
        );
        steering_acceleration.borrow_mut().angular = T::zero();
        steering_acceleration
    }
}

#[cfg(test)]
mod test {
    use super::Seek;
    use super::super::test_common::TestSteerable;
    use super::super::super::Steerable;
    use super::SteeringBehavior;
    use super::SteeringAccelerationCalculator;
    use super::SteeringAcceleration;
    use nalgebra::Vector3;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_same_location() {
        let mut test_target = TestSteerable::new();
        let mut test_owner = TestSteerable::new();

        let mut test_behavior = Seek {
            behavior: RefCell::new(SteeringBehavior {
                enabled: true,
                limiter: None,

                target: Rc::new(RefCell::new(test_target)),
                owner: Rc::new(RefCell::new(test_owner)),
            }),
        };

        let mut sa = Rc::new(RefCell::new(SteeringAcceleration::default()));

        let acceleration_result = test_behavior.calculate_steering(sa);
        // assert_eq!(Vector3::new(0.0f32,0.0,0.0), acceleration_result.linear);
        assert_eq!(0.0f32, acceleration_result.borrow().angular);
    }

    #[test]
    fn test_one_dimension() {
        let mut test_target = TestSteerable::new();
        let mut test_owner = TestSteerable::new();

        test_target.set_position(Vector3::new(1.0f32, 0.0, 0.0));
        test_owner.set_position(Vector3::new(0.0f32, 0.0, 0.0));

        let mut test_behavior = Seek {
            behavior: RefCell::new(SteeringBehavior {
                enabled: true,
                limiter: None,
                target: Rc::new(RefCell::new(test_target)),
                owner: Rc::new(RefCell::new(test_owner)),
            }),
        };

        let mut sa = Rc::new(RefCell::new(SteeringAcceleration::default()));

        let acceleration_result = test_behavior.calculate_steering(sa);
        assert_eq!(
            Vector3::new(1.0f32, 0.0, 0.0),
            acceleration_result.borrow().linear
        );
        assert_eq!(0.0f32, acceleration_result.borrow().angular);
    }
}
