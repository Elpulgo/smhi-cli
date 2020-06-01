extern crate textplots;

use textplots::{Chart, Plot, Shape};

pub fn print_chart(points: &[(f32, f32)]) {

    println!("\ny = interpolated points");
    Chart::new(200, 100, 0.0, points.len() as f32)
        .lineplot(Shape::Lines(&points))
        .nice();
    // .display();

    // println!("\ny = staircase points");
    // Chart::default()
    //     .lineplot( Shape::Lines(&points) )
    //     .display();

    // println!("\ny = scatter plot");
    // Chart::default()
    //     .lineplot( Shape::Points(&points) )
    //     .display();
}
