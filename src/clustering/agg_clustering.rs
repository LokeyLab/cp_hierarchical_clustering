use super::{
    linkages::{calc_dist, LinkageMethod},
    ClusterMap, Distances,
};
use rayon::prelude::*;

pub fn hierarchical_clustering(
    dist_mat: &[Vec<f64>],
    linkage: LinkageMethod,
) -> Vec<(usize, usize, f64, usize)> {
    let dist = Distances::new(dist_mat);
    let n = dist.len();
    if n == 0 {
        return vec![];
    }

    let mut cluster_map = ClusterMap::new(n);
    let mut active_clusters: Vec<usize> = (0..n).collect();

    let mut merges = Vec::with_capacity(n - 1);
    let mut next_cid = n;

    while active_clusters.len() > 1 {
        let (cid1, cid2, min_dist) = {
            let ac = &active_clusters;
            let cmap = &cluster_map;
            let dist_mat = &dist;

            ac.par_iter()
                .enumerate()
                .flat_map_iter(|(i, &c1)| {
                    active_clusters[i + 1..].iter().map(move |&c2| {
                        let curr_dist = calc_dist(c1, c2, cmap, dist_mat, linkage);
                        (c1, c2, curr_dist)
                    })
                })
                .reduce(
                    || (0, 0, f64::MAX),
                    |acc, val| {
                        if val.2 < acc.2 {
                            val
                        } else {
                            acc
                        }
                    },
                )
        };

        let new_cid = next_cid;
        next_cid += 1;

        let mut merged_items = Vec::new();
        merged_items.extend_from_slice(&cluster_map.get_cluster(cid1));
        merged_items.extend_from_slice(&cluster_map.get_cluster(cid2));

        if new_cid >= cluster_map.cluster_map.len() {
            cluster_map.cluster_map.resize(new_cid + 1, vec![]);
        }
        cluster_map.cluster_map[new_cid] = merged_items;

        merges.push((cid1, cid2, min_dist, new_cid));

        active_clusters.retain(|&id| id != cid1 && id != cid2);
        active_clusters.push(new_cid);
    }

    return merges;
}
