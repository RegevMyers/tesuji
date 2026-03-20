use crate::common::Color;

#[derive(Copy, Clone)]
pub struct Intersection {
    pub stone: Option<Color>,
    pub star: bool,
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection { stone: None, star: false }
    }
}
