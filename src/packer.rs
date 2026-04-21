use crate::config::Config;
use crate::math::{Bounds, Position, Size};
use crate::reader::Sprite;
use image::GenericImageView;

#[derive(Default, Debug)]
pub struct PackerResult {
    pub size: Size,
    pub sprites: Vec<Sprite>,
}

#[derive(Clone, Debug)]
struct Tree {
    nodes: Vec<Node>,
}

#[derive(Clone, Copy, Default, Debug)]
struct Node {
    state: NodeState,
    bounds: Bounds,
    children: Option<NodeChildren>,
}

#[derive(Clone, Copy, Default, Debug)]
enum NodeState {
    #[default]
    Unused,
    Used,
    UsedLeaf(usize),
}

#[derive(Clone, Copy, Debug)]
struct NodeChildren {
    right_index: usize,
    down_index: usize,
}

pub fn run(config: &Config, mut sprites: Vec<Sprite>) -> PackerResult {
    sprites.sort_by(|s1, s2| {
        let size1 = {
            let (w, h) = s1.image.dimensions();
            if w >= h { (w, h) } else { (h, w) }
        };

        let size2 = {
            let (w, h) = s2.image.dimensions();
            if w >= h { (w, h) } else { (h, w) }
        };

        (size1.cmp(&size2).reverse())
            .then_with(|| s1.name.cmp(&s2.name))
            .then_with(|| s1.index.cmp(&s2.index))
    });

    let Some((first, others)) = sprites.split_first() else {
        return PackerResult::default();
    };

    let first_size = {
        let (w, h) = first.image.dimensions();
        Size::new(w, h)
    };

    let spacing = Size::new(config.spacing_x, config.spacing_y);
    let mut tree = Tree::new(first_size, spacing);

    for (i, image) in others.iter().enumerate() {
        let size = {
            let (w, h) = image.image.dimensions();
            Size::new(w, h)
        };

        tree.insert(i + 1, size, spacing);
    }

    let mut size = tree.nodes[0].bounds.size();
    let padding = Size::new(config.padding_x, config.padding_y);

    size.w -= spacing.w;
    size.w += 2 * padding.w;

    size.h -= spacing.h;
    size.h += 2 * padding.h;

    tree.position_sprites(&mut sprites, padding);
    PackerResult { size, sprites }
}

impl Tree {
    fn new(image_size: Size, spacing: Size) -> Self {
        let root = Node {
            state: NodeState::UsedLeaf(0),
            bounds: Bounds::new(
                0,
                0,
                image_size.w + spacing.w,
                image_size.h + spacing.h,
            ),
            children: None,
        };

        Self { nodes: vec![root] }
    }

    fn insert(&mut self, image_index: usize, image_size: Size, spacing: Size) {
        let size =
            Size::new(image_size.w + spacing.w, image_size.h + spacing.h);

        let node_index = self.find(0, size).unwrap_or_else(|| self.grow(size));
        self.set_used(node_index, image_index, size);
    }

    fn find(&self, node_index: usize, size: Size) -> Option<usize> {
        let node = &self.nodes[node_index];

        if node.state.is_used() {
            let children = node.children?;

            self.find(children.right_index, size)
                .or_else(|| self.find(children.down_index, size))
        } else if size.w <= node.bounds.w && size.h <= node.bounds.h {
            Some(node_index)
        } else {
            None
        }
    }

    fn grow(&mut self, size: Size) -> usize {
        let bounds = self.nodes[0].bounds;
        let can_grow_right = size.h <= bounds.h;
        let can_grow_down = size.w <= bounds.w;

        let should_grow_right =
            can_grow_right && (bounds.w + size.w <= bounds.h);

        let should_grow_down = can_grow_down && (bounds.h + size.h <= bounds.w);

        if should_grow_right {
            self.grow_right(size.w)
        } else if should_grow_down {
            self.grow_down(size.h)
        } else if can_grow_right {
            self.grow_right(size.w)
        } else if can_grow_down {
            self.grow_down(size.h)
        } else {
            panic!("Cannot grow image");
        }
    }

    fn grow_right(&mut self, w: u32) -> usize {
        let bounds = self.nodes[0].bounds;

        let right_index = self.nodes.len();
        self.nodes.push(Node::unused(bounds.w, 0, w, bounds.h));

        let down_index = self.nodes.len();
        self.nodes.push(self.nodes[0]);

        self.nodes[0] = Node {
            state: NodeState::Used,
            bounds: Bounds::new(0, 0, bounds.w + w, bounds.h),
            children: Some(NodeChildren {
                right_index,
                down_index,
            }),
        };

        right_index
    }

    fn grow_down(&mut self, h: u32) -> usize {
        let bounds = self.nodes[0].bounds;

        let right_index = self.nodes.len();
        self.nodes.push(self.nodes[0]);

        let down_index = self.nodes.len();
        self.nodes.push(Node::unused(0, bounds.h, bounds.w, h));

        self.nodes[0] = Node {
            state: NodeState::Used,
            bounds: Bounds::new(0, 0, bounds.w, bounds.h + h),
            children: Some(NodeChildren {
                right_index,
                down_index,
            }),
        };

        down_index
    }

    fn set_used(&mut self, node_index: usize, image_index: usize, size: Size) {
        let node = &mut self.nodes[node_index];
        let bounds = node.bounds;
        node.state = NodeState::UsedLeaf(image_index);

        let right_index = self.nodes.len();
        self.nodes.push(Node::unused(
            bounds.x + size.w,
            bounds.y,
            bounds.w - size.w,
            bounds.h,
        ));

        let down_index = self.nodes.len();
        self.nodes.push(Node::unused(
            bounds.x,
            bounds.y + size.h,
            bounds.w,
            bounds.h - size.h,
        ));

        self.nodes[node_index].children = Some(NodeChildren {
            right_index,
            down_index,
        });
    }

    fn position_sprites(&self, sprites: &mut [Sprite], padding: Size) {
        for node in &self.nodes {
            if let NodeState::UsedLeaf(i) = &node.state {
                sprites[*i].position = Position::new(
                    node.bounds.x + padding.w,
                    node.bounds.y + padding.h,
                );
            }
        }
    }
}

impl Node {
    #[must_use]
    fn unused(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            state: NodeState::Unused,
            bounds: Bounds::new(x, y, w, h),
            children: None,
        }
    }
}

impl NodeState {
    #[inline]
    #[must_use]
    fn is_used(&self) -> bool {
        matches!(self, Self::Used | Self::UsedLeaf(_))
    }
}
