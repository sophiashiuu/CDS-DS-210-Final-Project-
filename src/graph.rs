use std::collections::{HashMap, HashSet};

pub struct LayoffsGraph {
    industries: HashMap<String, HashSet<String>>,
    edges: HashMap<(String, String), f64>, // Added edges field to store similarity
}

impl LayoffsGraph {
    // Existing methods...

    // New method to get total layoffs for a company
    pub fn get_total_layoffs(&self, company: &str) -> usize {
        if let Some(&layoffs) = self.layoffs_data.get(company) {
            layoffs
        } else {
            0 // Default to 0 if no layoffs data found for the company
        }
    }

    pub fn add_industry(&mut self, company: &str, industry: &str) {
        self.industries
            .entry(industry.to_string())
            .or_insert_with(HashSet::new)
            .insert(company.to_string());
    }

    pub fn add_similarity(&mut self, company1: &str, company2: &str, similarity: f64) {
        self.edges.insert((company1.to_string(), company2.to_string()), similarity);
    }

    pub fn get_clusters(&self) -> HashMap<String, Vec<String>> {
        let mut clusters = HashMap::new();

        for (company, industries) in &self.industries {
            for industry in industries {
                clusters
                    .entry(industry.clone())
                    .or_insert_with(Vec::new)
                    .push(company.clone());
            }
        }

        clusters
    }
}
