use crate::physics::MotionState;
use plotters::{
    backend::BitMapBackend, chart::ChartBuilder, drawing::IntoDrawingArea, element::PathElement,
    series::LineSeries, style::*,
};
use rgb::{AsPixels, RGB8, RGBA8};

pub fn graph(with_drag: Vec<MotionState>, without_drag: Vec<MotionState>) -> Vec<RGBA8> {
    let mut image_buffer = vec![0; 800 * 600 * 3];
    let graph = BitMapBackend::with_buffer(&mut image_buffer, (800, 600)).into_drawing_area();
    graph.fill(&WHITE).unwrap();

    let (max_x, max_y) = find_maxes(&without_drag);
    let mut chart = ChartBuilder::on(&graph)
        .margin(5)
        .build_cartesian_2d(0.0..max_x * 1.1, 0.0..max_y * 1.1)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            with_drag
                .iter()
                .map(|state| (state.position.x, state.position.y)),
            &MAGENTA,
        ))
        .unwrap()
        .label("With drag")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));
    chart
        .draw_series(LineSeries::new(
            without_drag
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
    drop(chart);
    drop(graph);

    // plotters doesn't support RGBA, but we need it for Iced
    let pixels: Vec<RGB8> = image_buffer.as_pixels().to_vec();
    let rgba_image_buffer: Vec<RGBA8> = pixels
        .into_iter()
        .map(RGBA8::from)
        .collect();

    rgba_image_buffer
}

fn find_maxes(states: &Vec<MotionState>) -> (f64, f64) {
    // I love when f64 doesn't implement Ord
    let max_x = states
        .iter()
        .map(|state| state.position.x)
        .fold(0.0, f64::max);
    let max_y = states
        .iter()
        .map(|state| state.position.y)
        .fold(0.0, f64::max);
    (max_x, max_y)
}
