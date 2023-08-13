use crate::pack::{Bounds, PackedSprite, Sprite};
use std::{iter, mem};

#[derive(Debug)]
pub enum NodeState<I> {
    Unused,
    Used,
    UsedLeaf(Sprite<I>),
}

impl<I> NodeState<I> {
    #[must_use]
    pub fn is_used(&self) -> bool {
        matches!(self, Self::Used | Self::UsedLeaf(_))
    }
}

#[derive(Debug)]
pub struct NodeChildren<I> {
    pub right: Node<I>,
    pub down: Node<I>,
}

#[derive(Debug)]
pub struct Node<I> {
    pub state: NodeState<I>,
    pub bounds: Bounds,
    pub children: Option<Box<NodeChildren<I>>>,
}

impl<I> Node<I> {
    pub fn root(sprite: Sprite<I>, spacing: (u32, u32)) -> Self {
        let w = sprite.size.0 + spacing.0;
        let h = sprite.size.1 + spacing.1;

        Self {
            state: NodeState::UsedLeaf(sprite),
            bounds: Bounds { x: 0, y: 0, w, h },
            children: None,
        }
    }

    pub fn unused(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            state: NodeState::Unused,
            bounds: Bounds { x, y, w, h },
            children: None,
        }
    }

    pub fn insert(&mut self, sprite: Sprite<I>, spacing: (u32, u32)) {
        let size = (sprite.size.0 + spacing.0, sprite.size.1 + spacing.1);

        let node = match self.find(size) {
            Some(node) => node,
            None => self.grow(size),
        };

        node.set_used(sprite, size);
    }

    pub fn into_packed_sprites(self) -> impl Iterator<Item = PackedSprite<I>> {
        let mut nodes = vec![self];

        iter::from_fn(move || {
            while let Some(node) = nodes.pop() {
                if node.state.is_used() {
                    if let Some(children) = node.children {
                        nodes.extend([children.right, children.down]);
                    }
                }

                if let NodeState::UsedLeaf(sprite) = node.state {
                    return Some(PackedSprite {
                        id: sprite.id,
                        bounds: Bounds::new(
                            node.bounds.x,
                            node.bounds.y,
                            sprite.size.0,
                            sprite.size.1,
                        ),
                    });
                }
            }

            None
        })
    }

    const fn empty() -> Self {
        Self {
            state: NodeState::Unused,
            bounds: Bounds::new(0, 0, 0, 0),
            children: None,
        }
    }

    fn find(&mut self, (w, h): (u32, u32)) -> Option<&mut Self> {
        if self.state.is_used() {
            let children = self.children.as_mut()?;

            match children.right.find((w, h)) {
                Some(node) => Some(node),
                None => children.down.find((w, h)),
            }
        } else if w <= self.bounds.w && h <= self.bounds.h {
            Some(self)
        } else {
            None
        }
    }

    fn set_used(&mut self, sprite: Sprite<I>, (w, h): (u32, u32)) {
        self.state = NodeState::UsedLeaf(sprite);

        let bounds = self.bounds;
        self.children = Some(Box::new(NodeChildren {
            right: Self::unused(bounds.x + w, bounds.y, bounds.w - w, bounds.h),
            down: Self::unused(bounds.x, bounds.y + h, w, bounds.h - h),
        }));
    }

    fn grow(&mut self, (w, h): (u32, u32)) -> &mut Self {
        let can_grow_right = h <= self.bounds.h;
        let can_grow_down = w <= self.bounds.w;

        let should_grow_right = can_grow_right && (self.bounds.w + w <= self.bounds.h);
        let should_grow_down = can_grow_down && (self.bounds.h + h <= self.bounds.w);

        if should_grow_right {
            self.grow_right(w)
        } else if should_grow_down {
            self.grow_down(h)
        } else if can_grow_right {
            self.grow_right(w)
        } else if can_grow_down {
            self.grow_down(h)
        } else {
            panic!("Cannot grow");
        }
    }

    fn grow_right(&mut self, w: u32) -> &mut Self {
        let bounds = self.bounds;
        let down = mem::replace(self, Self::empty());

        *self = Self {
            state: NodeState::Used,
            bounds: Bounds::new(0, 0, bounds.w + w, bounds.h),
            children: Some(Box::new(NodeChildren {
                right: Self::unused(bounds.w, 0, w, bounds.h),
                down,
            })),
        };

        &mut self.children.as_mut().unwrap().right
    }

    fn grow_down(&mut self, h: u32) -> &mut Self {
        let bounds = self.bounds;
        let right = mem::replace(self, Self::empty());

        *self = Self {
            state: NodeState::Used,
            bounds: Bounds::new(0, 0, bounds.w, bounds.h + h),
            children: Some(Box::new(NodeChildren {
                right,
                down: Self::unused(0, bounds.h, bounds.w, h),
            })),
        };

        &mut self.children.as_mut().unwrap().down
    }
}
