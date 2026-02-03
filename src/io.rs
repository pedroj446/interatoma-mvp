use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct InteractionRecord {
    pub virus: String,
    pub viral_protein: String,
    pub host_protein: String,
}

#[derive(Debug, Deserialize)]
pub struct HumanProteinRecord {
    pub protein: String,
}

pub fn read_interactions(path: &str) -> Result<Vec<InteractionRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut records = Vec::new();

    for result in reader.deserialize() {
        records.push(result?);
    }

    Ok(records)
}

pub fn read_human_proteins(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut proteins = Vec::new();

    for result in reader.deserialize::<HumanProteinRecord>() {
        proteins.push(result?.protein);
    }

    Ok(proteins)
}
