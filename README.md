[![Python CI/CD Pipeline](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/actions/workflows/pythonCI.yml/badge.svg)](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/actions/workflows/pythonCI.yml)
[![Rust CI/CD Pipeline](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/actions/workflows/rustCI.yml/badge.svg)](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/actions/workflows/rustCI.yml)

# Sizhe Chen Mini Project 8

The project processes candy data to calculate the mean sugar percentage for each competitor and logs results to two Markdown files in python and rust.

### Preparation and Dependency Installation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build` for dependencies installation
4. run: `cargo run` 

### Check Format and Test Erros: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

## Python Implementation:
The Python version features a similar CLI, utilizing the argparse module for argument parsing. It also offers options for mode, input text, and optional key and IV in base64 format. The CLI measures execution time, logs it, and displays the encrypted or decrypted message along with the elapsed time.


### Preparation: 
1. git clone the repo
2. install: `make python_install`
3. run: `python main.py` 

### Check Format and Test Errors: 
1. Format code `make python_format`
2. Lint code `make python_lint`
3. Test coce `make python_test`

# Performance Comparison of Rust and Python Implementations

## Summary

This project provides both Rust and Python implementations for processing candy data by calculating the average sugar percentage of candies grouped by competitor names. The main objective of this comparison is to measure improvements in terms of **speed** and **memory usage** between the two implementations.

## Results

### Rust Implementation

- **Elapsed Time**: Approximately `0.0009` seconds
- **Memory Used**: `0 KB`
[Link to Rust runtime Markdown File](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/blob/main/rust_times.md)

### Python Implementation

- **Elapsed Time**: Approximately `0.0074` seconds
- **Memory Used**: `1376 KB`
[Link to Python runtime Markdown File](https://github.com/jessc0202/Sizhe_Chen_mini_Project_8/blob/main/python_times.md)

## Observations and Improvements

- **Speed**: The Rust implementation processed the data roughly **8 times faster** than the Python version. Rust’s lower-level control over memory and computational efficiency results in significantly quicker execution times, especially in CPU-bound tasks.
  
- **Memory Usage**: Rust demonstrated remarkable efficiency with memory management, consuming minimal memory (`0 KB` reported). Python, on the other hand, used `1376 KB` of memory for the same task. This difference illustrates Rust's optimized memory usage, as it avoids the overhead of Python’s garbage collection and dynamic typing.

## Why Rust Outperforms Python

1. **Compiled Language**: Rust is a compiled language, meaning it converts code to machine language before execution. This leads to faster execution times than Python, which is interpreted.
2. **Memory Management**: Rust provides explicit control over memory allocation and does not use garbage collection, allowing for efficient memory usage without unnecessary overhead.
3. **Concurrency**: Although not utilized in this example, Rust’s support for concurrency makes it highly performant for parallel computations in larger datasets.

## Use Cases

- For **performance-critical applications** with substantial computational demands, Rust offers a robust solution with minimal memory footprint and fast execution.
- The Python implementation remains useful for **rapid prototyping** and scenarios where ease of development and flexibility outweigh performance concerns.

By implementing this project in both Rust and Python, we observe that Rust’s design provides clear advantages in speed and memory efficiency. However, Python's simplicity and ecosystem are beneficial for quick development and data science applications.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/DaGenix/rust-crypto/
* https://github.com/nogibjj/rust-data-engineering