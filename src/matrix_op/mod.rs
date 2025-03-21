use std::usize;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod metrics;
mod operations;

#[derive(Clone, Copy)]
pub enum Metric {
    Pearson,
    Distance,
}

pub fn calculate_matrix(mat: &[Vec<f64>], metric: Metric) -> Vec<Vec<f64>> {
    let n = mat.len();

    let init_val = match metric {
        Metric::Pearson => 1.0,
        Metric::Distance => 0.0,
    };
    let mut distances = vec![vec![init_val; n]; n];

    let pairwise_scores: Vec<(usize, usize, f64)> = (0..n)
        .into_par_iter()
        .flat_map_iter(|i| {
            (i..n).map(move |j| {
                let score = match metric {
                    Metric::Pearson => metrics::pearson_r(&mat[i], &mat[j]),
                    Metric::Distance => metrics::centered_correlation(&mat[i], &mat[j]),
                };

                (i, j, score)
            })
        })
        .collect();

    for (i, j, score) in pairwise_scores {
        distances[i][j] = score;
        distances[j][i] = score;
    }

    return distances;
}
