use crate::animation;
use yew::prelude::*;

#[allow(unused_imports)]
use crate::trace;

lazy_static! {
    /// Static variable that contains the loaded animations
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
    pub fn apply_offset_by(&self, initial_hip_coords: Point) -> Self {
        Point {
            x: self.x + initial_hip_coords.x,
            y: self.y + initial_hip_coords.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// struct containing the position of the a corner and the b corner
struct Position {
    a: Point,
    b: Point,
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

pub struct Viewport {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

pub struct Human {
    pub joints: animation::AnimationPosition,
    /// The current animation and its start time
    current_animation: Option<(&'static animation::Animation, f64, Point)>,
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
            joints: animation::AnimationPosition {
                left_foot: Point {
                    x: 0.6159340111255642,
                    y: 0.8270827460289001,
                },
                left_knee: Point {
                    x: 0.5924965111255642,
                    y: 0.6939577984809875,
                },
                hip: Point {
                    x: 0.6106285484790805,
                    y: 0.590322980928422,
                },
                right_knee: Point {
                    x: 0.6083327460289001,
                    y: 0.7036452221870422,
                },
                right_foot: Point {
                    x: 0.6279347174167649,
                    y: 0.846452502393722,
                },
                neck: Point {
                    x: 0.5676065373420718,
                    y: 0.36968227741718246,
                },
                left_elbow: Point {
                    x: 0.5385421784877777,
                    y: 0.49030727663040113,
                },
                left_hand: Point {
                    x: 0.5525000095367432,
                    y: 0.5759375691413879,
                },
                right_elbow: Point {
                    x: 0.6885385552883145,
                    y: 0.4378142968893053,
                },
                right_hand: Point {
                    x: 0.6315625309944153,
                    y: 0.45968756079673767,
                },
                head: Point {
                    x: 0.5425034793376926,
                    y: 0.2504119847059246,
                },
            },
            current_animation: None,
        }
    }
    pub fn update(&mut self, time: f64) {
        if let Some((animation, start_time, offset)) = &self.current_animation {
            if let Some(joints) = animation.step(time - start_time) {
                self.joints = joints.apply_offset_by(*offset);
            } else {
                self.current_animation = None;
            }
        } else {
            let animation = &ANIMATIONS_DATA.walking;
            if let Some(first_pos) = animation.positions.get(0) {
                let offset = Point { x: self.joints.hip.x - first_pos.hip.x, y: self.joints.hip.y - first_pos.hip.y }; 
                self.current_animation = Some((
                    animation,
                    time,
                    offset
                ));
            }
        }
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
