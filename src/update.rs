use std::time::{Duration, Instant};

use raylib::RaylibHandle;

pub struct UpdateData<'r> {
    pub raylib: &'r mut RaylibHandle,
    pub delta_time: f32,
}

impl<'r> UpdateData<'r> {
    pub fn new(raylib: &'r mut RaylibHandle) -> Self {
        Self {
            delta_time: raylib.get_frame_time(),
            raylib,
        }
    }
}
