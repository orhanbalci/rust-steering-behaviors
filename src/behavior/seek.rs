use super::super::SteeringBehavior;
use super::super::{SteeringAcceleration, SteeringAccelerationCalculator};
use alga::general::Real;
use alga::general::AbstractModule;
use Steerable;

/// Seek behavior calculates the maximum linear valocity to reach the target location
#[derive(Builder)]
pub struct Seek<'a, T>
where
    T: 'a + Real,
{
    /// common steering behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// steering target
    pub target: &'a Steerable<T>,
}

impl<'a, T: Real> SteeringAccelerationCalculator<T> for Seek<'a, T> {
    fn calculate_real_steering<'b>(
        &self,
        steering_acceleration: &'b mut SteeringAcceleration<T>,
        owner: &'b Steerable<T>,
    ) -> &'b mut SteeringAcceleration<T> {
        steering_acceleration.linear = (*self.target.get_position() - *owner.get_position())
            .normalize()
            .multiply_by(match self.behavior.limiter {
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

#[cfg(test)]
mod test {
    use super::Seek;
    use super::super::test_common::TestSteerable;
    use super::super::super::Steerable;
    use super::SteeringBehavior;
    use super::SteeringAccelerationCalculator;
    use super::SteeringAcceleration;
    use nalgebra::Vector3;

    #[test]
    fn test_same_location() {
        let mut test_target = TestSteerable::new();
        let mut test_owner = TestSteerable::new();

        let test_behavior = Seek {
            behavior: SteeringBehavior {
                enabled: true,
                limiter: None,
            },
            target: &test_target,
        };

        let mut sa = SteeringAcceleration::default();

        let acceleration_result = test_behavior.calculate_steering(&mut sa, &test_owner);
        // assert_eq!(Vector3::new(0.0f32,0.0,0.0), acceleration_result.linear);
        assert_eq!(0.0f32, acceleration_result.angular);
    }

    #[test]
    fn test_one_dimension() {
        let mut test_target = TestSteerable::new();
        let mut test_owner = TestSteerable::new();

        test_target.set_position(Vector3::new(1.0f32, 0.0, 0.0));
        test_owner.set_position(Vector3::new(0.0f32, 0.0, 0.0));

        let test_behavior = Seek {
            behavior: SteeringBehavior {
                enabled: true,
                limiter: None,
            },
            target: &test_target,
        };

        let mut sa = SteeringAcceleration::default();

        let acceleration_result = test_behavior.calculate_steering(&mut sa, &test_owner);
        assert_eq!(Vector3::new(1.0f32, 0.0, 0.0), acceleration_result.linear);
        assert_eq!(0.0f32, acceleration_result.angular);
    }
}
