use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy)]
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
    index: usize,
    north_east: Option<Box<QuadTree>>,
    north_west: Option<Box<QuadTree>>,
    south_east: Option<Box<QuadTree>>,
    south_west: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(origin: Vec2, half_size: Vec2, capacity: usize) -> Self {
        // calls new with 0 index to signify root
        return Self::new_child_segment(origin, half_size, capacity, 0);
    }

    fn new_child_segment(origin: Vec2, half_size: Vec2, capacity: usize, index: usize) -> Self {
        return Self {
            rect: Rect::from_center_half_size(origin, half_size),
            capacity,
            index,
            subdivided: false,
            children: Vec::new(),
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
        };
    }

    fn child_intersects(&self, point: &Vec2) -> bool {
        return self.rect.contains(*point);
    }

    pub fn insert(&mut self, child: TreeNode) {
        if self.children.len() < self.capacity {
            info!("Adding child to segment index: {}", self.index);
            self.children.push(child);
            return;
        }

        if !self.subdivided {
            info!("Capacity full, starting to subdivide");
            // otherwise, subdivide quad tree
            self.subdivide_tree();
        }

        // check each segment if child is contained within them
        for option in [
            self.north_east.as_mut(),
            self.north_west.as_mut(),
            self.south_east.as_mut(),
            self.south_west.as_mut(),
        ] {
            if let Some(segment) = option {
                if segment.child_intersects(&child.position) {
                    segment.insert(child);
                    return;
                }
            }
        }

        // info!("Failed to find place for child: {:?}", child.position);
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

        let mut child_rects: Vec<Rect> = vec![];

        // add root rect to list
        child_rects.append(&mut vec![self.rect.clone()]);

        // then get childen rects
        for option in [
            self.north_east.as_mut(),
            self.north_west.as_mut(),
            self.south_east.as_mut(),
            self.south_west.as_mut(),
        ] {
            if let Some(segment) = option {
                child_rects.append(&mut segment.get_tree_rects());
            }
        }

        return child_rects;
    }

    // hide ugly types so making new segments is easier to read
    fn new_tree_segment(&self, origin: &Vec2, half_size: &Vec2) -> Option<Box<QuadTree>> {
        let new_boxed_tree = Box::new(Self::new_child_segment(
            *origin,
            *half_size,
            self.capacity,
            self.index + 1,
        ));
        return Some(new_boxed_tree);
    }

    fn subdivide_tree(&mut self) {
        // calculate size of new segment by getting a quarter of the parent size
        // because we need half the parent size
        // and half of that again to create new rect
        let h = self.rect.height() / 4.0;
        let w = self.rect.width() / 4.0;
        let half_size: Vec2 = Vec2::new(w, h);

        // parent origin
        let x = self.rect.center().x;
        let y = self.rect.center().y;

        // calculate origin point for each new section
        let ne_origin: Vec2 = Vec2::new(x - w, y + h);
        let nw_origin: Vec2 = Vec2::new(x + w, y + h);
        let se_origin: Vec2 = Vec2::new(x - w, y - h);
        let sw_origin: Vec2 = Vec2::new(x + w, y - h);

        // create new tree segments
        self.north_east = self.new_tree_segment(&ne_origin, &half_size);
        self.north_west = self.new_tree_segment(&nw_origin, &half_size);
        self.south_east = self.new_tree_segment(&se_origin, &half_size);
        self.south_west = self.new_tree_segment(&sw_origin, &half_size);

        // mark as subdivided
        self.subdivided = true;
    }
}
