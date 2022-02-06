use crate::human;
use log::{debug, Level};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! log {
    ($($t:tt)*) => {
        debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}

pub enum TesterMsg {
    Update(human::UpdateHuman),
    ClickedJoint(Joints),
    MouseUp,
    MouseMove(MouseEvent),
    Ignore,
}

pub struct Tester {
    human: human::Human,
    current_joint: Option<Joints>,
}

enum Joints {
    LeftFoot,
    LeftKnee,
    Hip,
    RightKnee,
    RightFoot,
    Neck,
    LeftElbow,
    LeftHand,
    RightElbow,
    RightHand,
    Head,
}

impl Component for Tester {
    type Message = TesterMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        log!("create");
        Self {
            human: human::Human::new(),
            current_joint: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, tester_msg: Self::Message) -> bool {
        match tester_msg {
            TesterMsg::Update(update) => {
                self.human.update_human(update);
                true
            }
            TesterMsg::ClickedJoint(joint) => {
                self.current_joint = Some(joint);
                false
            }
            TesterMsg::MouseUp => {
                self.current_joint = None;
                false
            }
            TesterMsg::MouseMove(e) => {
                // if let Some(joint) = self.current_joint {
                    // let mouse_pos = (e.client_x(), e.client_y());
                    // let window = web_sys::window().expect("no global `window` exists");
                    // let document = window.document().expect("should have a document on window");
                    // let transform_matrix = document
                    //     .get_element_by_id("svg")
                    //     .unwrap()
                    //     .dyn_into::<web_sys::SvgGraphicsElement>()
                    //     .expect("should be an svg element")
                    //     .get_screen_ctm()
                    //     .expect("should have a transform matrix")
                    //     .inverse()
                    //     .expect("should have an inverse matrix");
                    // let mp = (mouse_pos.0 as f32, mouse_pos.1 as f32);
                    // let tm = transform_matrix;
                    // let new_pos = (
                    //     tm.a() * mp.0 + tm.c() * mp.1 + tm.e(),
                    //     tm.b() * mp.0 + tm.d() * mp.1 + tm.f(),
                    // );
                    // let original_pos = match joint {
                    //     Joints::LeftFoot => self.human.left_foot,
                    //     Joints::LeftKnee => self.human.get_left_calf().b,
                    //     Joints::Hip => self.human.get_left_thigh().b,
                    //     Joints::RightKnee => self.human.right_thigh().b,
                    //     Joints::RightFoot => self.human.right_calf().b,
                    //     Joints::Neck => self.human.get_torso().b,
                    //     Joints::LeftElbow => self.human.get_left_forearm().b,
                    //     Joints::LeftHand => self.human.get_left_bicep().b,
                    //     Joints::RightElbow => self.human.get_right_forearm().b,
                    //     Joints::RightHand => self.human.get_right_bicep().b,
                    //     Joints::Head => self.human.get_head().b,
                    // };
                    // let diff = (new_pos.0 - original_pos.0, new_pos.1 - original_pos.1);
                    // match joint {
                    //     Joints::LeftFoot => {
                    //         self.human.left_foot = (
                    //             self.human.left_foot.x + diff.0,
                    //             self.human.left_foot.y + diff.1,
                    //         );
                    //     }
                    //     Joints::LeftKnee => {
                    //         self.human.get_left_calf_mut().b = (
                    //             self.human.get_left_calf().b.x + diff.0,
                    //             self.human.get_left_calf().b.y + diff.1,
                    //         );
                    //     }
                    // }
                    // let joint_pos = joint_pos.add(Vector3::new(mouse_pos.x(), mouse_pos.y(), 0.0));
                    // self.human.set_joint_pos(joint, joint_pos);
                // }
                false
            }
            TesterMsg::Ignore => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let viewport = human::Viewport {
            x0: 0.0,
            y0: 0.0,
            x1: 1.5,
            y1: 1.0,
        };
        use human::UpdateHuman;
        fn cvt(e: InputEvent) -> f64 {
            let a: f64 = e
                .target_unchecked_into::<HtmlInputElement>()
                .value_as_number();
            if !a.is_nan() {
                a
            } else {
                0.0
            }
        }
        let (
            left_foot_a,
            left_foot_b,
            left_calf_x,
            left_calf_y,
            left_thigh_x,
            left_thigh_y,
            left_bicep_x,
            left_bicep_y,
            left_forearm_x,
            left_forearm_y,
            right_calf_x,
            right_calf_y,
            right_thigh_x,
            right_thigh_y,
            right_bicep_x,
            right_bicep_y,
            right_forearm_x,
            right_forearm_y,
            torso_x,
            torso_y,
            head_x,
            head_y,
        ) = (
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftFootA(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftFootB(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftCalfX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftCalfY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftThighX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftThighY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftBicepX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftBicepY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftForearmX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::LeftForearmY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightCalfX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightCalfY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightThighX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightThighY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightBicepX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightBicepY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightForearmX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::RightForearmY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::TorsoX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::TorsoY(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::HeadX(cvt(e)))),
            link.callback(|e| TesterMsg::Update(UpdateHuman::HeadY(cvt(e)))),
        );

        html! {
            <div>
                <label>{"left_foot_a"}</label>
                <input oninput={left_foot_a} type="number"/>
                <label>{"left_foot_b"}</label>
                <input oninput={left_foot_b} type="number"/>
                <br/>
                <label>{"left_calf_x"}</label>
                <input oninput={left_calf_x} type="number"/>
                <label>{"left_calf_y"}</label>
                <input oninput={left_calf_y} type="number"/>
                <br/>
                <label>{"left_thigh_x"}</label>
                <input oninput={left_thigh_x} type="number"/>
                <label>{"left_thigh_y"}</label>
                <input oninput={left_thigh_y} type="number"/>
                <br/>
                <label>{"left_bicep_x"}</label>
                <input oninput={left_bicep_x} type="number"/>
                <label>{"left_bicep_y"}</label>
                <input oninput={left_bicep_y} type="number"/>
                <br/>
                <label>{"left_forearm_x"}</label>
                <input oninput={left_forearm_x} type="number"/>
                <label>{"left_forearm_y"}</label>
                <input oninput={left_forearm_y} type="number"/>
                <br/>
                <label>{"right_calf_x"}</label>
                <input oninput={right_calf_x} type="number"/>
                <label>{"right_calf_y"}</label>
                <input oninput={right_calf_y} type="number"/>
                <br/>
                <label>{"right_thigh_x"}</label>
                <input oninput={right_thigh_x} type="number"/>
                <label>{"right_thigh_y"}</label>
                <input oninput={right_thigh_y} type="number"/>
                <br/>
                <label>{"right_bicep_x"}</label>
                <input oninput={right_bicep_x} type="number"/>
                <label>{"right_bicep_y"}</label>
                <input oninput={right_bicep_y} type="number"/>
                <br/>
                <label>{"right_forearm_x"}</label>
                <input oninput={right_forearm_x} type="number"/>
                <label>{"right_forearm_y"}</label>
                <input oninput={right_forearm_y} type="number"/>
                <br/>
                <label>{"torso_x"}</label>
                <input oninput={torso_x} type="number"/>
                <label>{"torso_y"}</label>
                <input oninput={torso_y} type="number"/>
                <br/>
                <label>{"head_x"}</label>
                <input oninput={head_x} type="number" name="head_x"/>
                <label>{"head_y"}</label>
                <input oninput={head_y} type="number" name="head_y"/>
                <br/>
                <svg viewBox={format!("{} {} {} {}", viewport.x0, viewport.y0, viewport.x1, viewport.y1)}  height="100%"  preserveAspectRatio="xMidYMid meet">
                    <image href="./public/temp.png" height="1" width="1.5"/>
                    { self.human.view(viewport) }
                </svg>
                </div>
        }
    }
}
