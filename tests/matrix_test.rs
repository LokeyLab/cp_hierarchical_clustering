use cp_hierarchical_clustering::*;
use polars::prelude::*;
use rand::Rng;

fn rand_matrix(m: usize, n: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::rng();
    (0..m)
        .map(|_| (0..n).map(|_| rng.random::<f64>()).collect())
        .collect()
}

fn matrix_to_df(mat: &[Vec<f64>]) -> PolarsResult<DataFrame> {
    let m = mat.len();
    let n = if m == 0 { 0 } else { mat[0].len() };

    // Build up one Series per column
    let mut columns: Vec<Column> = Vec::with_capacity(n);
    for col_index in 0..n {
        // Extract this column across all rows
        let col_data: Vec<f64> = mat.iter().map(|row| row[col_index]).collect();
        let series = Series::new(format!("Col: {}", col_index).into(), col_data);
        columns.push(series.into());
    }

    DataFrame::new(columns)
}

#[test]
fn calc_matrix_pearson_test() {
    let matrix = rand_matrix(384, 6000);

    let res = calculate_matrix(&matrix, Metric::Pearson, false);

    assert_eq!(res.len(), 384);
    assert_eq!(res[0].len(), 384);

    println!("{:?}", matrix_to_df(&res).unwrap());
}

#[test]
fn calc_matrix_dist_test() {
    let matrix = rand_matrix(384, 6000);

    let res = calculate_matrix(&matrix, Metric::Distance, false);

    assert_eq!(res.len(), 384);
    assert_eq!(res[0].len(), 384);

    println!("{:?}", matrix_to_df(&res).unwrap());
}

#[test]
fn calc_matrix_pearson_dist_test() {
    let matrix = rand_matrix(384, 6000);

    let res = calculate_matrix(&matrix, Metric::Pearson, true);

    assert_eq!(res.len(), 384);
    assert_eq!(res[0].len(), 384);

    println!("{:?}", matrix_to_df(&res).unwrap());
}
