#[derive(Debug, Clone, Copy)]
pub enum LinkageMethod {
    Single,
    Complete,
    Average,
}

pub(in crate::clustering) fn calc_dist(
    cid1: usize,
    cid2: usize,
    cluster_map: &[Vec<usize>],
    distances: &[Vec<f64>],
    method: LinkageMethod,
) {
}
