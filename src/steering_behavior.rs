use steerable::Steerable;
use limiter::Limiter;
/// Common properties of steering behaviors 
pub struct SteeringBehavior<'a, T: 'a> {
    /// ownew of this behavior upon which the calculations will occur
    pub owner: &'a Steerable<T>,
    /// is this behavior enabled
    pub enabled: bool,
    /// limitations on speed and velocity calculations
    pub limiter: &'a Limiter,
}
