use crate::physics::MotionState;
use plotters::{
    backend::BitMapBackend, chart::ChartBuilder, drawing::IntoDrawingArea, element::PathElement,
    series::LineSeries, style::*,
};
use rgb::{AsPixels, RGB8, RGBA8};

pub fn graph(with_drag: &Vec<MotionState>, without_drag: &Vec<MotionState>, size: (u32, u32)) -> Vec<RGBA8> {
    let mut image_buffer = vec![0; size.0 as usize * size.1 as usize * 3];
    let graph = BitMapBackend::with_buffer(&mut image_buffer, size).into_drawing_area();
    graph.fill(&WHITE).unwrap();

    let (max_x, max_y) = find_maxes(&without_drag);
    let mut chart = ChartBuilder::on(&graph)
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..max_x * 1.1, 0.0..max_y * 1.1)
        .unwrap();

    chart
        .configure_mesh()
        .axis_desc_style(TextStyle::from(("sans-serif", 20)))
        .x_desc("Horizontal Position")
        .y_desc("Vertical Position")
        .draw()
        .unwrap();

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
    let rgba_image_buffer: Vec<RGBA8> = pixels.into_iter().map(RGBA8::from).collect();

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
