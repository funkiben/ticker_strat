use std::cell::RefCell;
use std::rc::Rc;

#[derive(Eq, PartialEq)]
struct Vertex<T: Eq + PartialEq> {
    label: usize,
    value: T,
    neighbors: Vec<Rc<RefCell<Vertex<T>>>>,
}

impl<T: Eq + PartialEq> Vertex<T> {
    pub fn new(label: usize, value: T) -> Rc<RefCell<Vertex<T>>> {
        Rc::new(RefCell::new(Vertex {
            label,
            value,
            neighbors: Vec::new(),
        }))
    }

    pub fn add_neighbor(&mut self, neighbor: Rc<RefCell<Vertex<T>>>) -> bool {
        if !self.has_neighbor(&neighbor) {
            self.neighbors.push(neighbor);
            return true;
        }
        false
    }

    pub fn has_neighbor(&self, neighbor: &Rc<RefCell<Vertex<T>>>) -> bool {
        self.neighbors.contains(neighbor)
    }

    pub fn num_neighbors(&self) -> usize {
        self.neighbors.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_graph_setup() {
        let root = Vertex::new(0, "Hello");
        let neighbor = Vertex::new(1, "World");

        root.borrow_mut().add_neighbor(neighbor.clone());

        let neighbors = root.borrow().num_neighbors();
        assert_eq!(1, neighbors);

        let has_neighbor = root.borrow().has_neighbor(&neighbor);
        assert!(has_neighbor);
    }
}