mod physics;

use physics::*;
use plotters::{
    backend::BitMapBackend, chart::ChartBuilder, drawing::IntoDrawingArea, series::LineSeries,
    style::*, element::PathElement,
};
use std::io;

fn main() {
    let cross_area = prompt_number("Cross-sectional area (m^2)");
    let fluid_density = prompt_number("Fluid density (kg/m^3)");
    let drag_coefficient = prompt_number("Drag coefficient");
    let mass = prompt_number("Mass (kg)");
    let delta_time = prompt_number("Time step (s)");
    let initial_x = prompt_number("Initial x position (m)");
    let initial_y = prompt_number("Initial y position (m)");
    let initial_v = prompt_number("Initial velocity (m/s)");
    let initial_v_angle = prompt_number("Initial velocity angle (deg)");
    let ending_time = prompt_number("Ending time (s)");

    let initial_conditions = MotionState {
        position: Vec2::new(initial_x, initial_y),
        velocity: Vec2::from_magnitude_angle(initial_v, initial_v_angle),
        acceleration: Vec2::new(0.0, 0.0),
        time: 0.0,
    };

    let parameters = Parameters::new(
        cross_area,
        fluid_density,
        drag_coefficient,
        mass,
        delta_time,
        initial_conditions,
        ending_time,
    );
    let no_drag = Parameters::new(
        0.0,
        0.0,
        0.0,
        mass,
        delta_time,
        initial_conditions,
        ending_time,
    );

    let simulation = simulate_motion(parameters);
    let no_drag_simulation = simulate_motion(no_drag);

    let graph = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    graph.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&graph)
        .margin(5)
        .build_cartesian_2d(0.0..260.0, 0.0..30.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            simulation
                .iter()
                .map(|state| (state.position.x, state.position.y)),
            &MAGENTA,
        ))
        .unwrap()
        .label("With drag")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));
    chart
        .draw_series(LineSeries::new(
            no_drag_simulation
                .iter()
                .map(|state| (state.position.x, state.position.y)),
            &BLUE,
        ))
        .unwrap()
        .label("Without drag")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
    graph.present().unwrap();
}

fn prompt_number(prompt: &str) -> f64 {
    eprintln!("{}: ", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}
