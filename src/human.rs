use log::debug;
use yew::prelude::*;

#[allow(unused_macros)]
// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! log {
    ($($t:tt)*) => {
        debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

// Update point selectively
impl Point {
    fn update(&mut self, x: Option<f64>, y: Option<f64>) {
        self.x = x.unwrap_or(self.x);
        self.y = y.unwrap_or(self.y);
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
    pub left_foot: Point,
    pub left_knee: Point,
    pub hip: Point,
    pub right_knee: Point,
    pub right_foot: Point,
    pub neck: Point,
    pub left_elbow: Point,
    pub left_hand: Point,
    pub right_elbow: Point,
    pub right_hand: Point,
    pub head: Point,
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
    fn get_left_calf(&self) -> Position {
        Position {
            a: self.left_foot,
            b: self.left_knee,
        }
    }
    fn get_left_thigh(&self) -> Position {
        Position {
            a: self.left_knee,
            b: self.hip,
        }
    }
    fn get_right_thigh(&self) -> Position {
        Position {
            a: self.hip,
            b: self.right_knee,
        }
    }
    fn get_right_calf(&self) -> Position {
        Position {
            a: self.right_knee,
            b: self.right_foot,
        }
    }
    fn get_torso(&self) -> Position {
        Position {
            a: self.hip,
            b: self.neck,
        }
    }
    fn get_left_bicep(&self) -> Position {
        Position {
            a: self.neck,
            b: self.left_elbow,
        }
    }
    fn get_left_forearm(&self) -> Position {
        Position {
            a: self.left_elbow,
            b: self.left_hand,
        }
    }
    fn get_right_bicep(&self) -> Position {
        Position {
            a: self.neck,
            b: self.right_elbow,
        }
    }
    fn get_right_forearm(&self) -> Position {
        Position {
            a: self.right_elbow,
            b: self.right_hand,
        }
    }
    fn get_head(&self) -> Position {
        Position {
            a: self.neck,
            b: self.head,
        }
    }
    pub fn update_human(&mut self, update: UpdateHuman) {
        match update {
            UpdateHuman::LeftFoot(x, y) => self.left_foot.update(x, y),
            UpdateHuman::LeftKnee(x, y) => self.left_knee.update(x, y),
            UpdateHuman::Hip(x, y) => self.hip.update(x, y),
            UpdateHuman::RightKnee(x, y) => self.right_knee.update(x, y),
            UpdateHuman::RightFoot(x, y) => self.right_foot.update(x, y),
            UpdateHuman::Neck(x, y) => self.neck.update(x, y),
            UpdateHuman::LeftElbow(x, y) => self.left_elbow.update(x, y),
            UpdateHuman::LeftHand(x, y) => self.left_hand.update(x, y),
            UpdateHuman::RightElbow(x, y) => self.right_elbow.update(x, y),
            UpdateHuman::RightHand(x, y) => self.right_hand.update(x, y),
            UpdateHuman::Head(x, y) => self.head.update(x, y),
        }
    }
}

impl Human {
    pub fn new() -> Self {
        Human {
            time: 0.0,
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
        }
    }
    pub fn update(&mut self, time: f64) {
        self.time = time;
        // alias for the current time
        // let t = self.time / 477.46482928;
        // let (sin, cos) = (f64::sin, f64::cos);
        log!("{}", self.time);
        // self.left_calf.x = cos(t) * 0.01;
        // self.left_calf.y = sin(t) * 0.01;
    }

    pub fn view(&self) -> Html {
        html! {
            <>
                {self.get_left_calf().render()}
                {self.get_left_thigh().render()}
                {self.get_left_forearm().render()}
                {self.get_left_bicep().render()}
                {self.get_right_calf().render()}
                {self.get_right_thigh().render()}
                {self.get_right_forearm().render()}
                {self.get_right_bicep().render()}
                {self.get_torso().render()}
                {self.get_head().render()}
            </>
        }
    }
}
