mod linkages;

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
