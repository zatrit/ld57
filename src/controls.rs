use raylib::{RaylibHandle, ffi::KeyboardKey, math::Vector2};
use serde::{Serialize, Serializer, ser::SerializeMap};

use crate::update::UpdateData;

const PIXELS_PER_SECOND: f32 = 75.;
const DEGREES_PER_SECOND: f32 = 60.;

#[derive(Clone, Copy, Debug)]
pub struct Controls {
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub up: KeyboardKey,
    pub down: KeyboardKey,

    pub clockwise: KeyboardKey,
    pub counter_clockwise: KeyboardKey,

    pub recolor: KeyboardKey,
}

impl Default for Controls {
    fn default() -> Self {
        use KeyboardKey::*;
        Self {
            left: KEY_A,
            right: KEY_D,
            up: KEY_W,
            down: KEY_S,

            clockwise: KEY_E,
            counter_clockwise: KEY_Q,

            recolor: KEY_ENTER,
        }
    }
}

#[inline(always)]
fn axis(raylib: &mut RaylibHandle, key: KeyboardKey) -> f32 {
    raylib.is_key_down(key) as u32 as f32
}

impl Controls {
    pub fn update(&self, update_data: &mut UpdateData, pos: &mut Vector2, rotation: &mut f32) {
        let UpdateData { raylib, delta_time } = update_data;

        let d = delta_time.as_secs_f32() * PIXELS_PER_SECOND;

        let dx = (axis(raylib, self.right) - axis(raylib, self.left)) * d;
        pos.x += dx;

        let dy = (axis(raylib, self.down) - axis(raylib, self.up)) * d;
        pos.y += dy;

        let d = delta_time.as_secs_f32() * DEGREES_PER_SECOND;
        *rotation += (axis(raylib, self.clockwise) - axis(raylib, self.counter_clockwise)) * d;
    }
}

impl Serialize for Controls {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let (left, right, up, down) = (
            self.left as u32,
            self.right as u32,
            self.up as u32,
            self.down as u32,
        );
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("left", &left)?;
        map.serialize_entry("right", &right)?;
        map.serialize_entry("up", &up)?;
        map.serialize_entry("down", &down)?;
        map.end()
    }
}
