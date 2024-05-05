use std::collections::{HashMap, HashSet};
use crate::utils::calculate_median;

pub struct LayoffsGraph {
   pub industries: HashMap<String, HashSet<String>>,
   pub company_layoffs: HashMap<String, (u32, u32)>,
}

impl LayoffsGraph {
   pub fn new() -> Self {
       LayoffsGraph {
           industries: HashMap::new(),
           company_layoffs: HashMap::new(),
       }
   }

   pub fn add_company(&mut self, company: String, industry: String, year: u32, layoffs: u32) {
       self.industries
           .entry(industry)
           .or_insert_with(HashSet::new)
           .insert(company.clone());
       self.company_layoffs.insert(company, (layoffs, year));
   }

   pub fn to_adjacency_list(&self) -> HashMap<String, Vec<(String, u32)>> {
    let mut adjacency_list = HashMap::new();

    // Populate adjacency list with industries and their connected companies
    for (industry, companies) in &self.industries {
        let mut connections = vec![];
        for company in companies {
            if let Some(&(layoffs, _)) = self.company_layoffs.get(company) {
                connections.push((company.clone(), layoffs));
            }
        }
        adjacency_list.insert(industry.clone(), connections);
    }

    adjacency_list
}

   pub fn calculate_degree_centrality(&self) -> HashMap<String, usize> {
    let mut centrality = HashMap::new();
    for (industry, companies) in &self.industries {
        centrality.insert(industry.clone(), companies.len());
    }
    centrality
}


   pub fn get_industry_summary(&self) -> HashMap<String, (usize, u32, f32, f32)> {
       let mut summary = HashMap::new();
       for (industry, companies) in &self.industries {
           let mut total_layoffs = 0;
           let mut total_years = 0;
           for company in companies {
               if let Some(&(layoffs, year)) = self.company_layoffs.get(company) {
                   total_layoffs += layoffs;
                   total_years += year;
               }
           }


           let average_layoffs = if companies.len() > 0 {
               total_layoffs as f32 / companies.len() as f32
           } else {
               0.0
           };


           let average_year = if companies.len() > 0 {
               total_years as f32 / companies.len() as f32
           } else {
               0.0
           };


           summary.insert(
               industry.clone(),
               (
                   companies.len(),
                   total_layoffs,
                   average_layoffs,
                   average_year,
               ),
           );
       }


       summary
   }

//statistics 

   pub fn average_layoffs_per_year(&self) -> HashMap<u32, f32> {
       let mut total_layoffs_per_year = HashMap::new();
       //let mut _total_years = 0;
       let mut total_layoffs = 0;
       for (_company, &(layoffs, year)) in &self.company_layoffs {
           if year != 0 {
               *total_layoffs_per_year.entry(year).or_insert(0) += layoffs;
               //total_years += *year;
               total_layoffs += layoffs;
          
           }
       }

       let num_years = total_layoffs_per_year.len() as f32;
       let overall_average = if num_years > 0.0 { total_layoffs as f32 / num_years } else { 0.0 };

       let mut average_layoffs_per_year = HashMap::new();
       for (&year, &layoffs) in &total_layoffs_per_year {
           average_layoffs_per_year.insert(year, layoffs as f32/ num_years);
       }
       average_layoffs_per_year.insert(0, overall_average);
       average_layoffs_per_year
   }


   pub fn calculate_average_year_per_industry(&self) -> HashMap<String, f32> {
       let mut industry_averages = HashMap::new();
       for (industry, companies) in &self.industries {
           let mut total_years = 0;
           for company in companies {
               if let Some(&(_, year)) = self.company_layoffs.get(company) {
                   total_years += year;
               }
           }


           let average_year = if companies.len() > 0 {
               total_years as f32 / companies.len() as f32
           } else {
               0.0
           };


           industry_averages.insert(industry.clone(), average_year);
       }


       industry_averages
   }


   pub fn calculate_median_per_industry(&self, data: &HashMap<String, HashSet<String>>) -> HashMap<String, f32> {
       let mut industry_medians = HashMap::new();
       for (industry, companies) in data {
           let mut layoffs: Vec<u32> = companies.iter().filter_map(|company| {
               self.company_layoffs.get(company).map(|&(layoffs, _)| layoffs)
           }).collect();
           layoffs.sort();
           let median = calculate_median(&layoffs);
           industry_medians.insert(industry.clone(), median);
       }
       industry_medians
   }


   pub fn calculate_mode_per_industry(&self, data: &HashMap<String, HashSet<String>>) -> HashMap<String, u32> {
       let mut industry_modes = HashMap::new();
       for (industry, companies) in data {
           let mut layoffs_frequency = HashMap::new();
           for company in companies {
               if let Some(&(layoffs, _)) = self.company_layoffs.get(company) {
                   *layoffs_frequency.entry(layoffs).or_insert(0) += 1;
               }
           }
           let mode = *layoffs_frequency.iter().max_by_key(|&(_, count)| count).unwrap_or((&0, &0)).0;
           industry_modes.insert(industry.clone(), mode);
       }
       industry_modes
   }


   pub fn calculate_std_deviation_per_industry(&self, data: &HashMap<String, HashSet<String>>) -> HashMap<String, f32> {
       let mut industry_std_deviations = HashMap::new();
       for (industry, companies) in data {
           let layoffs: Vec<f32> = companies.iter().filter_map(|company| {
               self.company_layoffs.get(company).map(|&(layoffs, _)| layoffs as f32)
           }).collect();
           let mean = layoffs.iter().sum::<f32>() / companies.len() as f32;
           let variance = layoffs.iter().map(|layoffs| (*layoffs - mean).powi(2)).sum::<f32>() / companies.len() as f32;
           let std_deviation = variance.sqrt();
           industry_std_deviations.insert(industry.clone(), std_deviation);
       }
       industry_std_deviations
   }

   
}

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
