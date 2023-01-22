extern crate polars;
use polars::prelude::*;
use std::path::Path;

fn main() {
    let data_path = Path::new("data.csv");
    let df = DataFrame::from_csv(data_path).unwrap();

    println!("{}", df);
}


/*
This code reads a CSV file called "data.csv" and creates a DataFrame. 
The from_csv method will automatically detect the types of the columns
and also infers the name of the columns from the first row of the csv file.
*/
