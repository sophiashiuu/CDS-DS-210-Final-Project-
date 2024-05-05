//main.rs 
mod graph;
mod stats;


use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use graph::LayoffsGraph;
use stats::parse_year_from_date;


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
       let year = parse_year_from_date(parts[5]); // Adjust the index as per your CSV structure
       let layoffs: u32 = parts[2].parse().unwrap_or(0);
       graph.add_company(company, industry, year, layoffs);
   }


   let summaries = graph.get_industry_summary();
   let median_layoffs = graph.calculate_median_per_industry(&graph.industries);
   let mode_layoffs = graph.calculate_mode_per_industry(&graph.industries);
   let stddev_layoffs = graph.calculate_std_deviation_per_industry(&graph.industries);


   for (industry, (num_companies, total_layoffs, avg_layoffs, _)) in summaries {
       println!("-------------------");
       println!("Industry: {}", industry);
       println!("Number of companies: {}", num_companies);
       println!("Total number of layoffs: {}", total_layoffs);
       println!("Average number of layoffs per company: {:.2}", avg_layoffs);
       println!("Median layoffs: {:.2}", median_layoffs.get(&industry).unwrap_or(&0.0));
       println!("Mode layoffs: {}", mode_layoffs.get(&industry).unwrap_or(&0));
       println!("Standard deviation of layoffs: {:.2}", stddev_layoffs.get(&industry).unwrap_or(&0.0));
   }
   println!("-------------------");


   // Print average layoffs per year
   let average_layoffs = graph.average_layoffs_per_year();
   println!("Average layoffs per year:");
   for (year, average) in average_layoffs {
       println!("Year {}: {:.2}", year, average);
   }


   Ok(())




}
