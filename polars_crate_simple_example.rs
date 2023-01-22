extern crate polars;
use polars::prelude::*;

fn main() {
    let data = vec![
        (1, "Alice", 25),
        (2, "Bob", 30),
        (3, "Charlie", 35),
    ];

    let df = DataFrame::new(data, vec!["id", "name", "age"]);

    println!("{}", df);
}

/*
This code creates a new dataframe called df using the DataFrame::new() constructor. 
The first argument is a vector of data, which in this example is a vector of tuples. 
The second argument is a vector of column names. 
The polars crate infers the column types from the data.

You can use the DataFrame struct to perform various operations like filtering, 
sorting, and joining as well as visualize the data using the built-in plot function.
*/
