use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;

pub fn plot(quadtree: &crate::quad_tree::Quadtree, points: &Vec<crate::point::Point>) {
    let root = BitMapBackend::new("quadtree.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(-100.0..100.0, -100.0..100.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for point in points {
        chart.draw_series(PointSeries::of_element(
            vec![(point.x, point.y)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        )).unwrap();
    }

    plot_quadtree(&chart, quadtree);
}

fn plot_quadtree(chart: &ChartContext<BitMapBackend, RangedCoordf32>, quadtree: &crate::quad_tree::Quadtree) {
    // Implementation of plot_quadtree function
    // Add your logic here to plot the quadtree
}