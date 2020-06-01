extern crate textplots;
pub mod chart;
use chart::print_chart;

fn main() {
    let points = [
        (1.0, 12.0),
        (2.0, 23.0),
        (3.0, 19.0),
        (4.0, 15.0),
        (5.0, 23.7),
        (6.0, 25.8),
        (7.0, 10.0),
    ];

    print_chart(&points);
    // println!("\ny = interpolated points");
    // Chart::new(200, 100 , 0.0, points.len() as f32)
    //     .lineplot( Shape::Lines(&points) )
    //     .display();

    // println!("\ny = staircase points");
    // Chart::default()
    //     .lineplot( Shape::Lines(&points) )
    //     .display();

    // println!("\ny = scatter plot");
    // Chart::default()
    //     .lineplot( Shape::Points(&points) )
    //     .display();
}
