use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Element {
    value: u32,
}

#[derive(Debug)]
struct Vec2D {
    vec_2d: Vec<Element>,
    col: usize,
}

#[derive(Debug)]
struct Vec2DNeighbors<'a> {
    vec_2d: HashMap<&'a Element, Option<[Option<&'a Element>; 8]>>,
}

impl Vec2D {
    fn new(col: usize) -> Self {
        assert!(col > 0);
        Vec2D {
            vec_2d: Vec::new(),
            col,
        }
    }

    fn add(&mut self, value: u32) {
        self.vec_2d.push(Element { value });
    }

    /// ElementNeighbor-containing-version of this grid
    fn neighbor_grid(&self) -> Vec2DNeighbors<'_> {
        let mut m = HashMap::new();
        for (i, e) in self.vec_2d.iter().enumerate() {
            m.insert(e, self.element_neighbors(i));
        }
        Vec2DNeighbors {
            vec_2d: m
        }
    }

    /// ElementNeighbors for the given index, or None if the index is beyond the grid
    fn element_neighbors(&self, index: usize) -> Option<[Option<&Element>; 8]> {
        let row_position = index % self.col;
        // The index of the element to the left, or None if we're on the left edge
        let left = if row_position == 0 {
            None
        } else {
            Some(index - 1)
        };

        // The index of the element to the right, or None of we're on the right edge
        let right = if row_position == self.col - 1 {
            None
        } else {
            Some(index + 1)
        };

        // Neighbors indices: 3 above, left, right, 3 below
        let neighbor_indices = [
            left.and_then(|i| i.checked_sub(self.col)),
            index.checked_sub(self.col),
            right.and_then(|i| i.checked_sub(self.col)),
            left,
            right,
            left.map(|i| i + self.col),
            Some(index + self.col),
            right.map(|i| i + self.col),
        ];

        // Resolve indices to references (None if they're beyond the grid)
        let neighbors = [
            neighbor_indices[0].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[1].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[2].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[3].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[4].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[5].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[6].and_then(|i| self.vec_2d.get(i)),
            neighbor_indices[7].and_then(|i| self.vec_2d.get(i)),
        ];

        Some(neighbors)
    }
}

fn main() {
    let mut vec_2d = Vec2D::new(5);
    let e1 = 1;
    let e2 = 2;

    vec_2d.add(1);
    vec_2d.add(2);
    vec_2d.add(3);
    /*vec_2d.add(4);
    vec_2d.add(5);
    vec_2d.add(6);
    vec_2d.add(7);
    vec_2d.add(8);
    vec_2d.add(9);
    vec_2d.add(10);
    vec_2d.add(11);
    vec_2d.add(12);*/

    println!("{:#?}", vec_2d);

    let with_neighbors = vec_2d.neighbor_grid();
    println!("{:#?}", with_neighbors);
}