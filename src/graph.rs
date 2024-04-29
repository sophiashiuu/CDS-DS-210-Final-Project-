// graph.rs

use std::collections::{HashMap, HashSet};

pub struct LayoffsGraph {
    nodes: HashMap<String, HashSet<String>>,
}

impl LayoffsGraph {
    pub fn new() -> Self {
        LayoffsGraph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, company1: &str, company2: &str) {
        self.nodes
            .entry(company1.to_string())
            .or_insert_with(HashSet::new)
            .insert(company2.to_string());

        self.nodes
            .entry(company2.to_string())
            .or_insert_with(HashSet::new)
            .insert(company1.to_string());
    }

    pub fn get_clusters(&self) -> Vec<Vec<String>> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut clusters = Vec::new();

        for node in self.nodes.keys() {
            if !visited.contains(node) {
                let mut cluster = Vec::new();
                self.dfs(node, &mut visited, &mut cluster);
                clusters.push(cluster);
            }
        }

        clusters
    }

    fn dfs(&self, node: &str, visited: &mut HashSet<String>, cluster: &mut Vec<String>) {
        visited.insert(node.to_string());
        cluster.push(node.to_string());

        if let Some(neighbors) = self.nodes.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs(neighbor, visited, cluster);
                }
            }
        }
    }
}



// graph.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = LayoffsGraph::new();
        graph.add_edge("Company A", "Company B");
        graph.add_edge("Company B", "Company C");
        graph.add_edge("Company C", "Company D");

        assert!(graph.nodes.contains_key("Company A"));
        assert!(graph.nodes.contains_key("Company B"));
        assert!(graph.nodes.contains_key("Company C"));
        assert!(graph.nodes.contains_key("Company D"));

        assert!(graph.nodes.get("Company A").unwrap().contains("Company B"));
        assert!(graph.nodes.get("Company B").unwrap().contains("Company A"));
        assert!(graph.nodes.get("Company B").unwrap().contains("Company C"));
        assert!(graph.nodes.get("Company C").unwrap().contains("Company B"));
        assert!(graph.nodes.get("Company C").unwrap().contains("Company D"));
        assert!(graph.nodes.get("Company D").unwrap().contains("Company C"));
    }

    #[test]
    fn test_get_clusters() {
        let mut graph = LayoffsGraph::new();
        graph.add_edge("Company A", "Company B");
        graph.add_edge("Company B", "Company C");
        graph.add_edge("Company D", "Company E");

        let clusters = graph.get_clusters();
        assert_eq!(clusters.len(), 2);

        let mut cluster1_found = false;
        let mut cluster2_found = false;

        for cluster in clusters {
            match cluster.len() {
                3 => {
                    assert!(cluster.contains(&"Company A".to_string()));
                    assert!(cluster.contains(&"Company B".to_string()));
                    assert!(cluster.contains(&"Company C".to_string()));
                    cluster1_found = true;
                }
                2 => {
                    assert!(cluster.contains(&"Company D".to_string()));
                    assert!(cluster.contains(&"Company E".to_string()));
                    cluster2_found = true;
                }
                _ => panic!("Unexpected cluster size"),
            }
        }

        assert!(cluster1_found && cluster2_found);
    }
}
