use plotters::prelude::*;
use plotters::style::{WHITE, BLUE};
use petgraph::graph::{NodeIndex};
use std::error::Error;
use std::collections::HashMap;

/// Function to visualiz centrality measures
pub fn visualize_centrality(
    centrality: &HashMap<NodeIndex, f64>,
    title: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut filtered_data: Vec<_> = centrality.iter().filter(|&(_, &value)| value > 0.0).collect();
    filtered_data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_centrality = filtered_data.iter().map(|(_, &value)| value).fold(0.0, f64::max);

    let root_area = BitMapBackend::new(output_path, (800, 600))
        .into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(title, ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..filtered_data.len(), 0.0..max_centrality)?;
    
    chart.configure_mesh().draw()?;

    let data: Vec<_> = filtered_data
        .iter()
        .enumerate()
        .map(|(i, (_, &value))| (i, value))
        .collect();

    chart
        .draw_series(data.iter().map(|&(x, y)| {
            Rectangle::new([(x, 0.0), (x + 1, y)], BLUE.filled())
        }))?;

    root_area.present()?;
    println!("Closeness Centrality visualization saved to {}", output_path);

    Ok(())
}

/// Here visualize the clustering coefficients of nodes
pub fn visualize_clustering_coefficient(
    clustering_coefficient: &HashMap<NodeIndex, f64>,
    title: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut sorted_coeffs: Vec<_> = clustering_coefficient.iter().collect();
    sorted_coeffs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let max_coeff = sorted_coeffs.iter().map(|(_, &value)| value).fold(0.0, f64::max);

    let root_area = BitMapBackend::new(output_path, (800, 600))
        .into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(title, ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..sorted_coeffs.len(), 0.0..max_coeff)?;

    chart.configure_mesh().draw()?;

    let data: Vec<_> = sorted_coeffs
        .iter()
        .enumerate()
        .map(|(i, (_, &value))| (i, value))
        .collect();

    chart
        .draw_series(data.iter().map(|&(x, y)| {
            Rectangle::new([(x, 0.0), (x + 1, y)], BLUE.filled())
        }))?;

    root_area.present()?;
    println!("Clustering Coefficient visualization saved to {}", output_path);

    Ok(())
}
