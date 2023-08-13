use crate::pack::Bounds;

#[derive(Debug)]
pub struct Sprite<I> {
    pub id: I,
    pub size: (u32, u32),
}

#[derive(Debug)]
pub struct PackedSprite<I> {
    pub id: I,
    pub bounds: Bounds,
}
