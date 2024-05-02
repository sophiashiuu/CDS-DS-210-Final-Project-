use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct LayoffsGraph {
    industries: HashMap<String, HashSet<String>>,
    company_layoffs: HashMap<String, u32>, // Store layoffs per company
}

impl LayoffsGraph {
    pub fn new() -> Self {
        LayoffsGraph {
            industries: HashMap::new(),
            company_layoffs: HashMap::new(),
        }
    }

    pub fn add_company(&mut self, company: String, industry: String, layoffs: u32) {
        self.industries
            .entry(industry)
            .or_insert_with(HashSet::new)
            .insert(company.clone());
        self.company_layoffs.insert(company, layoffs);
    }

    pub fn get_industry_summary(&self) -> HashMap<String, (usize, u32)> {
        let mut summary = HashMap::new();
        for (industry, companies) in &self.industries {
            let total_layoffs: u32 = companies.iter()
                .map(|company| *self.company_layoffs.get(company).unwrap_or(&0))
                .sum();
            summary.insert(industry.clone(), (companies.len(), total_layoffs));
        }
        summary
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = LayoffsGraph::new();

    // Read data from CSV and construct graph
    let path = Path::new("layoffs(1).csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for (line_number, line) in reader.lines().enumerate().skip(1) {
        let line = line?;
        let parts: Vec<_> = line.split(',').map(|s| s.trim().to_string()).collect();
        if parts.len() < 3 {
            eprintln!("Invalid format in line {}: {}", line_number + 1, line);
            continue;
        }
        let company = parts[0].clone();
        let industry = parts[1].clone();
        let layoffs: u32 = parts[2].parse().unwrap_or(0);
        graph.add_company(company, industry, layoffs);
    }

    // Get industry summaries
    let summaries = graph.get_industry_summary();

    // Print summaries
    for (industry, (number_of_companies, total_layoffs)) in summaries {
        println!("-------------------");
        println!("Industry: {}", industry);
        println!("Number of companies: {}", number_of_companies);
        println!("Total number of Layoffs: {}", total_layoffs);
        println!("-------------------");
    }

    Ok(())
}

