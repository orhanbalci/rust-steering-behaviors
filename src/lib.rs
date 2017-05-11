extern crate nalgebra;
extern crate alga;
extern crate num_traits;
#[macro_use]
extern crate derive_builder;

pub use self::steerable::Steerable;
pub use self::steering_behavior::SteeringBehavior;
pub use self::limiter::Limiter;
pub use self::steering_acceleration::SteeringAcceleration;
pub use self::steering_acceleration::SteeringAccelerationCalculator;
pub use behavior::Seek;
pub use behavior::Flee;
pub use behavior::Pursue;
pub use behavior::Arrive;
pub use behavior::Evade;

mod steerable;
mod steering_behavior;
mod limiter;
mod steering_acceleration;
mod behavior;
