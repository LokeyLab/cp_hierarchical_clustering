use super::operations::*;

pub(in crate::matrix_op) fn centered_correlation(u: &Vec<f64>, v: &Vec<f64>) -> f64 {
    let umu = mean(u).unwrap();
    let vmu = mean(v).unwrap();

    let u_centered: Vec<f64> = u.iter().map(|x| x - umu).collect();
    let v_centered: Vec<f64> = v.iter().map(|x| x - vmu).collect();

    let uv = mean(&multiply(&u_centered, &v_centered)).unwrap();
    let uu = mean(&multiply(&u_centered, &u_centered)).unwrap();
    let vv = mean(&multiply(&v_centered, &v_centered)).unwrap();

    let res = (1.0 - uv / (uu * vv).sqrt()).abs();
    return res;
}

pub(in crate::matrix_op) fn pearson_r(x: &Vec<f64>, y: &Vec<f64>, distance: bool) -> f64 {
    let n = x.len() as f64;

    let sum_x = sum(x).unwrap();
    let sum_y = sum(y).unwrap();
    let sum_xy = multiply(&x, &y).iter().sum::<f64>();

    let sum_x_sq = x.iter().map(|xi| xi * xi).into_iter().sum::<f64>();
    let sum_y_sq = y.iter().map(|xi| xi * xi).into_iter().sum::<f64>();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x_sq - sum_x.powi(2)) * (n * sum_y_sq - sum_y.powi(2))).sqrt();

    let r = numerator / denominator;

    let result = match distance {
        true => 1.0 - r,
        false => r,
    };

    return result;
}
