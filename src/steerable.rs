use nalgebra::Vector3;

/// Steerable agent interface
pub trait Steerable<T> {
    /// returns the linear velocity vector of the agent
    fn get_linear_velocity(self: &Self) -> &Vector3<T>;

    /// returns angular velocity of the agent
    fn get_angular_velocity(self: &Self) -> T;

    /// returns bounding circle radius of the agent
    fn get_bounding_radius(self: &Self) -> T;
}
