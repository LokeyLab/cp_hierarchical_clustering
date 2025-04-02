# cp_hierarchical_clustering

## by Derfel Terciano

A hierarchical clustering library for Cytological profiling projects written in rust

## Purpose

This is a submodule that will be used for HistDiff Processor to generate HistDiff
heatmaps.

## Implements:

- Linkages:
  - Complete Linkage
  - Single Linkage
  - Average Linkage
- Metrics/Matrices:
  - Distance
  - Pearson Correlation

## Outputs:

- Dendrogram as json
- Row and column ordering

## main function:

```
create_hierarchy(
    raw_data: &[Vec<f64>],
    mat_metric: Metric,
    link_method: LinkageMethod,
) -> Result<ClusterHierarchy, Box<dyn Error>>
```

- This function takes a `&[Vec<f64>]` and creates a `ClusterHierarchy` struct that stores the clustering output.

```
create_hierarchy_from_df(
    df: &DataFrame,
    mat_metric: Metric,
    link_method: LinkageMethod,
    ignore_cols: &Option<Vec<usize>>,
) -> Result<ClusterHierarchy, Box<dyn Error>> {
```

- This is the same main function except it converts a polars dataframe into a hierarchy

# TODO:

- [x] Implement clustering foundation
  - [x] Implement Agglomerative clustering
- [x] Implement similarity/distance calculations
  - Use native vectors instead of ndarray
  - [x] Implement Distance matrix calculation
  - [x] Implement Pearson similarity matrix
- [x] Implement tree outputs
  - [x] Output tree JSON
  - [x] Output simple JSON output
  - [x] Output leaf orderings
  - [ ] Output dendrogram nodes themselves
