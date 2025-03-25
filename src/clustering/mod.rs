mod linkages;
use linkages::calc_dist;

pub(in crate::clustering) struct ClusterMap {
    cluster_map: Vec<Vec<usize>>,
}

impl ClusterMap {
    pub fn new(n_items: usize) -> Self {
        ClusterMap {
            cluster_map: (0..n_items).map(|i: usize| vec![i]).collect(),
        }
    }

    pub fn get_cluster(&self, cid: usize) -> &Vec<usize> {
        &self.cluster_map[cid]
    }
}

pub(in crate::clustering) struct Distances {
    distances: Vec<Vec<f64>>,
    len: usize,
}

impl Distances {
    pub fn new(dists: &[Vec<f64>]) -> Self {
        Distances {
            distances: dists.to_vec(),
            len: dists.len(),
        }
    }

    pub fn get_distances(&self, i: usize, j: usize) -> &f64 {
        &self.distances[i][j]
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
