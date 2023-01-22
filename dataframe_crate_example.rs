extern crate dataframe;

use dataframe::DataFrame;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![
        (4, "David", 40, "New York"),
        (5, "Eve", 45, "Chicago"),
        (6, "Frank", 50, "Los Angeles"),
    ];

    let df = DataFrame::new(data, vec!["id", "name", "age", "city"], vec![
        dataframe::ColumnType::Int,
        dataframe::ColumnType::String,
        dataframe::ColumnType::Int,
        dataframe::ColumnType::String,
    ])?;

    println!("{}", df);


    // Filter the dataframe to only include rows where the age is greater than or equal to 45
    df = df.filter("age >= 45");

    // Sort the dataframe by age in descending order
    df = df.sort("age", false);

    println!("{}", df);


    Ok(())
}


Data uses a vector of tuples with 4 fields, id, name, age and city. 
The resulting dataframe will have 4 columns, "id", "name", "age" and "city" 
and the data types of these columns are integer,string,integer and string respectively.
