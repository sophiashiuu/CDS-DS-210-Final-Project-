#[cfg(test)]
mod test {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add_company() {
        let mut graph = LayoffsGraph::new();
        let company_name = "Test Company".to_string();
        let industry_name = "Tech".to_string();
        let year = 2021;
        let layoffs = 100;

        // Add the company
        graph.add_company(company_name.clone(), industry_name.clone(), year, layoffs);

        // Check if the company is added correctly to industries HashMap
        assert!(graph.industries.contains_key(&industry_name));
        assert!(graph.industries.get(&industry_name).unwrap().contains(&company_name));

        // Check if the company is added correctly to company_layoffs HashMap
        assert!(graph.company_layoffs.contains_key(&company_name));
        assert_eq!(graph.company_layoffs.get(&company_name).unwrap(), &(layoffs, year));
    }
}