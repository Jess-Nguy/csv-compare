pub mod strings;
use colored::Colorize;

fn main() {
    // Example usage of the csv_compare module
    let file1 = "./file1.csv";
    let file2 = "./file2.csv";

    // Validate the CSV files before comparing
    if let Err(e) = validate_csv_files(file1, file2) {
        eprintln!("Error validating files: {}", e.to_string().red().bold());
        return;
    }

    // compare the rows as string match.
    match strings::compare_csv_files(file1, file2) {
        Ok(differences) => {
            if differences.is_empty() {
                println!("{}", "No differences found.".green().bold());
                return;
            }
            println!("NOTE: The search is case sensitive.");
            println!("Differences found:");
            for diff in differences {
                println!("{}", diff);
            }
        }
        Err(e) => eprintln!("Error comparing files: {}", e.to_string().red().bold()),
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

// validate the csv files. Files exists, headers are same, number of records are same.
fn validate_csv_files(file1: &str, file2: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure files exists before comparing
    if !std::path::Path::new(file1).exists() {
        return Err(format!("File {} does not exist.", file1).into());
    }
    if !std::path::Path::new(file2).exists() {
        return Err(format!("File {} does not exist.", file2).into());
    }

    let mut reader1 = csv::Reader::from_path(file1)?;
    let mut reader2 = csv::Reader::from_path(file2)?;

    // Check if the headers are the same
    let headers1 = reader1.headers()?.clone();
    let headers2 = reader2.headers()?.clone();
    if headers1 != headers2 {
        return Err(format!(
            "Headers are different:\nFile 1: {:?}\nFile 2: {:?}",
            headers1, headers2
        )
        .into());
    }
    // Check if the number of records is the same
    let num_records1 = reader1.records().count();
    let num_records2 = reader2.records().count();
    if num_records1 != num_records2 {
        return Err(format!(
            "Number of records are different:\nFile 1: {:?}\nFile 2: {:?}",
            num_records1, num_records2
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod validate_csv_files {
    use super::*;

    #[test]
    fn files_exist() {
        let file1 = "./file1.csv";
        let file2 = "./file2.csv";

        assert!(validate_csv_files(file1, file2).is_ok());
    }

    #[test]
    fn files_dont_exist() {
        let file1 = "./test-csv/header-diff-count-off1.csv";
        let file2 = "./test-csv/header-diff-count-off.csv";

        assert!(validate_csv_files(file1, file2).is_err());
    }

    #[test]
    fn headers_different() {
        let file1 = "./test-csv/header-diff-count-off1.csv";
        let file2 = "./test-csv/header-diff-count-off2.csv";

        assert!(validate_csv_files(file1, file2).is_err());
    }

    #[test]
    fn number_of_records_different() {
        let file1 = "./test-csv/header-diff-count-off1.csv";
        let file2 = "./test-csv/header-diff-count-off2.csv";
        assert!(validate_csv_files(file1, file2).is_err());
    }
}
