use std::cell::RefCell;
use std::rc::Rc;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum GraphError {
    VertexNotFound(usize),
    InvalidEdge(usize, usize)
}

impl Error for GraphError {}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphError::VertexNotFound(label) => {
                write!(f, "Vertex with id {} not found in graph.", label)
            },
            GraphError::InvalidEdge(source, sink) => {
                write!(f, "Edge from vertex with id {} to vertex with id {} is not valid.", source, sink)
            }
        }
    }
}

struct Graph<T: Eq + PartialEq> {
    vertices: Vec<Rc<RefCell<Vertex<T>>>>,
    available_labels: Vec<usize>
}

impl<T: Eq + PartialEq> Graph<T> {

    // returns a new empty graph
    pub fn new() -> Graph<T>{
        Graph {
            vertices: Vec::new(),
            available_labels: Vec::new()
        }
    }

    // returns the next available label
    fn get_label(&mut self) -> usize {
        self.available_labels.pop().unwrap_or(self.vertices.len())
    }

    // returns the number of vertices in the graph
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    // returns the number of edges in the graph
    pub fn num_edges(&self) -> usize {
        let mut sum = 0;
        for vertex in &self.vertices {
            sum += vertex.borrow().num_neighbors();
        }
        sum
    }

    // adds a new vertex with the given value and returns its label
    pub fn add_vertex(&mut self, value: T) -> usize {

        // get next label
        let label = self.get_label();

        // make vertex
        self.vertices.push(Rc::new(RefCell::new(Vertex {
            label,
            value,
            neighbors: Vec::new(),
        })));

        label
    }

    // get vertex with the given label
    pub fn get_vertex(&self, label: &usize) -> Option<&Rc<RefCell<Vertex<T>>>>{
        for vertex in &self.vertices {
            if vertex.borrow().label == *label {
                return Some(vertex)
            }
        }
        None
    }

    // returns true if the graph has a vertext with the given label
    pub fn has_vertex(&self, label: &usize) -> bool {
        for vertex in &self.vertices {
            if vertex.borrow().label == *label {
                return true
            }
        }
        false
    }

    // removes the vertex with the given label from the graph and returns its value
    pub fn delete_vertex(&mut self, label: &usize) -> Result<T, GraphError> {
        for vertex in &self.vertices {
            vertex.borrow_mut().delete_neighbor(label);
        }

        for index in 0..self.vertices.len() {

            let vertex = self.vertices.get(index).unwrap();

            if vertex.borrow().label == *label {

                let value = self.vertices.remove(index);
                self.available_labels.push(*label);
                return Ok(Rc::try_unwrap(value).ok().unwrap().into_inner().value)

            }
        }
        Err(GraphError::VertexNotFound(*label))
    }

    pub fn add_edge(&self, label_source: &usize, label_sink: usize) -> Result<bool, GraphError> {

        if label_sink == *label_source {
            return Err(GraphError::InvalidEdge(*label_source, label_sink))
        }

        if !self.has_vertex(&label_sink) {
            return Err(GraphError::VertexNotFound(label_sink))
        }

        let vertex = self.get_vertex(label_source);
        if vertex.is_some() {
            let mut neighbors = vertex.unwrap().borrow_mut();
            if !neighbors.has_neighbor(&label_sink) {
                neighbors.add_neighbor(label_sink);
                return Ok(true)
            }
            return Ok(false)
        }
        Err(GraphError::VertexNotFound(*label_source))
    }

    pub fn has_edge(&self, label_source: &usize, label_sink: &usize) -> bool {
        let vertex = self.get_vertex(label_source);
        if vertex.is_some() {
            return vertex.unwrap().borrow().has_neighbor(label_sink)
        }
        false
    }

