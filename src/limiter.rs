
/// Interface to set limits  on linear and angular speed and acceleration of the agent
pub trait Limiter {
    fn get_zero_linear_speed_threshold(self : &Self) -> f32;
    fn set_zero_linear_speed_threshold(self : &mut Self, threshold : f32);
    fn get_max_linear_sppeed(self: &Self) -> f32;
    fn set_max_linear_speed(self: &mut Self, linear_speed : f32);
    fn get_max_linear_acceleration(self: &Self) -> f32;
    fn set_max_linear_acceleration(self: &mut Self, linear_acceleration:f32);
    fn get_max_angular_speed(self: &Self) -> f32;
    fn set_max_angular_speed(self: &mut Self, angular_speed : f32);
    fn get_max_angular_acceleration(self : &Self) -> f32;
    fn set_max_angular_acceleration(self : &mut Self, angular_acceleration : f32);
}
