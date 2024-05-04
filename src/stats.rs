

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

