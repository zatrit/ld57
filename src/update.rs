use std::time::{Duration, Instant};

use raylib::RaylibHandle;

pub struct UpdateData<'r> {
    pub raylib: &'r mut RaylibHandle,
    pub delta_time: Duration,
}

impl<'r> UpdateData<'r> {
    pub fn new(raylib: &'r mut RaylibHandle, last_update: &mut Instant) -> Self {
        let now = Instant::now();
        let data = Self {
            raylib,
            delta_time: now - *last_update,
        };
        *last_update = now;

        data
    }
}
