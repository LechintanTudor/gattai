use crate::bounds::{Bounds, Position, Size};
use crate::cli_args::CliArgs;
use crate::reader::Image;
use image::GenericImageView;
use std::cmp::Reverse;

#[must_use]
#[derive(Default, Debug)]
pub struct PackerResult {
    pub size: Size,
    pub images: Vec<Image>,
    pub sprites: Vec<Sprite>,
}

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub image_index: usize,
    pub position: Position,
}

pub fn run(cli_args: &CliArgs, mut images: Vec<Image>) -> PackerResult {
    images.sort_unstable_by_key(|image| {
        let (w, h) = image.image.dimensions();
        let sizes = if w >= h { (w, h) } else { (h, w) };
        Reverse(sizes)
    });

    let Some((first, others)) = images.split_first() else {
        return PackerResult::default();
    };

    let (size, sprites) = {
        let first_size = {
            let (w, h) = first.image.dimensions();
            Size::new(w, h)
        };

        let spacing = Size::new(
            cli_args.spacing_x.unwrap_or(cli_args.spacing),
            cli_args.spacing_y.unwrap_or(cli_args.spacing),
        );

        let mut tree = Tree::new(first_size, spacing);

        for (i, image) in others.iter().enumerate() {
            let size = {
                let (w, h) = image.image.dimensions();
                Size::new(w, h)
            };

            tree.insert(i, size, spacing);
        }

        let padding = Size::new(
            cli_args.padding_x.unwrap_or(cli_args.padding),
            cli_args.padding_y.unwrap_or(cli_args.padding),
        );

        let mut size = tree.nodes[0].bounds.size();
        size.w += 2 * padding.w - spacing.w;
        size.h += 2 * padding.h - spacing.h;

        (size, tree.collect_sprites(padding))
    };

    PackerResult {
        size,
        images,
        sprites,
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Node {
    state: NodeState,
    bounds: Bounds,
    children: Option<NodeChildren>,
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

#[derive(Clone, Copy, Default, Debug)]
enum NodeState {
    #[default]
    Unused,
    Used,
    UsedLeaf(usize),
}

impl NodeState {
    #[inline]
    #[must_use]
    fn is_used(&self) -> bool {
        matches!(self, Self::Used | Self::UsedLeaf(_))
    }
}

#[derive(Clone, Copy, Debug)]
struct NodeChildren {
    right_index: usize,
    down_index: usize,
}

#[derive(Clone, Debug)]
struct Tree {
    nodes: Vec<Node>,
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
        let root = &self.nodes[0];
        let can_grow_right = size.h <= root.bounds.h;
        let can_grow_down = size.w <= root.bounds.w;

        let should_grow_right =
            can_grow_right && (root.bounds.w + size.w <= root.bounds.h);

        let should_grow_down =
            can_grow_down && (root.bounds.h + size.h <= root.bounds.w);

        if should_grow_right {
            self.grow_right(size.w)
        } else if should_grow_down {
            self.grow_down(size.h)
        } else if can_grow_right {
            self.grow_right(size.w)
        } else if can_grow_down {
            self.grow_down(size.h)
        } else {
            panic!("Cannot grow sprite tree");
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

    #[must_use]
    fn collect_sprites(&self, padding: Size) -> Vec<Sprite> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if let NodeState::UsedLeaf(image_index) = node.state {
                    Some(Sprite {
                        image_index,
                        position: Position::new(
                            node.bounds.x + padding.w,
                            node.bounds.y + padding.h,
                        ),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
