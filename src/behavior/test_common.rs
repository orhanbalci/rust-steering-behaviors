#[cfg(test)]
use super::super::{Steerable, SteeringBehavior, SteeringAcceleration,
                   SteeringAccelerationCalculator};
use nalgebra::Vector3;
use alga::general::AbstractModule;

pub struct TestSteerable {
    linear_velocity: Vector3<f32>,
    position: Vector3<f32>,
    angular_velocity: f32,
    bounding_radius: f32,
}

impl Steerable<f32> for TestSteerable {
    fn get_linear_velocity(&self) -> &Vector3<f32> {
        &self.linear_velocity
    }

    fn get_angular_velocity(&self) -> f32 {
        self.angular_velocity
    }

    fn get_bounding_radius(&self) -> f32 {
        self.bounding_radius
    }

    fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }
}

impl TestSteerable {
    pub fn new() -> Self {
        TestSteerable {
            linear_velocity: Vector3::new(1.0, 0.0, 0.0),
            position: Vector3::new(-50.0, 50.0, 0.0),
            angular_velocity: 0.0,
            bounding_radius: 2.0,
        }
    }

    pub fn advance(&mut self, calc: &SteeringAccelerationCalculator<f32>, milis: f32) {
        let mut sa = SteeringAcceleration::default();
        calc.calculate_steering(&mut sa, self);
        self.linear_velocity += sa.linear;
        self.angular_velocity += sa.angular;
        self.position = self.position + self.linear_velocity.multiply_by(milis / 1000.0);
    }

    pub fn advance_by_velocity(&mut self, milis: f32) {
        self.position += self.linear_velocity.multiply_by(milis / 1000.0);
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }
}
