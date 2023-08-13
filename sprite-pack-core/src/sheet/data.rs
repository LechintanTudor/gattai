use crate::pack::Bounds;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::hash::BuildHasher;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpriteData {
    pub name: String,
    pub bounds: Bounds,
}

pub trait SpriteSheetData: Serialize {
    fn add_sprite(&mut self, name: String, bounds: Bounds);
}

impl SpriteSheetData for Vec<SpriteData> {
    #[inline]
    fn add_sprite(&mut self, name: String, bounds: Bounds) {
        self.push(SpriteData { name, bounds })
    }
}

impl<S> SpriteSheetData for HashMap<String, Bounds, S>
where
    S: BuildHasher,
{
    #[inline]
    fn add_sprite(&mut self, name: String, bounds: Bounds) {
        let _ = self.insert(name, bounds);
    }
}

#[derive(Clone, Debug)]
pub enum GenericSpriteSheetData {
    Vec(Vec<SpriteData>),
    Map(HashMap<String, Bounds>),
}

impl GenericSpriteSheetData {
    #[inline]
    pub fn new_vec() -> Self {
        Self::Vec(Default::default())
    }

    #[inline]
    pub fn new_map() -> Self {
        Self::Map(Default::default())
    }
}

impl SpriteSheetData for GenericSpriteSheetData {
    #[inline]
    fn add_sprite(&mut self, name: String, bounds: Bounds) {
        match self {
            Self::Vec(data) => data.add_sprite(name, bounds),
            Self::Map(data) => data.add_sprite(name, bounds),
        }
    }
}

impl Serialize for GenericSpriteSheetData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Vec(data) => data.serialize(serializer),
            Self::Map(data) => data.serialize(serializer),
        }
    }
}
