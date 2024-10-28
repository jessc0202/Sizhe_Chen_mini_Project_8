# test_candy_data.py
import pytest
import pandas as pd
from main import log_to_md

@pytest.fixture
def sample_data():
    """Fixture for creating sample data similar to candy-data.csv format."""
    return pd.DataFrame({
        "competitorname": ["Candy A", "Candy B", "Candy A", "Candy C"],
        "sugarpercent": [0.5, 0.8, 0.7, 0.3]
    })

def test_log_to_md_not_empty(tmpdir, sample_data):
    """Test that log_to_md generates non-empty output and valid elapsed time."""
    log_file = tmpdir.join("test_python_times.md")
    
    
    output_data = sample_data.groupby("competitorname")["sugarpercent"].mean()
    elapsed_time = 0.1234  # Non-zero elapsed time
    memory_used = 2048     # Non-zero memory used

    
    log_to_md(sample_data, output_data, elapsed_time, 
              memory_used, filename=str(log_file))

    
    with open(log_file, "r") as file:
        content = file.read()
        assert "#### Output Data:" in content
        assert len(output_data) > 0, "Output data should not be empty."
        assert elapsed_time > 0, "Elapsed time should be greater than zero."
        assert "Elapsed Time" in content and "Memory Used" in content
