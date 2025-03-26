mod clustering;
mod matrix_op;

pub use clustering::{hierarchical_clustering, LinkageMethod};
pub use matrix_op::{calculate_matrix, Metric};
