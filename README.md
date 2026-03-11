```markdown
# Interactome Reconstruction Pipeline  
### Comparative Analysis of HTLV-1 and HTLV-2 Virus–Host Interactomes

This repository contains a prototype computational pipeline for reconstructing and analyzing **virus–host protein interaction networks** using public biological databases.

The project is part of a research initiative focused on understanding how differences in virus–host interaction networks may explain the distinct pathogenic profiles of **HTLV-1** and **HTLV-2**.

---

# Scientific Background

Human T-cell lymphotropic viruses type 1 and 2 (**HTLV-1** and **HTLV-2**) are closely related retroviruses with markedly different clinical outcomes.

**HTLV-1** is strongly associated with diseases such as:

- Adult T-cell leukemia/lymphoma (ATLL)
- HTLV-1 associated myelopathy (HAM/TSP)

In contrast, **HTLV-2** is rarely associated with oncogenic processes.

Despite their genomic similarity, the molecular mechanisms underlying this difference remain incompletely understood.

A promising systems biology approach to investigate this problem is the reconstruction of **virus–host interactomes**, which represent protein–protein interaction networks between viral proteins and host cellular proteins.

Analyzing these networks can reveal:

- highly connected proteins (network hubs)
- functional modules
- signaling pathways affected by viral proteins

Differences in network topology and functional enrichment may help explain the divergent pathogenic behavior of these viruses.

---

# Project Goal

The objective of this project is to develop a **reproducible computational pipeline** capable of:

- reconstructing virus–host interaction networks
- integrating multiple public interaction databases
- filtering interactions based on evidence
- generating standardized graph outputs
- enabling comparative network analysis

Although the initial application focuses on **HTLV-1 and HTLV-2**, the pipeline is designed to be **generic and reusable for other pathogens**.

---

# Features

Current prototype capabilities include:

- Parsing interaction datasets from public databases
- Integration of multiple interaction sources
- Interaction filtering based on evidence scores
- Deduplication of redundant interactions
- Normalization of protein identifiers (UniProt)
- Construction of virus–host interaction graphs
- Export of networks to standard formats

Planned analytical capabilities:

- Network topology analysis
- Centrality metrics
- Community detection
- Functional enrichment analysis
- Comparative network statistics

---

# Data Sources

The pipeline integrates interaction data from the following public biological databases:

- **STRING**
- **IntAct**
- **BioGRID**
- **UniProt**

These databases provide protein–protein interaction information and functional annotations necessary for constructing virus–host networks.

---

# Technology Stack

The software is implemented in **Rust**, chosen for:

- high computational performance
- memory safety
- reproducibility
- efficient processing of graph data

Downstream network analysis and visualization can be performed using tools such as:

- **Cytoscape**
- **Python** network analysis libraries
- **R** packages for graph statistics

---

# Repository Structure

```

interatoma-mvp/

src/                Rust source code
data/               Input datasets
output/             Generated network files
scripts/            Auxiliary scripts
Cargo.toml          Rust dependencies
README.md           Project documentation

````

---

# Example Workflow

1. Download interaction datasets from supported databases

2. Normalize protein identifiers using UniProt accessions

3. Filter interactions based on evidence level or confidence score

4. Construct virus–host interaction networks

5. Export graphs for downstream analysis

Supported output formats include:

- CSV
- JSON
- GraphML

These formats allow direct import into **Cytoscape** for visualization and further analysis.

---

# Installation

## Prerequisites

Install Rust:

https://www.rust-lang.org/tools/install

## Clone the repository

```bash
git clone https://github.com/pedroj446/interatoma-mvp.git
cd interatoma-mvp
````

## Build the project

```bash
cargo build --release
```

## Run the pipeline

```bash
cargo run
```

---

# Future Development

Planned improvements include:

* automated download of interaction databases
* configurable filtering parameters
* statistical comparison between interactomes
* integration with enrichment analysis tools
* support for additional viruses and pathogens
* containerized execution environment

---

# Reproducibility

All datasets used in this project are publicly available.
The objective is to provide a **fully reproducible computational workflow** for virus–host interactome reconstruction and analysis.

---


# License

MIT License

```

3. **a estrutura ideal de repositório para dissertação + artigo científico**.
```
