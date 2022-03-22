pub mod house_light;
pub mod traffic_light;

pub trait Light {
    fn get_name(&self) -> &str;
    fn get_state(&self) -> &dyn std::fmt::Debug;
}