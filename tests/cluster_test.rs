use cp_hierarchical_clustering::{
    calculate_matrix, create_hierarchy_from_df, hierarchical_clustering, LinkageMethod, Metric,
};

use polars::prelude::*;
use rand::Rng;
use rayon::ThreadPoolBuilder;

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
fn cluster_test() {
    _ = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get().saturating_sub(10))
        .build_global()
        .unwrap();

    let matrix = rand_matrix(20, 6000);

    let res = calculate_matrix(&matrix, Metric::Pearson, true);

    // assert_eq!(res.len(), 384);
    // assert_eq!(res[0].len(), 384);

    // println!("{:?}", matrix_to_df(&res).unwrap());

    let merges = hierarchical_clustering(&res, LinkageMethod::Complete).unwrap();

    // _ = merges.simple_save("test.json").unwrap();
    // let merge_str = merges.to_json_tree();
    println!("{:?}", merges.leaf_ordering());

    // merges.write_tree("tree.json").expect("cant write json");
}

#[test]
fn cluster_test_ignore_cols() {
    // _ = ThreadPoolBuilder::new()
    //     .num_threads(num_cpus::get().saturating_sub(10))
    //     .build_global()
    //     .unwrap();

    let matrix = rand_matrix(384, 6000);
    let df = matrix_to_df(&matrix).unwrap();

    let ignore_cols: Option<Vec<usize>> = Some(vec![0, 5, 3, 8]);

    let res = create_hierarchy_from_df(&df, Metric::Pearson, LinkageMethod::Complete, &ignore_cols)
        .unwrap();
    println!("{:?}", res.leaf_ordering());
}

#[test]
fn cluster_test_df() {
    // _ = ThreadPoolBuilder::new()
    //     .num_threads(num_cpus::get().saturating_sub(10))
    //     .build_global()
    //     .unwrap();

    let matrix = rand_matrix(384, 6000);
    let df = matrix_to_df(&matrix).unwrap();

    let res =
        create_hierarchy_from_df(&df, Metric::Pearson, LinkageMethod::Complete, &None).unwrap();
    println!("{:?}", res.leaf_ordering());
}
