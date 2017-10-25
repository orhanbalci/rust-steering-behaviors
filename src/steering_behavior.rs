use limiter::Limiter;
use alga::general::Real;

/// Common properties of steering behaviors
#[derive(Builder, Clone)]
pub struct SteeringBehavior<'a, T>
where
    T: 'a + Real,
{
    /// is this behavior enabled
    pub enabled: bool,
    /// limitations on speed and velocity calculations
    pub limiter: Option<&'a Limiter<T>>,
}
