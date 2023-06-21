use crate::map::Coords;

struct Node {
    coords : Coords,
}

#[derive(Copy,Clone)]
struct Edge {
    from : usize,
    to : usize,
    dist_sq : i32,
}

#[derive(Default)]
pub struct Graph{
    nodes : Vec<Node>,
    edges : Vec<Edge>,
}

impl Edge {
    fn new(graph : &Graph, from : usize, to : usize) -> Self {
        let dist_sq = graph.nodes[from].coords.eucledian_dist_sq(graph.nodes[to].coords);
        Self {from, to, dist_sq}
    }
}

impl Graph {
    pub fn add_node(&mut self, coords : Coords) {
        self.nodes.push(Node{coords})
    }
    
    pub fn connect_tree(&mut self) {
        // Using prims algorithm
        let mut unfound_data: Vec<Edge> = vec![];
    
        for id in 1..self.nodes.len() {
            unfound_data.push(Edge::new(&self, 0, id));
        }
    
        while let Some((id_to_remove, conn_edge)) = unfound_data.iter().enumerate().min_by_key(|(_, a)| a.dist_sq)
        {
            // Add edge outside update return value
           self.edges.push(conn_edge.clone());
            let connected = conn_edge.to;
    
            // Update unfound_data
            unfound_data.remove(id_to_remove);
            for test_edge in unfound_data.iter_mut() {
                let replace_edge = Edge::new(&self, connected, test_edge.to);
                if replace_edge.dist_sq < test_edge.dist_sq {
                    *test_edge = replace_edge;
                }
            }
        };
    }

    pub fn add_more_edges(&mut self) {
        // TODO: Implement
    }

    pub fn shuffle_edges(&mut self) {
        fastrand::shuffle(self.edges.as_mut_slice());
    }

    pub fn to_edges(&self) -> Vec<(Coords,Coords)> {
        self.edges.iter().map(|e| (self.nodes[e.from].coords, self.nodes[e.to].coords)).collect()
    }
}