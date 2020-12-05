use std::cell::RefCell;

#[derive(Eq, PartialEq)]
struct Vertex<'a, T: Eq + PartialEq> {
    id: usize,
    value: T,
    neighbors: Vec<&'a RefCell<Vertex<'a, T>>>,
}

impl<'a, T: Eq + PartialEq + Clone> Vertex<'a, T> {
    pub fn new(id: usize, value: T) -> RefCell<Vertex<'a, T>> {
        RefCell::new(Vertex {
            id,
            value,
            neighbors: Vec::new(),
        })
    }

    pub fn add_neighbor(&mut self, neighbor: &'a RefCell<Vertex<'a, T>>) -> bool {
        if !self.has_neighbor(neighbor) {
            self.neighbors.push(neighbor);
            return true;
        }
        false
    }

    pub fn has_neighbor(&self, neighbor: &RefCell<Vertex<'a, T>>) -> bool {
        self.neighbors.contains(&neighbor)
    }

    pub fn num_neighbors(&self) -> usize {
        self.neighbors.len()
    }

    pub fn traverse(&self) {
        let mut seen: Vec<usize> = Vec::new();
        seen.push(self.id);
    }
}

