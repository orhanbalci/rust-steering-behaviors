use nalgebra::Vector3;
use std::cmp::PartialEq;
use num::num_traits::Num;
use num::Zero;

/// Represents result of a steering behaviour computation. User can aggregate
/// more than one behaviour result into single acceleration struct.
pub struct SteeringAcceleration<T: Num>{
    pub linear : Vector3<T>,
    pub angular : T,
}


impl<T: Num> SteeringAcceleration<T>{
    fn new(linear_acceleration : Vector3<T>, angular_acceleration : T) -> SteeringAcceleration<T>{
        SteeringAcceleration{
            linear : linear_acceleration,
            angular : angular_acceleration,
        }
    }
    
    fn is_zero(self : &Self) -> bool{
        self.angular.is_zero()  && self.linear.is_zero()
    }
}
