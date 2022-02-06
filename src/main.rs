use gloo_render::{request_animation_frame, AnimationFrame};
use log::{debug, Level};
use yew::prelude::*;

mod human;
mod tester;

// Logging macro for easier debugging. Displays file and line number. Use with `log!("Hello World")`
macro_rules! log {
    ($($t:tt)*) => {
        debug!("[{}:{}] {}", file!(), line!(), &format_args!($($t)*).to_string());
    }
}

enum Msg {
    Tick(f64),
}

struct Model {
    human: human::Human,
    // last_tick: Option<f64>,
    animation_id: Option<AnimationFrame>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::Tick);
        let request_id = request_animation_frame(move |t: f64| callback.emit(t));
        log!("create");
        Self {
            human: human::Human::new(),
            // last_tick: None,
            animation_id: Some(request_id),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick(t) => {
                // let out = match self.last_tick {
                //     None => {
                //         self.last_tick = Some(t);
                //         false
                //     }
                //     Some(last_tick) => {
                //         // let delta = t - last_tick;
                //         self.last_tick = Some(t);
                //         false
                //     }
                // };
                self.human.update(t);
                let callback = ctx.link().callback(Msg::Tick);
                let request_id = request_animation_frame(move |t: f64| callback.emit(t));
                self.animation_id = Some(request_id);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let link = ctx.link();
        let viewport = human::Viewport {
            x0: 0.0,
            y0: 0.0,
            x1: 1.5,
            y1: 1.0,
        };
        html! {
            <div>
                <svg viewBox={format!("{} {} {} {}", viewport.x0, viewport.y0, viewport.x1, viewport.y1)} class="svg-container">
                    // <rect x="0" y="0" width="1.5" height="1.0"></rect>
                    // <circle cx={("0.0", rx_view.clone().filter_map(|x| future::ready(if let AppView::MoveDot(a, _) = x {Some(a.to_string())} else {None})))} cy={("0.0", rx_view.clone().filter_map(|x| future::ready(if let AppView::MoveDot(_, a) = x {Some(a.to_string())} else {None})))} r="0.1" fill="red" xmlns=ns></circle>
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
