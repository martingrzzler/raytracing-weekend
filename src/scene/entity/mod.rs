use hit::Hit;

pub mod hit;
pub mod sphere;

pub type Entity = Box<dyn Hit>;
