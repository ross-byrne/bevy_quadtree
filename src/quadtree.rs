use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct TreeNode {
    pub position: Vec2,
    pub entity: Option<Entity>,
}

impl TreeNode {
    pub fn new(entity: Option<Entity>, x: f32, y: f32) -> Self {
        return Self {
            entity,
            position: Vec2::new(x, y),
        };
    }
}

// Need to use box here becuase struct is recursive
#[derive(Debug, Default)]
pub struct QuadTree {
    pub children: Vec<TreeNode>,
    pub subdivided: bool,
    pub rect: Rect,
    capacity: usize,
    north_east: Option<Box<QuadTree>>,
    north_west: Option<Box<QuadTree>>,
    south_east: Option<Box<QuadTree>>,
    south_west: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(origin: Vec2, half_size: Vec2, capacity: usize) -> Self {
        return Self {
            rect: Rect::from_center_half_size(origin, half_size),
            capacity,
            subdivided: false,
            children: vec![],
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
        };
    }

    pub fn add_child(&mut self, child: TreeNode) {
        if self.children.len() < self.capacity {
            self.children.push(child);
            return;
        }

        // otherwise, subdivide quad tree
        self.subdivide_tree()

        // somehow figure out where to put child???
    }

    pub fn get_childen(&mut self) -> &[TreeNode] {
        if !self.subdivided {
            return self.children.as_slice();
        }

        return self.children.as_slice();
    }

    // Just for display purposes
    pub fn get_tree_rects(&mut self) -> Vec<Rect> {
        if !self.subdivided {
            return vec![self.rect.clone()];
        }

        // get childen rects
        let mut child_rects: Vec<Rect> = vec![];

        if let Some(child) = self.north_east.as_mut() {
            child_rects.append(&mut child.get_tree_rects());
        }
        if let Some(child) = self.north_west.as_mut() {
            child_rects.append(&mut child.get_tree_rects());
        }
        if let Some(child) = self.south_east.as_mut() {
            child_rects.append(&mut child.get_tree_rects());
        }
        if let Some(child) = self.south_west.as_mut() {
            child_rects.append(&mut child.get_tree_rects());
        }

        return child_rects;
    }

    // hide ugly types so making new segments is easier to read
    fn new_tree_segment(&self, origin: Vec2, half_size: Vec2) -> Option<Box<QuadTree>> {
        let new_boxed_tree = Box::new(Self::new(origin, half_size, self.capacity));
        return Some(new_boxed_tree);
    }

    fn subdivide_tree(&mut self) {
        // calculate size of new segment
        // by halving the existing size
        let half_h = self.rect.height() / 2.0;
        let half_w = self.rect.width() / 2.0;
        let half_size: Vec2 = Vec2::new(half_w, half_h);

        // parent origin
        let x = self.rect.center().x;
        let y = self.rect.center().y;

        // calculate origin point for each new section
        let ne_origin: Vec2 = Vec2::new(x - half_w, y + half_h);
        let nw_origin: Vec2 = Vec2::new(x + half_w, y + half_h);
        let se_origin: Vec2 = Vec2::new(x - half_w, y - half_h);
        let sw_origin: Vec2 = Vec2::new(x + half_w, y - half_h);

        // create new tree segments
        self.north_east = self.new_tree_segment(ne_origin, half_size);
        self.north_west = self.new_tree_segment(nw_origin, half_size);
        self.south_east = self.new_tree_segment(se_origin, half_size);
        self.south_west = self.new_tree_segment(sw_origin, half_size);

        // mark as subdivided
        self.subdivided = true;
    }
}
