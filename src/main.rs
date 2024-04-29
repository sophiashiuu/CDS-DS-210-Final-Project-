// main.rs
mod graph;

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use graph::LayoffsGraph;

fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut graph = LayoffsGraph::new();

    for line in io::BufReader::new(file).lines().skip(1) {
        let line = line?;
        let parts: Vec<_> = line.split(',').collect();
        let company1 = parts[0].trim();
        let company2 = parts[1].trim();

        graph.add_edge(company1, company2);
    }

    let clusters = graph.get_clusters();
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}:", i + 1);
        let num_layoffs = cluster.len(); // Number of layoffs in this cluster
        println!("Number of layoffs: {}", num_layoffs);
        for company in cluster {
            println!("{}", company);
        }
        println!("-------------------");
    }

    Ok(())
}
fn main() {
    if let Err(err) = read_csv("layoffs.csv") {
        eprintln!("Error: {}", err);
    }
}
