use crate::{animation, human};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[allow(unused_imports)]
use crate::trace;

pub enum TesterMsg {
    ClickedJoint(Joints, MouseEvent),
    MouseUp,
    MouseMove(MouseEvent),
    KeyDown(KeyboardEvent),
}

pub struct Tester {
    human: human::Human,
    /// target and offset x and y (in pixel coordinates)
    current_joint: Option<(Joints, (f64, f64))>,
    image_index: usize,
    editing_index: usize,
    output: animation::Animation,
}

#[derive(Clone)]
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
        trace!("create");
        Self {
            human: human::Human::new(),
            current_joint: None,
            image_index: 1,
            editing_index: 0,
            output: animation::Animation { positions: vec![] },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, tester_msg: Self::Message) -> bool {
        match tester_msg {
            TesterMsg::ClickedJoint(joint, e) => {
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
                let target = e
                    .target()
                    .expect("Should have target")
                    .dyn_into::<web_sys::SvgCircleElement>()
                    .unwrap();
                let offset = (
                    new_pos.0 - target.cx().base_val().value().unwrap() as f64,
                    new_pos.1 - target.cy().base_val().value().unwrap() as f64,
                );
                self.current_joint = Some((joint, offset));
                false
            }
            TesterMsg::MouseUp => {
                self.current_joint = None;
                false
            }
            TesterMsg::MouseMove(e) => {
                if let Some((joint, offset)) = &self.current_joint {
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
                    let new_pos = (new_pos.0 - offset.0, new_pos.1 - offset.1);
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
                const KEY_N: u32 = 78;
                const KEY_B: u32 = 66;
                const KEY_D: u32 = 68;
                const KEY_RIGHT: u32 = 39;
                const KEY_LEFT: u32 = 37;
                trace!("keydown: {:?}", e.key_code());
                let mut should_update = true;
                match e.key_code() {
                    KEY_N => {
                        self.output
                            .positions
                            .push(animation::AnimationPosition::from_human(&self.human));
                        self.image_index += 1;
                        self.editing_index += 1;
                    }
                    KEY_B => {
                        if self.output.positions.pop().is_some()
                            && self.image_index > 1
                            && self.editing_index > 0
                        {
                            self.image_index -= 1;
                            self.editing_index -= 1;
                        }
                    }
                    KEY_D => {
                        self.output.run_download();
                    }
                    KEY_RIGHT => {
                        self.image_index += 1;
                    }
                    KEY_LEFT => {
                        if let Some(i) = (self.image_index - 1).checked_sub(1) {
                            self.image_index = i + 1;
                        }
                    }
                    _ => should_update = false,
                }
                should_update
            }
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
        let joint_callback =
            move |a: Joints| link.callback(move |e| TesterMsg::ClickedJoint(a.clone(), e));
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
            joint_callback(Joints::LeftFoot),
            joint_callback(Joints::LeftKnee),
            joint_callback(Joints::Hip),
            joint_callback(Joints::RightKnee),
            joint_callback(Joints::RightFoot),
            joint_callback(Joints::Neck),
            joint_callback(Joints::LeftElbow),
            joint_callback(Joints::LeftHand),
            joint_callback(Joints::RightElbow),
            joint_callback(Joints::RightHand),
            joint_callback(Joints::Head),
        );

        html! {
            <div>
                <p>{ format!("Image index: {}, Editing index: {}", self.image_index, self.editing_index) }</p>
                <svg id="svg" viewBox={format!("{} {} {} {}", viewport.x0, viewport.y0, viewport.x1, viewport.y1)} onmouseup={link.callback(|_| TesterMsg::MouseUp)} onmousemove={link.callback(TesterMsg::MouseMove)} onkeydown={link.callback(TesterMsg::KeyDown)} height="100%" tabindex="0" preserveAspectRatio="xMidYMid meet">
                    <image href={format!("./public/output_{:0>3}.png", self.image_index)} height="1" width="1.5"/>
                    { self.human.view()}
                    <circle onmousedown={left_foot} cx={(self.human.joints.left_foot.x).to_string()} cy={(self.human.joints.left_foot.y).to_string()} r="0.02" stroke="aqua" stroke-width="0.01" />
                    <circle onmousedown={left_knee} cx={(self.human.joints.left_knee.x).to_string()} cy={(self.human.joints.left_knee.y).to_string()} r="0.02" stroke="aqua" stroke-width="0.01" />
                    <circle onmousedown={hip} cx={(self.human.joints.hip.x).to_string()} cy={(self.human.joints.hip.y).to_string()} r="0.02" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={right_knee} cx={(self.human.joints.right_knee.x).to_string()} cy={(self.human.joints.right_knee.y).to_string()} r="0.02" stroke="magenta" stroke-width="0.01" />
                    <circle onmousedown={right_foot} cx={(self.human.joints.right_foot.x).to_string()} cy={(self.human.joints.right_foot.y).to_string()} r="0.02" stroke="magenta" stroke-width="0.01" />
                    <circle onmousedown={neck} cx={(self.human.joints.neck.x).to_string()} cy={(self.human.joints.neck.y).to_string()} r="0.02" stroke="red" stroke-width="0.01" />
                    <circle onmousedown={left_elbow} cx={(self.human.joints.left_elbow.x).to_string()} cy={(self.human.joints.left_elbow.y).to_string()} r="0.02" stroke="blue" stroke-width="0.01" />
                    <circle onmousedown={left_hand} cx={(self.human.joints.left_hand.x).to_string()} cy={(self.human.joints.left_hand.y).to_string()} r="0.02" stroke="blue" stroke-width="0.01" />
                    <circle onmousedown={right_elbow} cx={(self.human.joints.right_elbow.x).to_string()} cy={(self.human.joints.right_elbow.y).to_string()} r="0.02" stroke="mediumorchid" stroke-width="0.01" />
                    <circle onmousedown={right_hand} cx={(self.human.joints.right_hand.x).to_string()} cy={(self.human.joints.right_hand.y).to_string()} r="0.02" stroke="mediumorchid" stroke-width="0.01" />
                    <circle onmousedown={head} cx={(self.human.joints.head.x).to_string()} cy={(self.human.joints.head.y).to_string()} r="0.02" stroke="red" stroke-width="0.01" />
                </svg>
            </div>
        }
    }
}
