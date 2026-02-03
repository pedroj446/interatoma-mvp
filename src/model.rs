#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Protein {
    Human(String),
    Viral { virus: String, name: String },
}

#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    VirusHost,
}

impl Protein {
    pub fn label(&self) -> &str {
        match self {
            Protein::Human(name) => name,
            Protein::Viral { name, .. } => name,
        }
    }

    pub fn cytoscape_id(&self) -> String {
        match self {
            Protein::Human(name) => format!("HUMAN::{}", name),
            Protein::Viral { virus, name } => format!("{}::{}", virus, name),
        }
    }

    pub fn node_type(&self) -> &'static str {
        match self {
            Protein::Human(_) => "human",
            Protein::Viral { .. } => "viral",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Protein::Human(_) => "#1f77b4", // azul
            Protein::Viral { .. } => "#d62728", // vermelho
        }
    }
}
