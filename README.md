# Description:

Created this executable tool to help with simple comparison of 2 csv files.This is a case sensitive search.
Checks for:

- same headers
- same number of rows
- same values

# How to run

1. Go to `target\release` to find `csv-compare.exe` and `run-csv-compare.bat`
2. Download both `csv-compare.exe` and `run-csv-compare.bat` and put it in a folder. You can call it `csv-compare`.
3. Add your 2 files you want to compare into the `csv-compare` folder or what ever you names it. Name the files them `file1.csv` and `file2.csv`.
4. Double click on `run-csv-compare.bat` and it should open a terminal and run the comparison for you.

# New release for production:

Build the Executable. Run the following command in the root directory of your project (where Cargo.toml is located):

```bash
cargo build --release
```

This will replace the files in `target\release` with a new release.
