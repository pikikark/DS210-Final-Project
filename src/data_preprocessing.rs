use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use rand::{thread_rng, Rng};
use serde_json::{self, Error as SerdeError};

#[derive(Debug, Deserialize)]
struct Paper {
    id: u64,
    authors: Option<Vec<Author>>,
    title: String,
    year: u64,
    n_citation: u64,
    references: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    org: Option<String>,
    id: Option<u64>,
}

pub fn build_citation_graph(file_path: &str, sample_size: usize) -> Result<DiGraph<String, ()>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = DiGraph::new();
    let mut paper_indices: HashMap<u64, NodeIndex> = HashMap::new();

    // Parse the entire file as a JSON array of Papers
    let papers: Vec<Paper> = serde_json::from_reader(reader)?;

    for paper in papers {
        // Check if the paper has authors and references
        if paper.authors.is_none() || paper.references.is_none() {
            continue;
        }

        let paper_index = *paper_indices.entry(paper.id)
            .or_insert_with(|| graph.add_node(paper.title.clone()));
        
        for &referenced_paper_id in paper.references.unwrap().iter() {
            let referenced_index = *paper_indices.entry(referenced_paper_id)
                .or_insert_with(|| graph.add_node(referenced_paper_id.to_string()));
            graph.add_edge(paper_index, referenced_index, ());
        }
    }

    println!("Graph has {} nodes and {} edges", graph.node_count(), graph.edge_count());
    Ok(graph)
}

//~20mins to run, processes 4,894,000 papers. "Graph has 4,894,063 nodes and 45,564,149 edges".