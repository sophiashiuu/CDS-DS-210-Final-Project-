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
