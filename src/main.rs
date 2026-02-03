mod analysis;
mod cytoscape;
mod graph;
mod io;
mod model;

use cytoscape::send_to_cytoscape;
use graph::build_interactome;
use io::{read_human_proteins, read_interactions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧬 HTLV Interactome with Human Background\n");

    let human = read_human_proteins("data/human_proteins.csv")?;

    let htlv1 = read_interactions("data/htlv1_ppi.csv")?;
    let htlv2 = read_interactions("data/htlv2_ppi.csv")?;

    let g1 = build_interactome(&htlv1, &human);
    let g2 = build_interactome(&htlv2, &human);

    send_to_cytoscape(&g1, "HTLV-1 + Human Proteome")?;
    send_to_cytoscape(&g2, "HTLV-2 + Human Proteome")?;

    Ok(())
}

