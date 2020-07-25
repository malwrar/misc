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
//Graph list of dump stats
//    Dump speed by read count
//    Dump speed by avg region size
//    Dump speed by total bytes read
//
//Can we build the same graphs but run them multiple times w/ a Z axis?
//
//use std::cmp::{min, max};
//use plotters::prelude::*;
//
//fn graph_stuff(dump_stats: &Vec<ProcDumpStats>) -> StdResult<(), Box<dyn std::error::Error>> {
//    let root = BitMapBackend::new("stats.png", (640, 480)).into_drawing_area();
//    root.fill(&WHITE)?;
//
//    let mut max_x = 10f32;
//    let mut max_y = 10f32;
//
//    let mut points: Vec<(f32, f32)> = Vec::new();
//    for stats in dump_stats {
//        let x = stats.bytes_per_second() as f32;
//        let y = stats.region_avg_size as f32;
//
//        max_x = x.max(max_x);
//        max_y = y.max(max_y);
//
//        points.push((x, y));
//    }
//
//    points.sort_by(|(x1, _y1), (x2, _y2)| x1.partial_cmp(x2).unwrap());
//
//    let mut chart = ChartBuilder::on(&root)
//        .caption("Region size speed", ("sans-serif", 50).into_font())
//        .margin(5)
//        .x_label_area_size(30)
//        .y_label_area_size(30)
//        .build_ranged(0f32..max_x, 0f32..max_y)?;
//
//    chart
//        .configure_mesh()
//        .x_labels(5)
//        .y_labels(5)
//        .x_label_formatter(&|x| format!("{}", ByteSize(*x as u64).to_string()))
//        .y_label_formatter(&|y| format!("{}", ByteSize(*y as u64).to_string()))
//        .draw()?;
//
//    chart.draw_series(LineSeries::new(points, &RED))?;
//
//    Ok(())
//}