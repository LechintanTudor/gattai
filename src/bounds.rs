use serde::Serialize;

#[derive(Clone, Copy, Default, Debug, Serialize)]
pub struct Bounds {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Bounds {
    #[inline]
    #[must_use]
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> Size {
        Size::new(self.w, self.h)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    #[inline]
    #[must_use]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    #[inline]
    #[must_use]
    pub const fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}
