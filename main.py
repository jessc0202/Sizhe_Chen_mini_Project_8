import pandas as pd
import time
import psutil
import os

def log_to_md(input_data, output_data, elapsed_time, memory_used, filename="python_times.md"):
    """Logs input, output, elapsed time, and memory used to a specified Markdown file."""
    with open(filename, "w") as file:
        file.write("### Candy Data Processing\n")
        file.write(f"#### Input Data:\n```\n{input_data}\n```\n\n")
        file.write(f"#### Output Data:\n```\n{output_data}\n```\n\n")
        file.write(f"- **Elapsed Time**: {elapsed_time:.4f} seconds\n")
        file.write(f"- **Memory Used**: {memory_used / 1024:.2f} KB\n")

def main():
    # Start tracking time and memory
    start_time = time.time()
    process = psutil.Process(os.getpid())
    start_memory = process.memory_info().rss

    # Read the data
    input_file = "candy-data.csv"
    data = pd.read_csv(input_file)
    input_data = data

    # Example processing: Group by 'competitorname' and calculate the mean of 'sugarpercent'
    processed_data = data.groupby("competitorname")["sugarpercent"].mean()
    output_data = processed_data  # Preview of output data

    # Calculate elapsed time and memory used
    elapsed_time = time.time() - start_time
    end_memory = process.memory_info().rss
    memory_used = end_memory - start_memory

    # Log to Markdown file
    log_to_md(input_data, output_data, elapsed_time, memory_used)
    print("Processed data written to python_times.md")

if __name__ == "__main__":
    main()
