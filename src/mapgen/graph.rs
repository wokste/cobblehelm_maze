use crate::grid::Coords;

struct Node {
    coords: Coords,
    edges: Vec<usize>,
}

#[derive(Copy, Clone)]
struct Edge {
    from: usize,
    to: usize,
    dist_sq: i32,
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<Node>,
}

impl Edge {
    fn new(graph: &Graph, from: usize, to: usize) -> Self {
        let dist_sq = graph.nodes[from]
            .coords
            .eucledian_dist_sq(graph.nodes[to].coords);
        Self { from, to, dist_sq }
    }
}

impl Graph {
    pub fn add_node(&mut self, coords: Coords) {
        self.nodes.push(Node {
            coords,
            edges: vec![],
        })
    }

    pub fn connect_tree(&mut self) {
        // Create minimum spanning tree using prims algorithm
        let mut unfound_data: Vec<Edge> = vec![];

        for id in 1..self.nodes.len() {
            unfound_data.push(Edge::new(self, 0, id));
        }

        while let Some((id_to_remove, conn_edge)) = unfound_data
            .iter()
            .enumerate()
            .min_by_key(|(_, a)| a.dist_sq)
        {
            // Add edge outside update return value
            self.connect(conn_edge.from, conn_edge.to);
            let connected = conn_edge.to;

            // Update unfound_data
            unfound_data.remove(id_to_remove);
            for test_edge in unfound_data.iter_mut() {
                let replace_edge = Edge::new(self, connected, test_edge.to);
                if replace_edge.dist_sq < test_edge.dist_sq {
                    *test_edge = replace_edge;
                }
            }
        }
    }

    fn connect(&mut self, from: usize, to: usize) {
        self.nodes[from].edges.push(to);
        self.nodes[to].edges.push(from);
    }

    pub fn add_more_edges(&mut self, rng: &mut fastrand::Rng, p_connect: f32) {
        let mut ids: Vec<usize> = self.nodes.iter().enumerate().map(|(i, _)| i).collect();
        rng.shuffle(ids.as_mut_slice());

        for id0 in ids {
            let n0 = &self.nodes[id0];

            if n0.edges.len() > 1 {
                continue;
            }

            if rng.f32() > p_connect {
                continue;
            }

            let Some((id1, _)) = self.nodes.iter().enumerate()
                .filter(
                    |(id1, _)| id0 != *id1 && !n0.edges.contains(id1)
                )
                .min_by_key(|
                    (_, n1)| n0.coords.eucledian_dist_sq(n1.coords)
                )
            else {continue;};

            // TODO: Do a distance check without the edge.

            self.connect(id0, id1);
        }
    }

    pub fn to_edges(&self) -> Vec<(Coords, Coords)> {
        let mut ret = vec![];
        for (id0, n0) in self.nodes.iter().enumerate() {
            for id1 in n0.edges.iter() {
                if id0 < *id1 {
                    continue;
                } // Don't need to print connections twice.

                ret.push((n0.coords, self.nodes[*id1].coords));
            }
        }
        ret
    }
}
