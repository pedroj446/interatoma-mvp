use crate::graph::build_interactome_from_records;
use crate::io::InteractionRecord;
use std::time::Instant;

pub fn synthetic_benchmark(human_proteins: &[String], size: usize) {
    let records: Vec<InteractionRecord> = (0..size)
        .map(|i| InteractionRecord {
            virus: if i % 2 == 0 { "HTLV-1" } else { "HTLV-2" }.to_string(),
            viral_protein: format!("V{}", i % 500),
            host_protein: human_proteins[i % human_proteins.len()].clone(),
        })
        .collect();

    let start = Instant::now();
    let graph = build_interactome_from_records(records.iter(), human_proteins);
    let elapsed = start.elapsed();

    println!(
        "🧪 benchmark(size={}): nodes={}, edges={}, runtime={:?}",
        size,
        graph.node_count(),
        graph.edge_count(),
        elapsed
    );
}
