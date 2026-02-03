use petgraph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;

use crate::model::Protein;

pub struct NetworkStats {
    pub nodes: usize,
    pub edges: usize,
    pub avg_degree: f64,
}

pub fn network_stats(graph: &Graph<Protein, (), Undirected>) -> NetworkStats {
    let nodes = graph.node_count();
    let edges = graph.edge_count();

    let avg_degree = if nodes > 0 {
        (2.0 * edges as f64) / nodes as f64
    } else {
        0.0
    };

    NetworkStats {
        nodes,
        edges,
        avg_degree,
    }
}

pub fn node_degrees(graph: &Graph<Protein, (), Undirected>) -> HashMap<Protein, usize> {
    graph
        .node_indices()
        .map(|i| (graph[i].clone(), graph.neighbors(i).count()))
        .collect()
}
