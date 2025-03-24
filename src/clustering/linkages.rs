use core::f64;

use crate::clustering::{ClusterMap, Distances};

#[derive(Debug, Clone, Copy)]
pub enum LinkageMethod {
    Single,
    Complete,
    Average,
}

pub(in crate::clustering) fn calc_dist(
    cid1: usize,
    cid2: usize,
    cluster_map: &ClusterMap,
    distances: &Distances,
    method: LinkageMethod,
) -> f64 {
    let item1 = cluster_map.get_cluster(cid1);
    let item2 = cluster_map.get_cluster(cid2);

    match method {
        LinkageMethod::Single => {
            let mut dist = f64::MAX;
            for &i in item1 {
                for &j in item2 {
                    let d = distances.get_distances(i, j).clone();
                    if d < dist {
                        dist = d;
                    }
                }
            }
            return dist;
        }

        LinkageMethod::Complete => {
            let mut dist = f64::MIN;
            for &i in item1 {
                for &j in item2 {
                    let d = distances.get_distances(i, j).clone();
                    if d > dist {
                        dist = d;
                    }
                }
            }

            return dist;
        }

        LinkageMethod::Average => {
            let mut total = 0.0;
            let mut count = 0.0;
            for &i in item1 {
                for &j in item2 {
                    total += distances.get_distances(i, j).clone();
                    count += 1.0;
                }
            }

            return total / count;
        }
    };
}
