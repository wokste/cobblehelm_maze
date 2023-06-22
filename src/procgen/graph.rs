use crate::grid::Coords;

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

    fn to_hash(self) -> usize {
        let (v1, v2)= if self.from < self.to {(self.from, self.to)} else {(self.to, self.from)};
        (v1 << 32) + v2
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
            unfound_data.push(Edge::new(self, 0, id));
        }
    
        while let Some((id_to_remove, conn_edge)) = unfound_data.iter().enumerate().min_by_key(|(_, a)| a.dist_sq)
        {
            // Add edge outside update return value
           self.edges.push(*conn_edge);
            let connected = conn_edge.to;
    
            // Update unfound_data
            unfound_data.remove(id_to_remove);
            for test_edge in unfound_data.iter_mut() {
                let replace_edge = Edge::new(self, connected, test_edge.to);
                if replace_edge.dist_sq < test_edge.dist_sq {
                    *test_edge = replace_edge;
                }
            }
        };
    }

    pub fn add_more_edges(&mut self, rng : &mut fastrand::Rng, max_dist_sq : i32) {
        let mut map = bevy::utils::HashSet::<usize>::new();
        let node_len = self.nodes.len();

        for e in self.edges.iter() {
            map.insert(e.to_hash());
        }

        for _ in 0 .. 1000 {
            let n0 = rng.usize(0..node_len - 1);
            let n1 = rng.usize((n0+1) ..node_len);
            let new_edge = Edge::new(self, n0, n1);

            if map.contains(&new_edge.to_hash())  { continue; }
            if new_edge.dist_sq > max_dist_sq     { continue; }

            map.insert(new_edge.to_hash()); // Either it will be inserted or it is too expensive to calculate this over and over again.

            // TODO: Do a distance check without the door.

            self.edges.push(new_edge);

            // TODO: Exit after X cycles
        }
    }

    pub fn to_edges(&self) -> Vec<(Coords,Coords)> {
        self.edges.iter().map(|e| (self.nodes[e.from].coords, self.nodes[e.to].coords)).collect()
    }
}