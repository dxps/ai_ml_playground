use rust_statistics::{fetch_dataset, load_dataset};

fn main() {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";

    match fetch_dataset(url) {
        Ok(csv_data) => match load_dataset(&csv_data) {
            Ok(dataset) => {
                println!("Loaded {} entries.", dataset.len())
            }
            Err(err) => {
                eprintln!("Failed to load the dataset: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch the dataset: {}", err)
        }
    }
}
