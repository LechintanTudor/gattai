use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Bounds {
    #[inline]
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    #[must_use]
    pub const fn position(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}
