use steerable::Steerable;
/// 
pub struct SteeringBehavior<'a, T: 'a> {
    pub owner: &'a Steerable<T>,
    pub enabled: bool,
}
