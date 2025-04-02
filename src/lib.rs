mod clustering;
mod matrix_op;

use polars::prelude::*;
use std::error::Error;

pub use clustering::DendrogramNode;
pub use clustering::{hierarchical_clustering, ClusterHierarchy, LinkageMethod};
pub use matrix_op::{calculate_matrix, Metric};

/// # Given a data matrix, generate hierarchical clustering of data.
///
/// This would mainly be used for cytological profiling (cp) data.
///
/// ### params:
/// - raw_data: `&[Vec<f64>]` (note outer slice are seen as the rows while the inner Vec are the features)
/// - mat_metric: `Metric` (Enum)
/// - link_method: `LinkageMethod` (Enum)
pub fn create_hierarchy(
    raw_data: &[Vec<f64>],
    mat_metric: Metric,
    link_method: LinkageMethod,
) -> Result<ClusterHierarchy, Box<dyn Error>> {
    let dist_matrix = calculate_matrix(raw_data, mat_metric, true);
    let hierarchy = hierarchical_clustering(&dist_matrix, link_method);

    return hierarchy;
}

/// Same as `create_hierarchy` except converts data frame into the raw data matrix
pub fn create_hierarchy_from_df(
    df: &DataFrame,
    mat_metric: Metric,
    link_method: LinkageMethod,
    ignore_cols: &Option<Vec<usize>>,
) -> Result<ClusterHierarchy, Box<dyn Error>> {
    let data = df_to_vec(df, ignore_cols)?;
    return create_hierarchy(&data, mat_metric, link_method);
}

fn df_to_vec(df: &DataFrame, ignore_cols: &Option<Vec<usize>>) -> PolarsResult<Vec<Vec<f64>>> {
    let ignore_cols = match ignore_cols {
        Some(cols) => cols,
        None => &vec![],
    };

    let nrows = df.height();
    let ncols = df.width();

    let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(nrows);

    let cols: Vec<usize> = (0..ncols).filter(|x| !ignore_cols.contains(x)).collect();

    for row_idx in 0..nrows {
        let mut row: Vec<f64> = Vec::with_capacity(cols.len());
        for col_idx in cols.iter() {
            // get col as f64
            let series = df.select_at_idx(col_idx.clone()).unwrap();
            let float_col = series.f64()?.clone();

            let val = float_col.get(row_idx).ok_or_else(|| {
                PolarsError::ComputeError(format!("Index out of bounds for row {}", row_idx).into())
            })?;

            row.push(val);
        }
        matrix.push(row);
    }

    return Ok(matrix);
}
