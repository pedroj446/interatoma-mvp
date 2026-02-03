use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

use crate::io::InteractionRecord;
use crate::model::{InteractionType, Protein};

pub fn build_interactome(
    records: &[InteractionRecord],
    human_proteins: &[String],
) -> Graph<Protein, InteractionType, Undirected> {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<Protein, NodeIndex> = HashMap::new();
    let mut edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

    // 1️⃣ Adiciona TODAS as proteínas humanas primeiro
    for h in human_proteins {
        let protein = Protein::Human(h.clone());
        nodes
            .entry(protein.clone())
            .or_insert_with(|| graph.add_node(protein));
    }

    // 2️⃣ Adiciona interações virais
    for r in records {
        let viral = Protein::Viral {
            virus: r.virus.clone(),
            name: r.viral_protein.clone(),
        };

        let human = Protein::Human(r.host_protein.clone());

        let v_idx = *nodes
            .entry(viral.clone())
            .or_insert_with(|| graph.add_node(viral));

        let h_idx = *nodes
            .entry(human.clone())
            .or_insert_with(|| graph.add_node(human));

        let key = if v_idx < h_idx {
            (v_idx, h_idx)
        } else {
            (h_idx, v_idx)
        };

        if edges.insert(key) {
            graph.add_edge(v_idx, h_idx, InteractionType::VirusHost);
        }
    }

    graph
}

