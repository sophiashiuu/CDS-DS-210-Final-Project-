use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod graph; 
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

fn calculate_median(data: &[u32]) -> f32 {
    let n = data.len();
    if n == 0 {
        return 0.0;
    }
    let mid = n / 2;
    if n % 2 == 0 {
        (data[mid - 1] as f32 + data[mid] as f32) / 2.0
    } else {
        data[mid] as f32
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = LayoffsGraph::new();

    
    let path = Path::new("layoffs(1).csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate().skip(1) {
        let line = line?;
        let parts: Vec<_> = line.split(',').map(|s| s.trim()).collect();
        if parts.len() < 4 {
            eprintln!("Invalid format in line {}: {}", line_number + 1, line);
            continue;
        }
        let company = parts[0].to_string();
        let industry = parts[1].to_string();
        let year = parse_year_from_date(parts[5]); // Assuming year is extracted from date at index 5
        let layoffs: u32 = parts[2].parse().unwrap_or(0);
        graph.add_company(company, industry, year, layoffs);
    }

    // Get industry summaries
    let summaries = graph.get_industry_summary();

    // Print summaries
    for (industry, (number_of_companies, total_layoffs, average_layoffs, _)) in &summaries {
        println!("-------------------");
        println!("Industry: {}", industry);
        println!("Number of companies: {}", number_of_companies);
        println!("Total number of Layoffs: {}", total_layoffs);
        println!("Average number of Layoffs per company: {:.2}", average_layoffs);
    
        // Calculate and print median
        if let Some(median) = graph.calculate_median_per_industry(&graph.industries).get(industry) {
            println!("Median layoffs: {:.2}", median);
        } else {
            println!("Median layoffs: N/A");
        }
    
        // Calculate and print mode
        if let Some(mode) = graph.calculate_mode_per_industry(&graph.industries).get(industry) {
            println!("Mode layoffs: {}", mode);
        } else {
            println!("Mode layoffs: N/A");
        }
    
        // Calculate and print standard deviation
        if let Some(std_deviation) = graph.calculate_std_deviation_per_industry(&graph.industries).get(industry) {
            println!("Standard Deviation of layoffs: {:.2}", std_deviation);
        } else {
            println!("Standard Deviation of layoffs: N/A");
        }
     
        println!("-------------------");
    }

    let average_layoffs_per_year = graph.average_layoffs_per_year();
    println!("Average layoffs per year:");
    for (year, average_layoffs) in &average_layoffs_per_year {
        println!("Year {}: {:.2}", year, average_layoffs);
    }

    Ok(())
}

fn parse_year_from_date(date: &str) -> u32 {
    date.split('-').next().unwrap_or("0").parse().unwrap_or(0)
}
