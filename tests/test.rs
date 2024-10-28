use sizhe_chen_mini_project_8::{process_data, CandyData}; 


// Helper function to create test data
fn create_sample_data() -> Vec<CandyData> {
    vec![
        CandyData { competitorname: "Candy A".to_string(), sugarpercent: 0.5 },
        CandyData { competitorname: "Candy B".to_string(), sugarpercent: 0.8 },
        CandyData { competitorname: "Candy A".to_string(), sugarpercent: 0.7 },
        CandyData { competitorname: "Candy C".to_string(), sugarpercent: 0.3 },
    ]
}

#[test]
fn test_calculate_mean_sugarpercent_non_zero() {
    // Arrange: Create sample data
    let input_data = create_sample_data();

    // Act: Convert `input_data` to a slice and process the data
    let result_means = process_data(&input_data[..]);

    // Assert: Check that each company's mean sugar percentage is non-zero
    for (company, mean) in &result_means {
        assert!(
            *mean > 0.0,
            "Mean sugar percentage for {} should be greater than zero, found {}",
            company,
            mean
        );
    }
}
