mod data_preprocessing;
mod centrality_analysis;
mod clustering;
mod visualization;
mod utilities;

fn main() {
    let result = utilities::common_utility_function();

    let graph = data_preprocessing::build_citation_graph("D:/DS210_FinalProject/dblp.v12.json", 1000)
    .expect("Failed to build graph");

    // Compute and display degree centrality
    let degree_centrality = centrality_analysis::compute_degree_centrality(&graph);
    // Here you can add code to process and display degree centrality results

    // Compute and display betweenness centrality
    //let betweenness_centrality = centrality_analysis::compute_betweenness_centrality(&graph);

    // Compute and display closeness centrality
    let closeness_centrality = centrality_analysis::compute_closeness_centrality(&graph);

    println!("Nodes with their degree centrality:");
    for (node, score) in degree_centrality.iter() {
        println!("Node {} has degree centrality {}", node, score);
    }

    let clusters = clustering::identify_clusters(&graph);

    // Process and display clustering results
    println!("Identified Clusters:");
    for (cluster_id, nodes) in clusters.iter() {
        println!("Cluster {}: {} nodes", cluster_id, nodes.len());
    }

    // Optionally analyze cluster coherence
    for (cluster_id, nodes) in clusters.iter() {
        let coherence = clustering::analyze_cluster_coherence(nodes, &graph);
        println!("Coherence of Cluster {}: {}", cluster_id, coherence);
    }

    if let Err(e) = visualization::visualize_graph(&graph, "graph_visualization") {
        eprintln!("Failed to visualize graph: {}", e);
    }

}

//This takes a long time to run. I wasn't timing it but it's definitely longer than 
//20 minutes. But hey! VSCode didn't crash so I see this as an absolute win.
//Output:"Graph has 4,146,772 nodes and 45,564,149 edges"
//Yeahhhhhh. It's been well over 2 hours, maybe even more than 3, and fn main 
//is STILL going.
//Broskis it is 1:36 AM WHEN ARE YOU GONNA FINISH I WANNA WRAP UP AND LAY DOWN GODDAMNNNN
//I wanna go to sleep please please pleaseeeeeeeee
//Now it is 3:19. I am t h i s close to giving up
//but then i gotta wait  from square 1 later
//and this is due at 11pm
//should i just leave my laptop awake??
//yea maybe
//let's do that then :)
//omg watch me wake up at 1pm to find the cargo still running