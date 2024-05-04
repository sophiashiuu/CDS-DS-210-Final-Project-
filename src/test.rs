#[cfg(test)]

    use super::*;

    #[test]
    fn test_add_company() {
        let mut graph = LayoffsGraph::new();
        graph.add_company("Company A".to_string(), "Tech".to_string(), 2020, 100);
        
        assert_eq!(graph.industries.get("Tech").unwrap().contains("Company A"), true);
        assert_eq!(graph.company_layoffs.get("Company A"), Some(&(100, 2020)));
    }