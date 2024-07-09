use bevy::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Point(pub Vec2);

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        return Self(Vec2::new(x, y));
    }
}

// Need to use box here becuase struct is recursive
#[derive(Debug, Default)]
pub struct QuadTree<T> {
    pub children: Vec<T>,
    pub subdivided: bool,
    capacity: usize,
    north_east: Option<Box<QuadTree<T>>>,
    north_west: Option<Box<QuadTree<T>>>,
    south_east: Option<Box<QuadTree<T>>>,
    south_west: Option<Box<QuadTree<T>>>,
}

impl<T> QuadTree<T> {
    pub fn new(capacity: usize) -> Self {
        return Self {
            capacity,
            subdivided: false,
            children: vec![],
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
        };
    }

    pub fn add_child(&mut self, child: T) {
        if self.children.len() < self.capacity {
            self.children.push(child);
            return;
        }

        // otherwise, subdivide quad tree
        self.subdivide_tree()

        // somehow figure out where to put child???
    }

    pub fn get_childen(&mut self) -> &[T] {
        if !self.subdivided {
            return self.children.as_slice();
        }

        return self.children.as_slice();
    }

    fn subdivide_tree(&mut self) {
        let new_boxed_tree = Box::new(Self::new(self.capacity));
        self.north_east = Some(new_boxed_tree);

        let new_boxed_tree = Box::new(Self::new(self.capacity));
        self.north_west = Some(new_boxed_tree);

        let new_boxed_tree = Box::new(Self::new(self.capacity));
        self.south_east = Some(new_boxed_tree);

        let new_boxed_tree = Box::new(Self::new(self.capacity));
        self.south_west = Some(new_boxed_tree);

        // make as subdivided
        self.subdivided = true;
    }
}
