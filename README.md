# 📊 DS 210 Final Project: Technology Layoffs Since COVID-19  
**By: Sophia Shiu**

## 📌 Project Overview

This project explores the patterns and potential impacts of layoffs in the technology sector during and after the COVID-19 pandemic using a cleaned dataset of reported technology layoffs. The original dataset contained ~3,500 entries (companies), but after removing incomplete data, the final analysis was conducted on ~1,600 companies.

The goal was to investigate:
- 📉 Whether certain industries experienced more significant layoffs than others
- 🧩 If companies could be grouped into clusters based on industry similarities
- 🔍 Whether a relationship exists between the number of layoffs and industry centrality

## 🔍 Key Questions

- Is there a correlation between a specific industry and technology layoffs?
- Can we identify clusters or communities of companies disproportionately affected?
- Were layoffs more industry-specific, or a systemic outcome of the pandemic?

## 🧠 Methodology

Each company is treated as a **node**, and **edges** are created based on shared characteristics (such as industry). Using clustering by industry, we extract insights on how the pandemic may have impacted the technology workforce.

The graph-based analysis included:
- Converting data into an adjacency list
- Calculating **degree centrality** to identify industries with higher company counts
- Computing summary statistics (mean, median, mode, standard deviation) for layoffs across industries

---

## 📁 Project Structure

The project is split into three core modules:

### 1. `main.rs`
- Entry point of the program.
- Reads and parses the cleaned CSV dataset.
- Constructs the `LayoffsGraph` and generates summary statistics.
- Prints average layoffs and degree centrality for each industry.

### 2. `graph.rs`
- Defines the `LayoffsGraph` struct.
- Implements graph logic and analytics:
  - `to_adjacency_list()`: Builds adjacency list by industry.
  - `calculate_degree_centrality()`: Measures the importance of each industry.
  - `get_industry_summary()`: Returns summary statistics for each industry.
- Includes unit tests for core graph functionality.

### 3. `utils.rs`
- Utility functions for data processing:
  - `calculate_median`: Computes median of layoffs.
  - `parse_year_from_date`: Extracts the year from date strings.
- Includes tests for date parsing logic.

---

## 📈 Results

### Degree Centrality by Industry:
| Industry  | Degree (Number of Companies) |
|-----------|-------------------------------|
| Product   | 24                            |
| Hardware  | 14                            |
| AI        | 2                             |
| Consumer  | 60                            |

### Key Observations:
- **Hardware** had significantly higher total layoffs (21,972) across just 14 companies.
- **AI** had a very low total (170 layoffs across 2 companies).
- **Consumer** industries had both high degree centrality and high layoffs, indicating widespread impact.
- Average layoffs varied sharply by year:
  - 2020: ~13,096 layoffs
  - 2021: ~773 layoffs
  - 2022: ~21,913 layoffs

### 📌 Conclusion

Layoffs were not distributed evenly across industries:
- Some industries (like **Hardware** and **Consumer**) showed disproportionately high layoffs.
- Others, like **Product** and **AI**, were less affected.
- The number of companies (degree centrality) often corresponded with higher total layoffs.

Interestingly, the number of layoffs over time showed inconsistency—falling in 2021 before rising sharply in 2022. This makes it difficult to attribute the trends solely to the pandemic without pre-2020 baseline data.

---

## 🧪 Tests

Included unit tests check:
- Accurate addition of company data.
- Correct parsing of valid and invalid date formats.
- Functional accuracy of core utility and graph operations.

---

## 🛠️ Technologies Used
- Rust 🦀 (with modules for graph processing and statistics)
- CSV data parsing
- Basic statistics (mean, median, mode, std. deviation)

---

## 📂 Dataset
Dataset: *Technology Layoffs Since COVID-19*  
- Cleaned from ~3,500 to ~1,600 records for accuracy  
- Contains: Company Name, Industry, Date, Layoff Count, Location

---

## 📌 Future Work
- Compare against pre-pandemic layoffs to better understand causality
- Visualize industry clusters using tools like Gephi or Python-based network graphs
- Incorporate company size or revenue data for deeper correlation insights

---

## 📝 License
This project is for educational purposes (DS 210: Programming for Data Science, Summer 2025).

---

## 🙏 Acknowledgments
Thanks to the dataset providers and course instructors for support and guidance.
