use log::debug;
use yew::prelude::*;

#[allow(unused_macros)]
// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! log {
    ($($t:tt)*) => {
        debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}

#[derive(Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Default)]
/// struct containing the position of the a corner and the b corner
struct Position {
    a: Point,
    b: Point,
}

#[derive(Default)]
struct Vector {
    x: f64,
    y: f64,
}

pub struct Viewport {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Position {
    fn render(&self, v: &Viewport) -> Html {
        html! {
            <>
                <line x1={(self.a.x * (v.x1 - v.x0) + v.x0).to_string()} y1={(v.y1 - self.a.y * (v.y1 - v.y0)).to_string()} x2={(self.b.x * (v.x1 - v.x0) + v.x0).to_string()} y2={(v.y1 - self.b.y * (v.y1 - v.y0)).to_string()} stroke="black" stroke-width="0.01" />
            </>
        }
    }
}

#[derive(Default)]
pub struct Human {
    time: f64,
    left_foot: Point,
    /// total time elapsed
    left_calf: Vector,
    left_thigh: Vector,
    left_bicep: Vector,
    left_forearm: Vector,
    right_calf: Vector,
    right_thigh: Vector,
    right_bicep: Vector,
    right_forearm: Vector,
    torso: Vector,
    head: Vector,
}

pub enum UpdateHuman {
    LeftFootA(f64),
    LeftFootB(f64),
    LeftCalfX(f64),
    LeftCalfY(f64),
    LeftThighX(f64),
    LeftThighY(f64),
    LeftBicepX(f64),
    LeftBicepY(f64),
    LeftForearmX(f64),
    LeftForearmY(f64),
    RightCalfX(f64),
    RightCalfY(f64),
    RightThighX(f64),
    RightThighY(f64),
    RightBicepX(f64),
    RightBicepY(f64),
    RightForearmX(f64),
    RightForearmY(f64),
    TorsoX(f64),
    TorsoY(f64),
    HeadX(f64),
    HeadY(f64),
}

// Getters for the position of limbs
impl Human {
    fn get_left_calf(&self) -> Position {
        Position {
            a: Point {
                x: self.left_foot.x,
                y: self.left_foot.y,
            },
            b: Point {
                x: self.left_calf.x + self.left_foot.x,
                y: self.left_calf.y + self.left_foot.y,
            },
        }
    }
    fn get_left_thigh(&self) -> Position {
        let left_calf_pos = self.get_left_calf();
        Position {
            a: Point {
                x: left_calf_pos.b.x,
                y: left_calf_pos.b.y,
            },
            b: Point {
                x: left_calf_pos.b.x + self.left_thigh.x,
                y: left_calf_pos.b.y + self.left_thigh.y,
            },
        }
    }
    fn get_right_thigh(&self) -> Position {
        let left_thigh_pos = self.get_left_thigh();
        Position {
            a: Point {
                x: left_thigh_pos.b.x,
                y: left_thigh_pos.b.y,
            },
            b: Point {
                x: left_thigh_pos.b.x + self.right_thigh.x,
                y: left_thigh_pos.b.y + self.right_thigh.y,
            },
        }
    }
    fn get_right_calf(&self) -> Position {
        let right_thigh_pos = self.get_right_thigh();
        Position {
            a: Point {
                x: right_thigh_pos.b.x,
                y: right_thigh_pos.b.y,
            },
            b: Point {
                x: right_thigh_pos.b.x + self.right_calf.x,
                y: right_thigh_pos.b.y + self.right_calf.y,
            },
        }
    }
    fn get_torso(&self) -> Position {
        let left_thigh_pos = self.get_left_thigh();
        Position {
            a: Point {
                x: left_thigh_pos.b.x,
                y: left_thigh_pos.b.y,
            },
            b: Point {
                x: left_thigh_pos.b.x + self.torso.x,
                y: left_thigh_pos.b.y + self.torso.y,
            },
        }
    }
    fn get_left_bicep(&self) -> Position {
        let torso_pos = self.get_torso();
        Position {
            a: Point {
                x: torso_pos.b.x,
                y: torso_pos.b.y,
            },
            b: Point {
                x: torso_pos.b.x + self.left_bicep.x,
                y: torso_pos.b.y + self.left_bicep.y,
            },
        }
    }
    fn get_left_forearm(&self) -> Position {
        let left_bicep_pos = self.get_left_bicep();
        Position {
            a: Point {
                x: left_bicep_pos.b.x,
                y: left_bicep_pos.b.y,
            },
            b: Point {
                x: left_bicep_pos.b.x + self.left_forearm.x,
                y: left_bicep_pos.b.y + self.left_forearm.y,
            },
        }
    }
    fn get_right_bicep(&self) -> Position {
        let torso_pos = self.get_torso();
        Position {
            a: Point {
                x: torso_pos.b.x,
                y: torso_pos.b.y,
            },
            b: Point {
                x: torso_pos.b.x + self.right_bicep.x,
                y: torso_pos.b.y + self.right_bicep.y,
            },
        }
    }
    fn get_right_forearm(&self) -> Position {
        let right_bicep_pos = self.get_right_bicep();
        Position {
            a: Point {
                x: right_bicep_pos.b.x,
                y: right_bicep_pos.b.y,
            },
            b: Point {
                x: right_bicep_pos.b.x + self.right_forearm.x,
                y: right_bicep_pos.b.y + self.right_forearm.y,
            },
        }
    }
    fn get_head(&self) -> Position {
        let torso_pos = self.get_torso();
        Position {
            a: Point {
                x: torso_pos.b.x,
                y: torso_pos.b.y,
            },
            b: Point {
                x: torso_pos.b.x + self.head.x,
                y: torso_pos.b.y + self.head.y,
            },
        }
    }
    pub fn update_human(&mut self, update: UpdateHuman) {
        match update {
            UpdateHuman::LeftFootA(x) => self.left_foot.x = x,
            UpdateHuman::LeftFootB(y) => self.left_foot.y = y,
            UpdateHuman::LeftCalfX(x) => self.left_calf.x = x,
            UpdateHuman::LeftCalfY(y) => self.left_calf.y = y,
            UpdateHuman::LeftThighX(x) => self.left_thigh.x = x,
            UpdateHuman::LeftThighY(y) => self.left_thigh.y = y,
            UpdateHuman::LeftBicepX(x) => self.left_bicep.x = x,
            UpdateHuman::LeftBicepY(y) => self.left_bicep.y = y,
            UpdateHuman::LeftForearmX(x) => self.left_forearm.x = x,
            UpdateHuman::LeftForearmY(y) => self.left_forearm.y = y,
            UpdateHuman::RightCalfX(x) => self.right_calf.x = x,
            UpdateHuman::RightCalfY(y) => self.right_calf.y = y,
            UpdateHuman::RightThighX(x) => self.right_thigh.x = x,
            UpdateHuman::RightThighY(y) => self.right_thigh.y = y,
            UpdateHuman::RightBicepX(x) => self.right_bicep.x = x,
            UpdateHuman::RightBicepY(y) => self.right_bicep.y = y,
            UpdateHuman::RightForearmX(x) => self.right_forearm.x = x,
            UpdateHuman::RightForearmY(y) => self.right_forearm.y = y,
            UpdateHuman::TorsoX(x) => self.torso.x = x,
            UpdateHuman::TorsoY(y) => self.torso.y = y,
            UpdateHuman::HeadX(x) => self.head.x = x,
            UpdateHuman::HeadY(y) => self.head.y = y,
        }
    }

}

impl Human {
    pub fn new() -> Self {
        Human {
            left_foot: Point { x: 0.1, y: 0.1 },
            left_calf: Vector { x: 0.05, y: 0.05 },
            left_thigh: Vector { x: 0.05, y: 0.03 },
            left_bicep: Vector { x: -0.05, y: 0.05 },
            left_forearm: Vector { x: -0.05, y: -0.03 },
            right_calf: Vector { x: 0.05, y: 0.05 },
            right_thigh: Vector { x: 0.05, y: 0.03 },
            right_bicep: Vector { x: -0.05, y: 0.05 },
            right_forearm: Vector { x: -0.05, y: -0.03 },
            torso: Vector { x: 0.0, y: 0.05 },
            head: Vector { x: 0.001, y: 0.05 },
            ..Default::default()
        }
    }
    pub fn update(&mut self, time: f64) {
        self.time = time;
        // alias for the current time
        let t = self.time / 477.46482928;
        let (sin, cos) = (f64::sin, f64::cos);
        log!("{}", self.time);
        self.left_calf.x = cos(t) * 0.01;
        self.left_calf.y = sin(t) * 0.01;
    }

    pub fn view(&self, v: Viewport) -> Html {
        html! {
            <>
                {self.get_left_calf().render(&v)}
                {self.get_left_thigh().render(&v)}
                {self.get_left_forearm().render(&v)}
                {self.get_left_bicep().render(&v)}
                {self.get_right_calf().render(&v)}
                {self.get_right_thigh().render(&v)}
                {self.get_right_forearm().render(&v)}
                {self.get_right_bicep().render(&v)}
                {self.get_torso().render(&v)}
                {self.get_head().render(&v)}
            </>
        }
    }
}
