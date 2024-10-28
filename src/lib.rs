use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{File, OpenOptions}; // Single import for `File` and `OpenOptions`
use std::io::Write;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct CandyData {
    pub competitorname: String,
    pub sugarpercent: f64,
}

pub fn read_data(file_path: &str) -> Result<Vec<CandyData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: CandyData = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn process_data(data: &[CandyData]) -> HashMap<String, f64> {
    let mut company_totals = HashMap::new();
    let mut company_counts = HashMap::new();

    for record in data {
        *company_totals.entry(record.competitorname.clone()).or_insert(0.0) += record.sugarpercent;
        *company_counts.entry(record.competitorname.clone()).or_insert(0) += 1;
    }

    // Calculate the mean
    let mut company_means = HashMap::new();
    for (company, total) in company_totals {
        if let Some(count) = company_counts.get(&company) {
            company_means.insert(company, total / *count as f64);
        }
    }

    company_means
}

pub fn log_to_md(
    input_data: &[CandyData],
    output_data: &HashMap<String, f64>,
    elapsed_time: f64,
    memory_used: u64,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    writeln!(file, "### Candy Data Processing")?;
    writeln!(file, "Elapsed time: {} seconds", elapsed_time)?;
    writeln!(file, "Memory used: {} KB", memory_used)?;

    // Log input and output data
    writeln!(file, "#### Input Data:")?;
    for record in input_data {
        writeln!(file, "{:?}", record)?;
    }

    writeln!(file, "#### Output Data:")?;
    for (company, mean) in output_data {
        writeln!(file, "{}: {:.3}", company, mean)?;
    }

    Ok(())
}
