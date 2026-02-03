use petgraph::visit::{EdgeRef, IntoNodeReferences};
use petgraph::{Graph, Undirected};
use serde_json::json;
use std::error::Error;

use crate::model::Protein;

const CYTOSCAPE_BASE: &str = "http://localhost:1234/v1";

/// Envia um grafo para o Cytoscape e aplica cores via Visual Style
pub fn send_to_cytoscape<E>(
    graph: &Graph<Protein, E, Undirected>,
    network_name: &str,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    // ----------------------------
    // NODES
    // ----------------------------
    let nodes: Vec<_> = graph
        .node_references()
        .map(|(_, protein)| {
            json!({
                "data": {
                    "id": protein.cytoscape_id(),
                    "label": protein.label(),
                    "type": protein.node_type()
                }
            })
        })
        .collect();

    // ----------------------------
    // EDGES
    // ----------------------------
    let edges: Vec<_> = graph
        .edge_references()
        .map(|edge| {
            let src = &graph[edge.source()];
            let tgt = &graph[edge.target()];

            json!({
                "data": {
                    "source": src.cytoscape_id(),
                    "target": tgt.cytoscape_id()
                }
            })
        })
        .collect();

    // ----------------------------
    // NETWORK PAYLOAD
    // ----------------------------
    let payload = json!({
        "data": { "name": network_name },
        "elements": { "nodes": nodes, "edges": edges }
    });

    // ----------------------------
    // CREATE NETWORK
    // ----------------------------
    let response = client
        .post(format!("{CYTOSCAPE_BASE}/networks"))
        .json(&payload)
        .send()?
        .error_for_status()?
        .json::<serde_json::Value>()?;

    let network_suid = response["networkSUID"]
        .as_i64()
        .ok_or("Network SUID not found")?;

    println!("✅ Network '{}' created with SUID {}", network_name, network_suid);

    // ----------------------------
    // CREATE VISUAL STYLE
    // ----------------------------
    let style_name = format!("{}_style", network_name);

    let style = json!({
        "title": style_name,
        "defaults": [
            { "visualProperty": "NODE_SHAPE", "value": "ELLIPSE" },
            { "visualProperty": "NODE_SIZE", "value": 35 },
            { "visualProperty": "EDGE_WIDTH", "value": 2 }
        ],
        "mappings": [
            {
                "mappingType": "discrete",
                "mappingColumn": "type",
                "mappingColumnType": "String",
                "visualProperty": "NODE_FILL_COLOR",
                "map": [
                    { "key": "human", "value": "#1f77b4" },  // Azul
                    { "key": "viral", "value": "#d62728" }   // Vermelho
                ]
            }
        ]
    });

    client
        .post(format!("{CYTOSCAPE_BASE}/styles"))
        .json(&style)
        .send()?
        .error_for_status()?;

    // ----------------------------
    // APPLY STYLE
    // ----------------------------
    client
        .get(format!(
            "{CYTOSCAPE_BASE}/apply/styles/{}/{}",
            style_name, network_suid
        ))
        .send()?
        .error_for_status()?;

    println!("🎨 Style '{}' applied successfully", style_name);

    Ok(())
}

