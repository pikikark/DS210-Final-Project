use petgraph::graph::DiGraph;
use std::collections::HashMap;

pub fn identify_clusters(graph: &DiGraph<String, ()>) -> HashMap<usize, Vec<usize>> {
    let mut clusters = HashMap::new();

    // Simple example logic: Group nodes by the number of their neighbors (degree)
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        clusters.entry(degree).or_insert_with(Vec::new).push(node.index());
    }

    clusters
}

pub fn analyze_cluster_coherence(cluster: &[usize], _graph: &DiGraph<String, ()>) -> f64 {
    // Simple example logic: Define coherence based on the size of the cluster
    cluster.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_cluster_identification() {
        let mut graph = DiGraph::<String, ()>::new();
        let n1 = graph.add_node("Paper A".to_string());
        let n2 = graph.add_node("Paper B".to_string());
        let n3 = graph.add_node("Paper C".to_string());
        
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        // Note: n3 is not connected to n1 to form two separate clusters

        let clusters = identify_clusters(&graph);
        assert!(!clusters.is_empty(), "No clusters identified");
        assert_eq!(clusters.len(), 2, "Expected two distinct clusters");
    }

    #[test]
    fn test_cluster_coherence() {
        let mut graph = DiGraph::<String, ()>::new();
        let n1 = graph.add_node("Paper A".to_string());
        let n2 = graph.add_node("Paper B".to_string());
        let n3 = graph.add_node("Paper C".to_string());

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());

        let clusters = identify_clusters(&graph);
        for (cluster_id, nodes) in clusters {
            let coherence = analyze_cluster_coherence(&nodes, &graph);
            assert!(coherence > 0.0, "Cluster {} has zero coherence", cluster_id);
        }
    }
}