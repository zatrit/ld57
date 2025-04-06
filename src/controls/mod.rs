mod key;

use key::u32_to_keyboard_key;
use raylib::{
    RaylibHandle,
    ffi::KeyboardKey::{self, *},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! key_check {
    ($func:ident, $check:ident) => {
        pub fn $func(&self, raylib: &mut RaylibHandle) -> bool {
            self.0.is_some_and(|k| raylib.$check(k)) || self.1.is_some_and(|k| raylib.$check(k))
        }
    };
}

#[derive(Default, Debug)]
pub struct KeyMap(pub Option<KeyboardKey>, pub Option<KeyboardKey>);

#[derive(Serialize, Deserialize, Debug)]
pub struct RawKeyMap(pub Option<u32>, pub Option<u32>);

impl From<&KeyMap> for RawKeyMap {
    fn from(value: &KeyMap) -> Self {
        RawKeyMap(value.1.map(|k| k as u32), value.0.map(|k| k as u32))
    }
}

impl From<RawKeyMap> for KeyMap {
    fn from(value: RawKeyMap) -> Self {
        KeyMap(
            value.1.and_then(u32_to_keyboard_key),
            value.0.and_then(u32_to_keyboard_key),
        )
    }
}

#[allow(unused)]
impl KeyMap {
    key_check!(is_pressed, is_key_pressed);
    key_check!(is_pressed_repeat, is_key_pressed_repeat);
    key_check!(is_down, is_key_down);
    key_check!(is_up, is_key_up);
    key_check!(is_released, is_key_released);
}

impl Serialize for KeyMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let raw = RawKeyMap::from(self);
        raw.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeyMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawKeyMap::deserialize(deserializer)?;
        Ok(Self::from(raw))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Controls {
    pub left: KeyMap,
    pub right: KeyMap,
    pub up: KeyMap,
    pub down: KeyMap,
    pub interact: KeyMap,
    pub inventory: KeyMap,
}

impl Controls {
    pub const DEFAULT: Controls = Controls {
        left: KeyMap(Some(KEY_LEFT), Some(KEY_A)),
        right: KeyMap(Some(KEY_RIGHT), Some(KEY_D)),
        up: KeyMap(Some(KEY_UP), Some(KEY_W)),
        down: KeyMap(Some(KEY_DOWN), Some(KEY_S)),
        interact: KeyMap(Some(KEY_E), Some(KEY_Z)),
        inventory: KeyMap(Some(KEY_R), Some(KEY_C)),
    };
}

impl Default for Controls {
    fn default() -> Self {
        Self::DEFAULT
    }
}