    pub fn delete_edge(&self, label_source: &usize, label_sink: &usize) {
        let vertex = self.get_vertex(label_source);
        if vertex.is_some() {
            let mut neighbors = vertex.unwrap().borrow_mut();
            neighbors.delete_neighbor(label_sink);
        }
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = Vec::new();
        let mut rec_stack = Vec::new();
        for vertex in &self.vertices {
            let current_label = vertex.borrow().label.clone();
            if !visited.contains(&current_label) {
                if self.has_cycle_util(current_label, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        false
    }

    fn has_cycle_util(&self, current_label: usize, visited: &mut Vec<usize>, rec_stack: &mut Vec<usize>) -> bool {
        visited.push(current_label);
        rec_stack.push(current_label);

        for neighbor in &self.get_vertex(&current_label).unwrap().borrow().neighbors {
            if !visited.contains(neighbor) {
                if self.has_cycle_util(*neighbor, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(neighbor) {
                return true;
            }
        }

        for i in 0..rec_stack.len() {
            if *rec_stack.get(i).unwrap() == current_label {
                rec_stack.remove(i);
            }
        }
        false
    }
}

#[derive(Eq, PartialEq)]
struct Vertex<T: Eq + PartialEq> {
    label: usize,
    value: T,
    neighbors: Vec<usize>,
}

impl<T: Eq + PartialEq> Vertex<T> {

    // add a neighbor to the vertex
    // returns true if added, false if the neighbor was already a neighbor
    fn add_neighbor(&mut self, label: usize) -> bool {
        if !self.has_neighbor(&label) {
            self.neighbors.push(label);
            return true
        }
        false
    }

    // returns true if the vertex has a given neighbor
    fn has_neighbor(&self, neighbor: &usize) -> bool {
        self.neighbors.contains(neighbor)
    }

    // deletes the given neighbor of the vertex
    // returns true if the neighbor was removed, false if it was not a neighbor originally
    fn delete_neighbor(&mut self, neighbor: &usize) -> bool {
        for i in 0..self.neighbors.len() {
            if self.neighbors.get(i).unwrap() == neighbor {
                self.neighbors.remove(i);
                return true
            }
        }
        false
    }

    // returns the number of neighbors of the vertex
    fn num_neighbors(&self) -> usize {
        self.neighbors.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph: Graph<&str> = Graph::new();
        assert_eq!(0, graph.num_vertices());
        assert_eq!(0, graph.num_edges());
        assert_eq!(false, graph.has_cycle())
    }

    #[test]
    fn test_basic_graph() -> Result<(), GraphError>{
        let mut graph = Graph::new();

        let hello  = graph.add_vertex("Hello");
        assert_eq!(1, graph.num_vertices());
        assert!(graph.has_vertex(&hello));
        assert_eq!("Hello", graph.get_vertex(&hello).unwrap().borrow().value);

        let world = graph.add_vertex("World");
        graph.add_edge(&hello, world);
        assert_eq!(1, graph.num_edges());
        assert!(graph.has_edge(&hello, &world));
        graph.delete_edge(&hello, &world);
        assert_eq!(false, graph.has_edge(&hello, &world));
        assert_eq!(0, graph.num_edges());

        let delete = graph.add_vertex("nope");
        graph.add_edge(&world, delete);
        let deleted_value = graph.delete_vertex(&delete);
        assert_eq!("nope", deleted_value?);
        assert_eq!(2, graph.num_vertices());
        assert_eq!(false, graph.has_vertex(&delete));
        Ok(())
    }

    #[test]
    fn test_has_cycle() {
        let mut graph = Graph::new();
        assert_eq!(false, graph.has_cycle());

        let hello  = graph.add_vertex("Hello");
        assert_eq!(false, graph.has_cycle());

        let world = graph.add_vertex("World");
        graph.add_edge(&hello, world);
        assert_eq!(false, graph.has_cycle());

        graph.add_edge(&world, hello);
        assert_eq!(true, graph.has_cycle());

        graph.delete_edge(&world, &hello);
        assert_eq!(false, graph.has_cycle());

        let something = graph.add_vertex("Something");
        let something_else = graph.add_vertex("Else");

        graph.add_edge(&world, something);
        graph.add_edge(&something, something_else);

        graph.add_edge(&something_else, world);
        assert_eq!(true, graph.has_cycle());
    }
}
