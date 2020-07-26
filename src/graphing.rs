//
//Move graphing utils here
//
//Create stats.rs
//    struct ReadStats -> Stats collected for each read
//    struct DumpStats -> Stats collected for each process dump, including all reads in order
//
//Figure out how to make graphs w/ multiple Y axes
//
//Graph list of read stats (take in stats list, closure for x, y?)
//    Read speed by region size
//    Read speed by region count
//
//Graph list of dump stats (we'll need to get rid of that aggregate thing and just take a list of IoStats)
//    Dump speed by read count
//    Dump speed by avg region size
//    Dump speed by total bytes read
//
//Can we build the same graphs but run them multiple times w/ a Z axis?
//

use std::cmp::{min, max};
use std::error::Error;
use std::result::Result;

use bytesize::ByteSize;
use plotters::prelude::*;

use super::stats::{IoStats};

/// Render a graph that shows read size compared to speed.
pub fn graph_read_size_to_speed(stats: &IoStats) -> Result<(), Box<dyn Error>> {
    /* Create a list of points. */
    let mut max_x = 10f32;
    let mut max_y = 10f32;

    let mut points: Vec<(f32, f32)> = Vec::new();
    for metric in &stats.io_metrics {
        let x = metric.size as f32;
        let y = metric.bytes_per_second() as f32;

        max_x = x.max(max_x);
        max_y = y.max(max_y);

        points.push((x, y));
    }

    points.sort_by(|(x1, _y1), (x2, _y2)| x1.partial_cmp(x2).unwrap());

    /* Graph points. */
    let root = BitMapBackend::new("read_size_to_speed.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Region size vs speed", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_ranged(0f32..max_x, 0f32..max_y)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_label_formatter(&|x| format!("{}", ByteSize(*x as u64).to_string_as(true)))
        .y_label_formatter(&|y| format!("{}/s", ByteSize(*y as u64).to_string_as(true)))
        .draw()?;

    chart.draw_series(LineSeries::new(points, &RED))?;

    Ok(())
}