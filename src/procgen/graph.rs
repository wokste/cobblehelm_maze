use crate::map::Coords;


#[derive(Copy,Clone)]
struct PrimData{
    inside : usize,
    outside : usize,
    cost : i32,
}

impl PrimData {
    fn new(graph : &Vec<Coords>, inside : usize, outside : usize) -> Self {
        Self {inside, outside, cost : graph[inside].manhattan_dist(graph[outside])}
    }
}

pub fn make_tree(graph : Vec<Coords>) -> Vec<(Coords,Coords)> {
    make_minimum_spanning_tree(graph)
}

pub fn make_minimum_spanning_tree(graph : Vec<Coords>) -> Vec<(Coords,Coords)> {
    // Using prims algorithm
    let mut found_edges = vec![];
    let mut unfound_data: Vec<PrimData> = vec![];

    for id in 1..graph.len() {
        unfound_data.push(PrimData::new(&graph, 0, id));
    }

    while let Some((id_to_remove, conn_edge)) = unfound_data.iter().enumerate().min_by_key(|(_, a)| a.cost)
    {
        // Add edge outside update return value
        found_edges.push((graph[conn_edge.inside], graph[conn_edge.outside]));
        let connected = conn_edge.outside;

        // Update unfound_data
        unfound_data.remove(id_to_remove);
        for test_edge in unfound_data.iter_mut() {
            let replace_edge = PrimData::new(&graph, connected, test_edge.outside);
            if replace_edge.cost < test_edge.cost {
                *test_edge = replace_edge;
            }
        }
    }

    found_edges
}