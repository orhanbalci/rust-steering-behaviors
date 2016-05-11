use steerable::Steerable;
use limiter::Limiter;
/// 
pub struct SteeringBehavior<'a, T: 'a> {
    pub owner: &'a Steerable<T>,
    pub enabled: bool,
    pub limiter: &'a Limiter,
}
