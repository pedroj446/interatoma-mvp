use petgraph::Undirected;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};

use crate::io::InteractionRecord;
use crate::model::{InteractionType, Protein};

#[derive(Debug, Clone, Copy)]
pub struct BuildSummary {
    pub records_seen: usize,
    pub nodes_added_from_human_background: usize,
    pub nodes_added_from_interactions: usize,
    pub edges_added: usize,
    pub duplicate_edges_skipped: usize,
}

pub fn build_interactome(
    records: &[InteractionRecord],
    human_proteins: &[String],
) -> Graph<Protein, InteractionType, Undirected> {
    build_interactome_with_summary_from_records(records.iter(), human_proteins).0
}

pub fn build_interactome_from_records<'a>(
    records: impl IntoIterator<Item = &'a InteractionRecord>,
    human_proteins: &[String],
) -> Graph<Protein, InteractionType, Undirected> {
    build_interactome_with_summary_from_records(records, human_proteins).0
}

pub fn build_interactome_with_summary_from_records<'a>(
    records: impl IntoIterator<Item = &'a InteractionRecord>,
    human_proteins: &[String],
) -> (Graph<Protein, InteractionType, Undirected>, BuildSummary) {
    let mut graph = Graph::new_undirected();
    let mut nodes: HashMap<Protein, NodeIndex> = HashMap::new();
    let mut edges: HashSet<(NodeIndex, NodeIndex)> = HashSet::new();

    let mut summary = BuildSummary {
        records_seen: 0,
        nodes_added_from_human_background: 0,
        nodes_added_from_interactions: 0,
        edges_added: 0,
        duplicate_edges_skipped: 0,
    };

    for h in human_proteins {
        let protein = Protein::Human(h.clone());
        if let std::collections::hash_map::Entry::Vacant(entry) = nodes.entry(protein.clone()) {
            entry.insert(graph.add_node(protein));
            summary.nodes_added_from_human_background += 1;
        }
    }

    for r in records {
        summary.records_seen += 1;

        let viral = Protein::Viral {
            virus: r.virus.clone(),
            name: r.viral_protein.clone(),
        };

        let human = Protein::Human(r.host_protein.clone());

        let v_idx = *nodes.entry(viral.clone()).or_insert_with(|| {
            summary.nodes_added_from_interactions += 1;
            graph.add_node(viral)
        });

        let h_idx = *nodes.entry(human.clone()).or_insert_with(|| {
            summary.nodes_added_from_interactions += 1;
            graph.add_node(human)
        });

        let key = if v_idx < h_idx {
            (v_idx, h_idx)
        } else {
            (h_idx, v_idx)
        };

        if edges.insert(key) {
            graph.add_edge(v_idx, h_idx, InteractionType::VirusHost);
            summary.edges_added += 1;
        } else {
            summary.duplicate_edges_skipped += 1;
        }
    }

    (graph, summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_human() -> Vec<String> {
        vec!["H1".to_string(), "H2".to_string()]
    }

    #[test]
    fn deduplicates_duplicate_interactions() {
        let records = vec![
            InteractionRecord {
                virus: "V1".to_string(),
                viral_protein: "VP1".to_string(),
                host_protein: "H1".to_string(),
            },
            InteractionRecord {
                virus: "V1".to_string(),
                viral_protein: "VP1".to_string(),
                host_protein: "H1".to_string(),
            },
        ];

        let (g, s) = build_interactome_with_summary_from_records(records.iter(), &sample_human());
        assert_eq!(g.edge_count(), 1);
        assert_eq!(s.duplicate_edges_skipped, 1);
    }

    #[test]
    fn builds_from_iterator_without_intermediate_merge_vec() {
        let a = vec![InteractionRecord {
            virus: "V1".to_string(),
            viral_protein: "VP1".to_string(),
            host_protein: "H1".to_string(),
        }];
        let b = vec![InteractionRecord {
            virus: "V2".to_string(),
            viral_protein: "VP2".to_string(),
            host_protein: "H2".to_string(),
        }];

        let iter = a.iter().chain(b.iter());
        let g = build_interactome_from_records(iter, &sample_human());

        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.node_count(), 4);
    }
}
