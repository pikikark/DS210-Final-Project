use petgraph::dot::{Dot, Config};
use petgraph::graph::DiGraph;
use plotters::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn visualize_graph(graph: &DiGraph<String, ()>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    // Write the dot format to a file
    let dot_file_path = format!("{}.dot", file_path);
    let mut dot_file = File::create(&dot_file_path)?;
    writeln!(dot_file, "{:?}", dot)?;

    // Store the file path in a variable
    let image_file_path = format!("{}.png", file_path);

    // Use plotters to create an image file
    let root = BitMapBackend::new(&image_file_path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let font: FontDesc = ("sans-serif", 20).into();

    if let Ok(chart) = ChartBuilder::on(&root)
        .caption("Graph Visualization", font)
        .build_cartesian_2d(-100..100, -100..100) {
            // Here you can customize how you want to represent the graph
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;
    use petgraph::prelude::*;
    use std::fs;

    #[test]
    fn test_visualize_graph() {
        // Create a simple test graph
        let mut graph = DiGraph::new();
        let n1 = graph.add_node("Node 1".to_string());
        let n2 = graph.add_node("Node 2".to_string());
        graph.add_edge(n1, n2, ());

        // Call the visualization function
        let file_path = "test_graph_visualization";
        let result = visualize_graph(&graph, file_path);

        // Check if the function executed without errors
        assert!(result.is_ok(), "Visualization function should execute without errors.");

        // Check if the output file is created
        let png_path = format!("{}.png", file_path);
        assert!(fs::metadata(png_path).is_ok(), "Output file should be created.");
    }
}