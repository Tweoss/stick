use gloo_render::{request_animation_frame, AnimationFrame};
use log::Level;
use yew::prelude::*;
#[macro_use]
extern crate lazy_static;

mod animation;
mod human;
mod tester;

// Allow other modules to use the logging macro
// use print;
// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! trace {
    ($($t:tt)*) => {
        log::debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}
pub(crate) use trace;

enum Msg {
    Tick(f64),
}

struct Model {
    human: human::Human,
    animation_id: Option<AnimationFrame>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::Tick);
        let request_id = request_animation_frame(move |t: f64| callback.emit(t));
        print!("create");
        Self {
            human: human::Human::new(),
            animation_id: Some(request_id),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick(t) => {
                self.human.update(t);
                let callback = ctx.link().callback(Msg::Tick);
                let request_id = request_animation_frame(move |t: f64| callback.emit(t));
                self.animation_id = Some(request_id);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let viewport = human::Viewport {
            x0: 0.0,
            y0: 0.0,
            x1: 1.5,
            y1: 1.0,
        };
        html! {
            <div>
                <svg viewBox={format!("{} {} {} {}", viewport.x0, viewport.y0, viewport.x1, viewport.y1)} class="svg-container">
                    { self.human.view() }
                </svg>
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Debug).unwrap();
    // yew::start_app::<Model>();
    yew::start_app::<tester::Tester>();
}
