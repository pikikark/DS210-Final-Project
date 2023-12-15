use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use petgraph::algo::{dijkstra, all_simple_paths};
use std::collections::HashMap;
use petgraph::visit::NodeIndexable;

pub fn compute_degree_centrality(graph: &DiGraph<String, ()>) -> HashMap<usize, usize> {
    let mut centrality = HashMap::new();
    for node in graph.node_indices() {
        centrality.insert(graph.to_index(node), graph.neighbors(node).count());
    }
    centrality
}

pub fn compute_betweenness_centrality(graph: &DiGraph<String, ()>) -> HashMap<usize, f64> {
    let mut centrality = HashMap::new();
    let node_count = graph.node_count();

    for node in graph.node_indices() {
        let mut node_centrality = 0.0;
        for start in graph.node_indices() {
            for end in graph.node_indices() {
                if start != end && node != start && node != end {
                    let all_paths: Vec<_> = all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None).collect();
                    let total_paths = all_paths.len() as f64;
                    let paths_through_node = all_paths.iter().filter(|path| path.contains(&node)).count() as f64;

                    if total_paths > 0.0 {
                        node_centrality += paths_through_node / total_paths;
                    }
                }
            }
        }
        centrality.insert(graph.to_index(node), node_centrality / (node_count - 1) as f64);
    }
    centrality
}

pub fn compute_closeness_centrality(graph: &DiGraph<String, ()>) -> HashMap<NodeIndex, f64> {
    let mut centrality_scores = HashMap::new();

    for start_node in graph.node_indices() {
        let mut total_distance = 0.0;
        let shortest_paths = dijkstra(graph, start_node, None, |_| 1.0);

        for end_node in graph.node_indices() {
            if start_node != end_node {
                if let Some(distance) = shortest_paths.get(&end_node) {
                    total_distance += *distance;
                } else {
                    // Assuming unreachable nodes contribute a distance of infinity.
                    total_distance += f64::INFINITY;
                }
            }
        }

        let centrality = if total_distance > 0.0 && total_distance != f64::INFINITY {
            1.0 / total_distance
        } else {
            0.0
        };

        centrality_scores.insert(start_node, centrality);
    }

    centrality_scores
}

#[cfg(test)]
mod tests {
    use super::*;
    //use petgraph::dot::{Dot, Config};
    use petgraph::graph::DiGraph;

    fn create_test_graph() -> DiGraph<String, ()> {
        let mut graph = DiGraph::new();
        let n1 = graph.add_node("Paper A".to_string());
        let n2 = graph.add_node("Paper B".to_string());
        let n3 = graph.add_node("Paper C".to_string());

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n1, ());
        graph
    }

    #[test]
    fn test_degree_centrality() {
        let graph = create_test_graph();
        let centrality = compute_degree_centrality(&graph);
        assert_eq!(centrality[&0], 1); // Assuming node indices start at 0
        assert_eq!(centrality[&1], 1);
        assert_eq!(centrality[&2], 1);
    }

    #[test]
    fn test_betweenness_centrality() {
        let graph = create_test_graph();
        let centrality = compute_betweenness_centrality(&graph);
        // Add assertions based on expected betweenness centrality values
    }

    #[test]
    fn test_closeness_centrality() {
        let graph = create_test_graph();
        let centrality = compute_closeness_centrality(&graph);
        // Add assertions based on expected closeness centrality values
    }
}