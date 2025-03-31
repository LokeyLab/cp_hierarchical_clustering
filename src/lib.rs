mod clustering;
mod matrix_op;

use std::error::Error;

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
