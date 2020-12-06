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

    fn traverse<F>(&self, f: &F, seen: &mut Vec<usize>)
        where F: Fn(&T)
    {
        if seen.contains(&self.label) {
            return;
        }
        f(&self.value);
        seen.push(self.label);
        for n in &self.neighbors {
            n.borrow().traverse(f, seen);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_graph_setup() {
        let root = Vertex::new(0, "Hello");
        let neighbor_one = Vertex::new(1, "World");
        let neighbor_two = Vertex::new(2, "!");

        let mut mut_root = root.borrow_mut();
        mut_root.add_neighbor(neighbor_one.clone());
        mut_root.add_neighbor(neighbor_two.clone());

        assert_eq!(2, mut_root.num_neighbors());
        assert!(mut_root.has_neighbor(&neighbor_one));
        assert!(mut_root.has_neighbor(&neighbor_two));

        mut_root.traverse(&|d| println!("{}", d), &mut Vec::new());
    }
}