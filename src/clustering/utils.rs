use std::{collections::HashMap, error::Error, fs::File, io::Write};

use serde::{Deserialize, Serialize};

use super::{ClusterHierarchy, Merge};

impl ClusterHierarchy {
    pub(in crate::clustering) fn new(merges: &[(usize, usize, f64, usize)], n: usize) -> Self {
        let merge_vec: Vec<Merge> = merges
            .iter()
            .map(|&(cid1, cid2, dist, new_cid)| Merge {
                cid1,
                cid2,
                dist,
                new_cid,
            })
            .collect();

        let mut cluster = ClusterHierarchy {
            merges: merge_vec,
            original_n: n,
            tree: None,
        };

        let tree = build_tree(&cluster);

        cluster.tree = Some(tree);

        return cluster;
    }

    /// gives original input items
    pub fn leaf_size(&self) -> usize {
        self.original_n
    }

    /// This saves the raw merge list as a json file. No trees are involved here
    pub fn simple_save(&self, filename: &str) -> std::io::Result<()> {
        let json_str =
            serde_json::to_string_pretty(&self.merges).expect("Can't serialize hierarchy!");

        let mut file = File::create(filename)?;
        file.write_all(json_str.as_bytes())?;

        return Ok(());
    }

    /// returns the merge list as a json formatted string
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        return Ok(serde_json::to_string_pretty(&self.merges).expect("Can't serialize hierarchy!"));
    }

    /// Returns the cluster outputin json tree form
    ///
    /// In json tree form, nodes are described as the following Node:
    ///
    /// - DendrogramNode:
    ///     - cid: cluster id as a usize var
    ///     - dist: the distance between 2 nodes as a f64 float
    ///     - left: the Left child as a DendrogramNode or None if it is a leaf
    ///     - right: the Right child as a DendrogramNode or None if it is a leaf
    pub fn to_json_tree(&self) -> Result<String, Box<dyn Error>> {
        if let Some(ref root) = &self.tree {
            let json_str = serde_json::to_string_pretty(&root).expect("Couldn't build json tree!");

            return Ok(json_str);
        } else {
            return Err("No Tree found!".into());
        }
    }

    /// Writes the json tree to a file
    pub fn write_tree(&self, fname: &str) -> Result<(), Box<dyn Error>> {
        let json_str = self.to_json_tree();

        if let Ok(json) = json_str {
            let mut file = File::create(fname)?;
            file.write_all(json.as_bytes())?;

            return Ok(());
        }

        return Err("Couldn't retrieve json tree".into());
    }

    /// Returns the leaf ordering which can be used to reorder heatmaps
    pub fn leaf_ordering(&self) -> Vec<usize> {
        if let Some(ref tree) = &self.tree {
            return get_leaf_order(&tree);
        } else {
            return vec![];
        }
    }

    /// Returns a copy of the Dendrogram nodes
    pub fn get_raw_nodes(&self) -> Option<DendrogramNode> {
        self.tree.clone()
    }
}

/// Dendrogram node
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DendrogramNode {
    cid: usize,
    distance: f64,
    left: Option<Box<DendrogramNode>>,
    right: Option<Box<DendrogramNode>>,
}

impl DendrogramNode {
    pub(in crate::clustering) fn new(
        cid: usize,
        dist: f64,
        left: Option<Box<DendrogramNode>>,
        right: Option<Box<DendrogramNode>>,
    ) -> Self {
        DendrogramNode {
            cid,
            distance: dist,
            left,
            right,
        }
    }
}

/// Builds a dendrogram tree
pub(in crate::clustering) fn build_tree(cluster: &ClusterHierarchy) -> DendrogramNode {
    // We know that the last item in merge list is the root
    // We also know that the first n items are leaves
    let mut nodes: HashMap<usize, DendrogramNode> = HashMap::new();

    // create leaf nodes
    for i in 0..cluster.original_n.clone() {
        nodes.insert(i, DendrogramNode::new(i, 0.0, None, None));
    }

    for &merge in &cluster.merges {
        let cid1 = merge.cid1;
        let cid2 = merge.cid2;
        let dist = merge.dist;
        let new_cid = merge.new_cid;

        let left_node = nodes
            .remove(&cid1)
            .expect(format!("Can't find left node of {} in node list!", cid1).as_str());
        let right_node = nodes
            .remove(&cid2)
            .expect(format!("Can't find right node of {} in node list!", cid2).as_str());

        let new_node = DendrogramNode::new(
            new_cid,
            dist,
            Some(Box::new(left_node)),
            Some(Box::new(right_node)),
        );

        nodes.insert(new_cid, new_node);
    }

    let root = cluster.merges.last().unwrap().new_cid;
    let root_node = nodes.remove(&root).expect("Can't find root node");
    return root_node;
}

/// Grabs Tree Leaf ordering
fn get_leaf_order(root: &DendrogramNode) -> Vec<usize> {
    if root.left.is_none() && root.right.is_none() {
        return vec![root.cid];
    }

    let mut orders: Vec<usize> = Vec::new();

    if let Some(ref left) = root.left {
        orders.extend(get_leaf_order(left));
    }
    if let Some(ref right) = root.right {
        orders.extend(get_leaf_order(right));
    }

    return orders;
}
