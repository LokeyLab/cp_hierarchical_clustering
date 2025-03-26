use std::error::Error;

use super::{
    linkages::{calc_dist, LinkageMethod},
    ClusterHierarchy, ClusterMap, Distances,
};
use rayon::prelude::*;

pub fn hierarchical_clustering(
    dist_mat: &[Vec<f64>],
    linkage: LinkageMethod,
) -> Result<ClusterHierarchy, Box<dyn Error>> {
    let dist = Distances::new(dist_mat);
    let n = dist.len();
    if n == 0 {
        return Ok(ClusterHierarchy::new(&vec![(0, 0, 0.0, 0)]));
    }

    let mut cluster_map = ClusterMap::new(n);
    let mut active_clusters: Vec<usize> = (0..n).collect();

    let mut merges: Vec<(usize, usize, f64, usize)> = Vec::with_capacity(n - 1);
    let mut next_cid = n;

    while active_clusters.len() > 1 {
        let (cid1, cid2, min_dist) = {
            let ac = &active_clusters;
            let cmap = &cluster_map;
            let dist_map = &dist;

            ac.par_iter()
                .enumerate()
                .flat_map_iter(|(i, &c1)| {
                    // Find all distances from both clusters

                    active_clusters[i + 1..].iter().map(move |&c2| {
                        let curr_dist = calc_dist(c1, c2, cmap, dist_map, linkage);
                        (c1, c2, curr_dist)
                    })
                })
                .reduce(
                    // Find the minimum cluster distance
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

        // eprintln!("curr cluster: {:?} {:?} {:?}", cid1, cid2, min_dist);

        let new_cid = next_cid;
        next_cid += 1;

        let mut merged_cids = Vec::new();
        merged_cids.extend_from_slice(&cluster_map.get_cluster(cid1));
        merged_cids.extend_from_slice(&cluster_map.get_cluster(cid2));

        if new_cid >= cluster_map.len() {
            cluster_map.add_new_cid(new_cid + 1);
        }
        cluster_map.replace_cid_vals(new_cid, &merged_cids);

        merges.push((cid1, cid2, min_dist, new_cid));

        active_clusters.retain(|&id| id != cid1 && id != cid2);
        active_clusters.push(new_cid);

        // eprintln!("active clusters: {:?}", active_clusters);
    }

    return Ok(ClusterHierarchy::new(&merges));
}
