//! This library implements real life like behaviors for your autonomous in game agents
//! Implement Steerable trait for your agent, choose a behaviour, calculate steering and apply
//! calculated acceleration to your agents velocity.
extern crate alga;
#[macro_use]
extern crate derive_builder;
extern crate nalgebra;
extern crate num_traits;

pub use self::steerable::Steerable;
pub use self::steering_behavior::SteeringBehavior;
pub use self::steering_behavior::HasSteeringBehavior;
pub use self::steering_behavior::IsEnabled;
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
mod limiters;
