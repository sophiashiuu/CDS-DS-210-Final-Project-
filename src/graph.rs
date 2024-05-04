use std::collections::{HashMap, HashSet};
use crate::utils::{calculate_median, calculate_mode, calculate_std_deviation};

pub struct LayoffsGraph {
    industries: HashMap<String, HashSet<String>>,
    company_layoffs: HashMap<String, (u32, u32)>, // Store layoffs per company and year
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
            .entry(industry.clone())
            .or_insert_with(HashSet::new)
            .insert(company.clone());
        self.company_layoffs.insert(company, (layoffs, year));
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

    pub fn average_layoffs_per_year(&self) -> HashMap<u32, f32> {
        let mut total_layoffs_per_year = HashMap::new();
        let mut _total_years = 0;
        let mut total_layoffs = 0;

        for (_company, (layoffs, year)) in &self.company_layoffs {
            if *year != 0 {
                *total_layoffs_per_year.entry(*year).or_insert(0) += layoffs;
                _total_years += *year;
                total_layoffs += *layoffs;
            }
        }

        let num_years = total_layoffs_per_year.len() as f32;
        let overall_average = if num_years > 0.0 {
            total_layoffs as f32 / num_years
        } else {
            0.0
        };

        let mut average_layoffs_per_year = HashMap::new();
        for (year, layoffs) in total_layoffs_per_year {
            average_layoffs_per_year.insert(year, layoffs as f32 / num_years);
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

}
