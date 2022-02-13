use crate::animation;
use yew::prelude::*;

#[allow(unused_imports)]
use crate::trace;

pub struct Background {
    offset: f64,
    total_offset: f64,
    href: String,
}

// rendering a Position
impl Background {
    pub fn new() -> Self {
        Background {
            offset: 0.0,
            total_offset: 0.0,
            href: "./public/background.png".to_string(),
        }
    }
    pub fn view(&self) -> Html {
        html! {
            <>
                <image x={self.offset.to_string()} href={self.href.clone()} height="1"/>
            </>
        }
    }
    pub fn apply_offset(&mut self, joints: &animation::AnimationPosition) {
        /// offset is how much the background will move to the right 
        /// to match the position of the human
        let offset = joints.hip;
    }
}
