use textplots::{Chart, Plot, Shape};

fn main() {
    Chart::default()
        .lineplot(&Shape::Continuous(Box::new(|x| x.sin().powi(3) / x)))
        .display();
}