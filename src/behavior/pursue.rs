use nalgebra::{Vector3, BaseFloat, ApproxEq};
use super::super::{SteeringBehavior, SteeringAcceleration, SteeringAccelerationCalculator};

pub struct Pursue<'a, T: 'a + BaseFloat + ApproxEq<T>> {
    /// Common behavior attributes
    pub behavior: SteeringBehavior<'a, T>,
    /// Target to pursue
    pub target: Vector3<T>,

    pub max_prediction_time: T,
}


// impl<'a, T: 'a + BaseFloat + ApproxEq<T>> SteeringAccelerationCalculator<T> for Pursue<'a, T> {
// fn calculate_real_steering<'b>(&mut self,
//
// steering_acceleration: &'b mut SteeringAcceleration<T>)
// -> &'b mut SteeringAcceleration<T> {
//
// }
//
// fn is_enabled(&self) -> bool {
// self.behavior.enabled
// }
// }
