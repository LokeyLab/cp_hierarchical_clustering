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

# TODO:

- [x] Implement clustering foundation
  - [x] Implement Agglomerative clustering
- [x] Implement similarity/distance calculations
  - Use native vectors instead of ndarray
  - [x] Implement Distance matrix calculation
  - [x] Implement Pearson similarity matrix
- [ ] Implement tree outputs
  - [x] Output tree JSON
  - [x] Output simple JSON output
  - [ ] Output leaf orderings
