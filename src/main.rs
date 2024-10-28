// src/main.rs
use std::error::Error;
use std::time::Instant;
use sys_info;
use std::path::Path;

use sizhe_chen_mini_project_8::{read_data, process_data, log_to_md};

fn main() -> Result<(), Box<dyn Error>> {
    // Start tracking time and memory
    let start_time = Instant::now();
    let start_memory = sys_info::mem_info()?.free;

    // Read the data
    let data = read_data("candy-data.csv")?;

    // Process the data: Group by 'competitorname' and calculate the mean of 'sugarpercent'
    let processed_data = process_data(&data);  // Pass &data, which is a Vec<CandyData>

    // Calculate elapsed time and memory used
    let elapsed_time = start_time.elapsed().as_secs_f64();  // Get elapsed time in seconds
    let end_memory = sys_info::mem_info()?.free;
    let memory_used = start_memory.saturating_sub(end_memory);

    // Define the path for the Markdown file
    let path = Path::new("rust_times.md");

    // Log all data to Markdown file
    log_to_md(&data, &processed_data, elapsed_time, memory_used, &path)?;
    println!("Processed data written to rust_times.md");

    Ok(())
}
