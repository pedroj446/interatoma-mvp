mod analysis;
mod audit;
mod bench;
mod cytoscape;
mod graph;
mod io;
mod model;

use analysis::network_stats;
use audit::{AuditWriter, LogLevel, Logger, generate_run_id};
use cytoscape::send_to_cytoscape;
use graph::{build_interactome, build_interactome_with_summary_from_records};
use io::{read_human_proteins, read_interactions};
use serde_json::json;
use std::env;
use std::time::Instant;

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn parse_bench_size(args: &[String]) -> Option<usize> {
    let prefix = "--bench-size=";
    args.iter()
        .find_map(|a| a.strip_prefix(prefix))
        .and_then(|v| v.parse::<usize>().ok())
}

fn parse_option_value<'a>(args: &'a [String], prefix: &str) -> Option<&'a str> {
    args.iter().find_map(|a| a.strip_prefix(prefix))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let headless = has_flag(&args, "--headless");
    let merged_mode = has_flag(&args, "--merge");
    let bench_size = parse_bench_size(&args);
    let log_level = parse_option_value(&args, "--log-level=")
        .and_then(LogLevel::parse)
        .unwrap_or(LogLevel::Info);
    let logger = Logger::new(log_level);

    let run_id = generate_run_id();
    let mut audit = if let Some(path) = parse_option_value(&args, "--audit-log=") {
        AuditWriter::enabled(path, run_id.clone())?
    } else {
        AuditWriter::disabled(run_id.clone())
    };

    logger.log(LogLevel::Info, "Iniciando execução do merge de interatoma");
    println!("🧬 HTLV Interactome with Human Background\n");

    let start = Instant::now();
    audit.emit(
        "run_started",
        json!({
            "run_id": audit.run_id(),
            "headless": headless,
            "merge_mode": merged_mode,
            "bench_size": bench_size,
            "log_level": log_level.as_str(),
        }),
    )?;

    let io_start = Instant::now();
    let human = read_human_proteins("data/human_proteins.csv")?;
    let htlv1 = read_interactions("data/htlv1_ppi.csv")?;
    let htlv2 = read_interactions("data/htlv2_ppi.csv")?;
    logger.log(LogLevel::Info, "Entrada carregada com sucesso");
    audit.emit(
        "input_loaded",
        json!({
            "human_proteins": human.len(),
            "htlv1_records": htlv1.len(),
            "htlv2_records": htlv2.len(),
            "duration_ms": io_start.elapsed().as_millis(),
        }),
    )?;

    if let Some(size) = bench_size {
        logger.log(LogLevel::Info, "Executando benchmark sintético");
        bench::synthetic_benchmark(&human, size);
        audit.emit(
            "benchmark_executed",
            json!({ "size": size, "duration_ms": start.elapsed().as_millis() }),
        )?;
    }

    if merged_mode {
        logger.log(LogLevel::Info, "Executando modo merge único");
        let merge_start = Instant::now();
        let (merged_graph, merge_summary) =
            build_interactome_with_summary_from_records(htlv1.iter().chain(htlv2.iter()), &human);

        let stats = network_stats(&merged_graph);
        println!(
            "📊 merged: nodes={}, edges={}, avg_degree={:.2}",
            stats.nodes, stats.edges, stats.avg_degree
        );

        audit.emit(
            "merge_finished",
            json!({
                "records_seen": merge_summary.records_seen,
                "nodes_added_from_human_background": merge_summary.nodes_added_from_human_background,
                "nodes_added_from_interactions": merge_summary.nodes_added_from_interactions,
                "edges_added": merge_summary.edges_added,
                "duplicate_edges_skipped": merge_summary.duplicate_edges_skipped,
                "nodes_total": stats.nodes,
                "edges_total": stats.edges,
                "duration_ms": merge_start.elapsed().as_millis(),
            }),
        )?;

        if !headless {
            send_to_cytoscape(&merged_graph, "HTLV-1 + HTLV-2 + Human Proteome")?;
            audit.emit(
                "cytoscape_exported",
                json!({ "network_name": "HTLV-1 + HTLV-2 + Human Proteome" }),
            )?;
        }
    } else {
        logger.log(LogLevel::Info, "Executando redes separadas por vírus");
        let g1 = build_interactome(&htlv1, &human);
        let g2 = build_interactome(&htlv2, &human);

        let s1 = network_stats(&g1);
        let s2 = network_stats(&g2);
        println!(
            "📊 HTLV-1: nodes={}, edges={}, avg_degree={:.2}",
            s1.nodes, s1.edges, s1.avg_degree
        );
        println!(
            "📊 HTLV-2: nodes={}, edges={}, avg_degree={:.2}",
            s2.nodes, s2.edges, s2.avg_degree
        );

        audit.emit(
            "separate_networks_finished",
            json!({
                "htlv1_nodes": s1.nodes,
                "htlv1_edges": s1.edges,
                "htlv2_nodes": s2.nodes,
                "htlv2_edges": s2.edges,
            }),
        )?;

        if !headless {
            send_to_cytoscape(&g1, "HTLV-1 + Human Proteome")?;
            send_to_cytoscape(&g2, "HTLV-2 + Human Proteome")?;
            audit.emit(
                "cytoscape_exported",
                json!({ "networks": ["HTLV-1 + Human Proteome", "HTLV-2 + Human Proteome"] }),
            )?;
        }
    }

    let runtime = start.elapsed();
    println!("⏱️ runtime total: {:?}", runtime);
    logger.log(LogLevel::Info, "Execução finalizada");
    audit.emit(
        "run_finished",
        json!({
            "duration_ms": runtime.as_millis(),
            "status": "ok"
        }),
    )?;

    Ok(())
}
