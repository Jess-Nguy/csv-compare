use colored::Colorize;

pub fn compare_csv_files(
    file1: &str,
    file2: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut reader1 = csv::Reader::from_path(file1)?;
    let mut reader2 = csv::Reader::from_path(file2)?;

    let mut differences = Vec::new();

    // Assuming the headers values are the same in both files.
    let headers = reader1.headers()?.clone();

    for (row, (record1, record2)) in reader1.records().zip(reader2.records()).enumerate() {
        let record1 = record1?;
        let record2 = record2?;
        // where in the csv file the difference is found.
        for (col, (field1, field2)) in record1.iter().zip(record2.iter()).enumerate() {
            if field1 != field2 {
                let column_print = format!("Column #{}({})", col + 1, headers.get(col).unwrap());
                let row_print = format!("Row {}", row + 1);
                differences.push(format!(
                    "{} on {}: {} {} {}",
                    column_print.cyan().bold(),
                    row_print.cyan().bold(),
                    field1,
                    "!=".red().bold(),
                    field2
                ));
            }
        }
    }

    Ok(differences)
}
