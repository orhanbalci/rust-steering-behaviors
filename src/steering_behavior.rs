use limiter::Limiter;
use alga::general::Real;

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use Steerable;
/// Common properties of steering behaviors
#[builder(pattern = "immutable")]
#[derive(Builder, Clone)]
pub struct SteeringBehavior<T>
where
    T: Real,
{
    /// is this behavior enabled
    pub enabled: bool,
    /// limitations on speed and velocity calculations
    pub limiter: Option<Rc<RefCell<Limiter<T>>>>,

    pub owner: Rc<RefCell<Steerable<T>>>,

    pub target: Rc<RefCell<Steerable<T>>>,
}


pub trait HasSteeringBehavior<T: Real> {
    fn get_steering_behavior(&mut self) -> RefMut<SteeringBehavior<T>>;
}

pub trait IsEnabled<T> {
    fn is_enabled(&mut self) -> bool;
    fn set_enabled(&mut self, value: bool);
}

impl<T: Real, U> IsEnabled<T> for U
where
    U: HasSteeringBehavior<T>,
{
    fn is_enabled(&mut self) -> bool {
        self.get_steering_behavior().enabled
    }

    fn set_enabled(&mut self, value: bool) {
        self.get_steering_behavior().enabled = value;
    }
}
