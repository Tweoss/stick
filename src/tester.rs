use crate::human;
use log::{debug};
use wasm_bindgen::JsCast;
use yew::prelude::*;

// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! log {
    ($($t:tt)*) => {
        debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}

pub enum TesterMsg {
    ClickedJoint(Joints),
    MouseUp,
    MouseMove(MouseEvent),
    KeyDown(KeyboardEvent),
}

pub struct Tester {
    human: human::Human,
    current_joint: Option<Joints>,
    image_index: usize,
}

pub enum Joints {
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

    fn create(_ctx: &Context<Self>) -> Self {
        log!("create");
        Self {
            human: human::Human::new(),
            current_joint: None,
            image_index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, tester_msg: Self::Message) -> bool {
        match tester_msg {
            TesterMsg::ClickedJoint(joint) => {
                self.current_joint = Some(joint);
                false
            }
            TesterMsg::MouseUp => {
                self.current_joint = None;
                false
            }
            TesterMsg::MouseMove(e) => {
                if let Some(joint) = &self.current_joint {
                    let mouse_pos = (e.client_x(), e.client_y());
                    let window = web_sys::window().expect("no global `window` exists");
                    let document = window.document().expect("should have a document on window");
                    let transform_matrix = document
                        .get_element_by_id("svg")
                        .unwrap()
                        .dyn_into::<web_sys::SvgGraphicsElement>()
                        .expect("should be an svg element")
                        .get_screen_ctm()
                        .expect("should have a transform matrix")
                        .inverse()
                        .expect("should have an inverse matrix");
                    let mp = (mouse_pos.0 as f32, mouse_pos.1 as f32);
                    let tm = transform_matrix;
                    let new_pos = (
                        (tm.a() * mp.0 + tm.c() * mp.1 + tm.e()) as f64,
                        (tm.b() * mp.0 + tm.d() * mp.1 + tm.f()) as f64,
                    );
                    use human::UpdateHuman;
                    self.human.update_human(match joint {
                        Joints::LeftFoot => UpdateHuman::LeftFoot(Some(new_pos.0), Some(new_pos.1)),
                        Joints::LeftKnee => UpdateHuman::LeftKnee(Some(new_pos.0), Some(new_pos.1)),
                        Joints::Hip => UpdateHuman::Hip(Some(new_pos.0), Some(new_pos.1)),
                        Joints::RightKnee => {
                            UpdateHuman::RightKnee(Some(new_pos.0), Some(new_pos.1))
                        }
                        Joints::RightFoot => {
                            UpdateHuman::RightFoot(Some(new_pos.0), Some(new_pos.1))
                        }
                        Joints::Neck => UpdateHuman::Neck(Some(new_pos.0), Some(new_pos.1)),
                        Joints::LeftElbow => {
                            UpdateHuman::LeftElbow(Some(new_pos.0), Some(new_pos.1))
                        }
                        Joints::LeftHand => UpdateHuman::LeftHand(Some(new_pos.0), Some(new_pos.1)),
                        Joints::RightElbow => {
                            UpdateHuman::RightElbow(Some(new_pos.0), Some(new_pos.1))
                        }
                        Joints::RightHand => {
                            UpdateHuman::RightHand(Some(new_pos.0), Some(new_pos.1))
                        }
                        Joints::Head => UpdateHuman::Head(Some(new_pos.0), Some(new_pos.1)),
                    });
                }
                true
            }
            TesterMsg::KeyDown(e) => {
                const KEY_N: u32= 78;
                log!("keydown: {:?}", e.key_code());
                if e.key_code() == KEY_N {
                    self.image_index += 1;
                    true
                } else {
                    false
                }
            },
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
        let (
            left_foot,
            left_knee,
            hip,
            right_knee,
            right_foot,
            neck,
            left_elbow,
            left_hand,
            right_elbow,
            right_hand,
            head,
        ) = (
            link.callback(|_| TesterMsg::ClickedJoint(Joints::LeftFoot)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::LeftKnee)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::Hip)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::RightKnee)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::RightFoot)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::Neck)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::LeftElbow)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::LeftHand)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::RightElbow)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::RightHand)),
            link.callback(|_| TesterMsg::ClickedJoint(Joints::Head)),
        );

        html! {
            <div>
                <p>{ format!("{}", self.image_index) }</p>
                <svg id="svg" viewBox={format!("{} {} {} {}", viewport.x0, viewport.y0, viewport.x1, viewport.y1)} onmouseup={link.callback(|_| TesterMsg::MouseUp)} onmousemove={link.callback(TesterMsg::MouseMove)} onkeydown={link.callback(TesterMsg::KeyDown)} height="100%" tabindex="0" preserveAspectRatio="xMidYMid meet">
                    <image href={format!("./public/temp{}.png", self.image_index)} height="1" width="1.5"/>
                    { self.human.view() }
                    <circle onmousedown={left_foot} cx={(self.human.left_foot.x).to_string()} cy={(self.human.left_foot.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={left_knee} cx={(self.human.left_knee.x).to_string()} cy={(self.human.left_knee.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={hip} cx={(self.human.hip.x).to_string()} cy={(self.human.hip.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={right_knee} cx={(self.human.right_knee.x).to_string()} cy={(self.human.right_knee.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={right_foot} cx={(self.human.right_foot.x).to_string()} cy={(self.human.right_foot.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={neck} cx={(self.human.neck.x).to_string()} cy={(self.human.neck.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={left_elbow} cx={(self.human.left_elbow.x).to_string()} cy={(self.human.left_elbow.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={left_hand} cx={(self.human.left_hand.x).to_string()} cy={(self.human.left_hand.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={right_elbow} cx={(self.human.right_elbow.x).to_string()} cy={(self.human.right_elbow.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={right_hand} cx={(self.human.right_hand.x).to_string()} cy={(self.human.right_hand.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={head} cx={(self.human.head.x).to_string()} cy={(self.human.head.y).to_string()} r="0.005" stroke="red" stroke-width="0.01" />
                </svg>
            </div>
        }
    }
}
