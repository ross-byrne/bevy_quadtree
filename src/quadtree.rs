use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone)]
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
        return Self::new_segment(origin, half_size, capacity, 0);
    }

    fn new_segment(origin: Vec2, size: Vec2, capacity: usize, index: usize) -> Self {
        return Self {
            rect: Rect::from_center_size(origin, size),
            capacity,
            index,
            subdivided: false,
            children: Vec::with_capacity(capacity),
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
        };
    }

    fn child_intersects(&self, point: &Vec2) -> bool {
        return self.rect.contains(*point);
    }

    fn insert_into_subdivision(&mut self, child: &TreeNode) -> bool {
        assert!(
            self.subdivided,
            "Cannot add child to subdivision if not subdivided!"
        );

        // get segments
        let (ne, nw, se, sw) = self.get_segments_as_mut();

        // only insert into one segment and return
        let result = ne.insert(child) || nw.insert(child) || se.insert(child) || sw.insert(child);
        assert!(result, "Failed to add child to a segment!");

        return result;
    }

    pub fn insert(&mut self, child: &TreeNode) -> bool {
        // return false if child doesn't intersect
        if !self.child_intersects(&child.position) {
            return false;
        }

        // check if tree is subdivided (if it's a leaf node)
        if self.subdivided {
            // add child to subdivision
            return self.insert_into_subdivision(child);
        }

        // if not divided, check if node has capacity
        if self.children.len() < self.capacity {
            self.children.push(child.to_owned());
            return true;
        }

        // if no capacity, subdivide
        self.subdivide_tree();

        // add child to subdivision
        return self.insert_into_subdivision(child);
    }

    pub fn query<'a>(&self, range: &Rect, count: &'a mut i32) -> Vec<&TreeNode> {
        // check if range is contained within current segment
        let intersection = self.rect.intersect(*range);
        if intersection.is_empty() {
            return Vec::new();
        }

        let mut result: Vec<&TreeNode> = Vec::new();

        // test if children are inside capture rect
        for child in self.children.as_slice() {
            *count += 1; // count comparisons
            if range.contains(child.position) {
                result.push(child);
            }
        }

        // no subdivisions, return result
        if !self.subdivided {
            return result;
        }

        // check subdivisions
        let (ne, nw, se, sw) = self.get_segments_as_ref();

        // query subdivisions with range
        let child_list: &[&TreeNode] = &[
            ne.query(range, count),
            nw.query(range, count),
            se.query(range, count),
            sw.query(range, count),
        ]
        .concat();

        // collect into result
        for child in child_list {
            result.push(child);
        }

        return result;
    }

    pub fn get_childen(&self) -> Vec<&TreeNode> {
        // get children as ref
        let mut result = self.children.iter().collect::<Vec<&TreeNode>>();

        // no sub divisions, return early
        if !self.subdivided {
            return result;
        }

        // get segments
        let (ne, nw, se, sw) = self.get_segments_as_ref();

        // add children of subdivions to return result
        let child_list: &[&TreeNode] = &[
            ne.get_childen(),
            nw.get_childen(),
            se.get_childen(),
            sw.get_childen(),
        ]
        .concat();

        for child in child_list {
            result.push(child);
        }

        return result;
    }

    // Just for display purposes
    pub fn get_tree_rects(&self) -> Vec<&Rect> {
        if !self.subdivided {
            return vec![&self.rect];
        }

        // get segments
        let (ne, nw, se, sw) = self.get_segments_as_ref();

        let mut result: Vec<&Rect> = vec![];

        // get references to all rects
        for child_list in [
            &[&self.rect],
            ne.get_tree_rects().as_slice(),
            nw.get_tree_rects().as_slice(),
            se.get_tree_rects().as_slice(),
            sw.get_tree_rects().as_slice(),
        ] {
            for child in child_list {
                result.push(*child);
            }
        }

        return result;
    }

    // hide ugly types so making new segments is easier to read
    fn new_tree_segment(&self, origin: &Vec2, half_size: &Vec2) -> Option<Box<QuadTree>> {
        let new_boxed_tree = Box::new(Self::new_segment(
            *origin,
            *half_size,
            self.capacity,
            self.index + 1,
        ));
        return Some(new_boxed_tree);
    }

    fn subdivide_tree(&mut self) {
        // calculate size of new segment by getting a half of the parent size
        let h = self.rect.height() / 2.0;
        let w = self.rect.width() / 2.0;
        let size: Vec2 = Vec2::new(w, h);

        // parent origin
        let x = self.rect.center().x;
        let y = self.rect.center().y;

        // calculate origin point for each new section
        let ne_origin: Vec2 = Vec2::new(x - (w / 2.0), y + (h / 2.0));
        let nw_origin: Vec2 = Vec2::new(x + (w / 2.0), y + (h / 2.0));
        let se_origin: Vec2 = Vec2::new(x - (w / 2.0), y - (h / 2.0));
        let sw_origin: Vec2 = Vec2::new(x + (w / 2.0), y - (h / 2.0));

        // create new tree segments
        self.north_east = self.new_tree_segment(&ne_origin, &size);
        self.north_west = self.new_tree_segment(&nw_origin, &size);
        self.south_east = self.new_tree_segment(&se_origin, &size);
        self.south_west = self.new_tree_segment(&sw_origin, &size);

        // mark as subdivided
        self.subdivided = true;

        // reparent children to new leaf nodes
        self.reparent_children();
    }

    fn reparent_children(&mut self) {
        assert!(
            self.subdivided,
            "Cannot reparent children if not subdivided!"
        );

        // take a copy of children and insert them back into tree
        // this should add them to new leaf nodes
        // TODO: update this so a copy isn't needed
        for child in self.children.clone() {
            self.insert(&child.to_owned());
        }

        // remove children from this level of the tree
        self.children.clear();
    }

    fn get_segments_as_ref(&self) -> (&QuadTree, &QuadTree, &QuadTree, &QuadTree) {
        assert!(
            self.subdivided,
            "Error! Cannot get segments before subdividing tree!"
        );

        return (
            self.north_east.as_ref().unwrap(),
            self.north_west.as_ref().unwrap(),
            self.south_east.as_ref().unwrap(),
            self.south_west.as_ref().unwrap(),
        );
    }

    fn get_segments_as_mut(
        &mut self,
    ) -> (&mut QuadTree, &mut QuadTree, &mut QuadTree, &mut QuadTree) {
        assert!(
            self.subdivided,
            "Error! Cannot get segments before subdividing tree!"
        );

        return (
            self.north_east.as_mut().unwrap(),
            self.north_west.as_mut().unwrap(),
            self.south_east.as_mut().unwrap(),
            self.south_west.as_mut().unwrap(),
        );
    }
}
