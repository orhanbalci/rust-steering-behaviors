use steerable::Steerable;
use limiter::Limiter;
use alga::general::Real;

/// Common properties of steering behaviors 
#[derive(Builder, Clone)]
pub struct SteeringBehavior<'a, T> where T: 'a + Real{
    /// ownew of this behavior upon which the calculations will occur
    pub owner: &'a Steerable<T>,
    /// is this behavior enabled
    pub enabled: bool,
    /// limitations on speed and velocity calculations
    pub limiter: Option<&'a Limiter<T>>,
}
