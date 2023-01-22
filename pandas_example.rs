use pandas::prelude::*;

fn main() {
    let radius = vec![1.0, 2.0, 3.0];
    let angle = vec![0.0, 30.0, 45.0];
    let df = DataFrame::new()
        .add_column("radius", &radius)
        .add_column("angle", &angle);

    println!("{}", df);
}
