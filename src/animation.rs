use crate::human;
use human::Point;
use wasm_bindgen::JsCast;

#[allow(unused_imports)]
use crate::trace;

/// Milliseconds in between animation positions.
const ANIMATION_POSITION_INTERVAL: f64 = 20.0;

#[derive(Debug)]
pub struct AnimationsData {
    pub walking: Animation,
}

impl AnimationsData {
    /// Include animations from files as parsed bytes.
    pub fn load_animations() -> AnimationsData {
        AnimationsData {
            walking: serde_cbor::from_slice(include_bytes!("../data/output.cbor")).unwrap(),
        }
    }
}

/// Struct containing animation positions
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Animation {
    pub positions: Vec<AnimationPosition>,
}

impl Animation {
    /// Generate a download of the positions stored (for development purposes)
    pub fn run_download(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let byte_vector = serde_cbor::to_vec(&self).expect("unable to serialize object");
        let array = js_sys::Array::new();
        array.push(&js_sys::Uint8Array::from(&byte_vector[..]));
        let blob =
            web_sys::Blob::new_with_u8_array_sequence(&array).expect("unable to create blob");

        let a = document
            .create_element("a")
            .expect("should have created an element");
        let a = a
            .dyn_ref::<web_sys::HtmlAnchorElement>()
            .expect("should have created an anchor element");
        a.set_href(
            &web_sys::Url::create_object_url_with_blob(&blob).expect("unable to create url"),
        );
        let body = document.body().expect("should have a body");
        a.set_download("output.cbor");
        body.append_child(a)
            .expect("should have appended the anchor element to the body");
        a.click();
        body.remove_child(a)
            .expect("should have removed the anchor element from the body");
        trace!("downloaded");
    }
    /// Get an interpolated position for a time since the start of the animation
    /// Returns none if the time is out of bounds for the specific animation
    pub fn step(&self, time_step: f64) -> Option<AnimationPosition> {
        let index = (time_step / ANIMATION_POSITION_INTERVAL).floor() as usize;
        if index >= self.positions.len() {
            None
        } else {
            let start = &self.positions.get(index)?;
            let end = &self.positions.get(index + 1)?;
            let time_offset = time_step % ANIMATION_POSITION_INTERVAL;
            let offset_normalized = time_offset / ANIMATION_POSITION_INTERVAL;
            Some(start.lerp(end, offset_normalized))
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AnimationPosition {
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

impl AnimationPosition {
    /// Clone the position of a human into a new position
    pub fn from_human(human: &human::Human) -> Self {
        human.joints.clone()
    }
    /// linear interpolation between two positions
    fn lerp(&self, next: &AnimationPosition, offset: f64) -> Self {
        AnimationPosition {
            left_foot: self.left_foot.lerp(&next.left_foot, offset),
            left_knee: self.left_knee.lerp(&next.left_knee, offset),
            hip: self.hip.lerp(&next.hip, offset),
            right_knee: self.right_knee.lerp(&next.right_knee, offset),
            right_foot: self.right_foot.lerp(&next.right_foot, offset),
            neck: self.neck.lerp(&next.neck, offset),
            left_elbow: self.left_elbow.lerp(&next.left_elbow, offset),
            left_hand: self.left_hand.lerp(&next.left_hand, offset),
            right_elbow: self.right_elbow.lerp(&next.right_elbow, offset),
            right_hand: self.right_hand.lerp(&next.right_hand, offset),
            head: self.head.lerp(&next.head, offset),
        }
    }
}
