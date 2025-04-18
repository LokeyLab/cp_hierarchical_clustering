mod agg_clustering;
mod linkages;
mod utils;

pub use agg_clustering::hierarchical_clustering;
pub use linkages::LinkageMethod;
use serde::{Deserialize, Serialize};
pub use utils::DendrogramNode;

#[derive(Debug, Clone)]
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

    pub fn len(&self) -> usize {
        self.cluster_map.len()
    }

    pub fn replace_cid_vals(&mut self, cid: usize, values: &[usize]) {
        self.cluster_map[cid] = values.to_vec();
    }

    pub fn add_new_cid(&mut self, cid: usize) {
        self.cluster_map.resize(cid, vec![]);
    }
}

#[derive(Debug, Clone)]
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

/// Output for Agglomerative clustering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHierarchy {
    merges: Vec<Merge>,
    original_n: usize,
    tree: Option<DendrogramNode>,
}

/// Struct to represent merges
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Merge {
    cid1: usize,
    cid2: usize,
    dist: f64,
    new_cid: usize,
}
