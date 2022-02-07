use crate::{animation, trace};
use yew::prelude::*;

lazy_static! {
    static ref ANIMATIONS_DATA: animation::AnimationsData =
        animation::AnimationsData::load_animations();
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Update point selectively
    fn update(&mut self, x: Option<f64>, y: Option<f64>) {
        self.x = x.unwrap_or(self.x);
        self.y = y.unwrap_or(self.y);
    }
    /// Linear interpolation between two points
    pub fn lerp(&self, next: &Point, offset: f64) -> Self {
        Point {
            x: self.x + (next.x - self.x) * offset,
            y: self.y + (next.y - self.y) * offset,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// struct containing the position of the a corner and the b corner
struct Position {
    a: Point,
    b: Point,
}

pub struct Viewport {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

// rendering a Position
impl Position {
    fn render(&self) -> Html {
        html! {
            <>
                <line x1={(self.a.x).to_string()} y1={(self.a.y).to_string()} x2={(self.b.x).to_string()} y2={(self.b.y).to_string()} stroke="black" stroke-width="0.01" />
            </>
        }
    }
}

pub struct Human {
    time: f64,
    pub joints: animation::AnimationPosition,
    current_animation: Option<(&'static animation::Animation, f64)>,
}

pub enum UpdateHuman {
    LeftFoot(Option<f64>, Option<f64>),
    LeftKnee(Option<f64>, Option<f64>),
    Hip(Option<f64>, Option<f64>),
    RightKnee(Option<f64>, Option<f64>),
    RightFoot(Option<f64>, Option<f64>),
    Neck(Option<f64>, Option<f64>),
    LeftElbow(Option<f64>, Option<f64>),
    LeftHand(Option<f64>, Option<f64>),
    RightElbow(Option<f64>, Option<f64>),
    RightHand(Option<f64>, Option<f64>),
    Head(Option<f64>, Option<f64>),
}

// Getters for the position of limbs
impl Human {
    pub fn update_human(&mut self, update: UpdateHuman) {
        let joints = &mut self.joints;
        match update {
            UpdateHuman::LeftFoot(x, y) => joints.left_foot.update(x, y),
            UpdateHuman::LeftKnee(x, y) => joints.left_knee.update(x, y),
            UpdateHuman::Hip(x, y) => joints.hip.update(x, y),
            UpdateHuman::RightKnee(x, y) => joints.right_knee.update(x, y),
            UpdateHuman::RightFoot(x, y) => joints.right_foot.update(x, y),
            UpdateHuman::Neck(x, y) => joints.neck.update(x, y),
            UpdateHuman::LeftElbow(x, y) => joints.left_elbow.update(x, y),
            UpdateHuman::LeftHand(x, y) => joints.left_hand.update(x, y),
            UpdateHuman::RightElbow(x, y) => joints.right_elbow.update(x, y),
            UpdateHuman::RightHand(x, y) => joints.right_hand.update(x, y),
            UpdateHuman::Head(x, y) => joints.head.update(x, y),
        }
    }
}

impl Human {
    pub fn new() -> Self {
        Human {
            time: 0.0,
            joints: animation::AnimationPosition {
                left_foot: Point { x: 0.15, y: 0.9 },
                left_knee: Point { x: 0.225, y: 0.85 },
                hip: Point { x: 0.3, y: 0.82 },
                right_knee: Point { x: 0.375, y: 0.77 },
                right_foot: Point { x: 0.45, y: 0.74 },
                neck: Point { x: 0.3, y: 0.77 },
                left_elbow: Point { x: 0.225, y: 0.72 },
                left_hand: Point { x: 0.15, y: 0.74 },
                right_elbow: Point { x: 0.375, y: 0.72 },
                right_hand: Point { x: 0.45, y: 0.72 },
                head: Point { x: 0.315, y: 0.72 },
            },
            current_animation: None,
        }
    }
    pub fn update(&mut self, time: f64) {
        if let Some((animation, start_time)) = &self.current_animation {
            trace!("hi");
            if let Some(joints) = animation.step(time - start_time) {
                self.joints = joints
            } else {
                self.current_animation = None;
            }
        } else {
            self.current_animation = Some((&ANIMATIONS_DATA.walking, time));
        }
        trace!("{}", self.time);
    }

    pub fn view(&self) -> Html {
        let to_pos = |a, b| Position { a, b };
        let joints = &self.joints;
        let (
            left_calf,
            left_thigh,
            right_thigh,
            right_calf,
            torso,
            left_bicep,
            left_forearm,
            right_bicep,
            right_forearm,
            head,
        ) = (
            to_pos(joints.left_foot, joints.left_knee),
            to_pos(joints.left_knee, joints.hip),
            to_pos(joints.hip, joints.right_knee),
            to_pos(joints.right_knee, joints.right_foot),
            to_pos(joints.hip, joints.neck),
            to_pos(joints.neck, joints.left_elbow),
            to_pos(joints.left_elbow, joints.left_hand),
            to_pos(joints.neck, joints.right_elbow),
            to_pos(joints.right_elbow, joints.right_hand),
            to_pos(joints.neck, joints.head),
        );

        html! {
            <>
                {left_calf.render()}
                {left_thigh.render()}
                {left_forearm.render()}
                {left_bicep.render()}
                {right_calf.render()}
                {right_thigh.render()}
                {right_forearm.render()}
                {right_bicep.render()}
                {torso.render()}
                {head.render()}
            </>
        }
    }
}
