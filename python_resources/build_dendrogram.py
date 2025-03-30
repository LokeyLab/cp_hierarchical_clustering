#!/usr/bin/env python3
from __future__ import annotations

import json
import sys

from matplotlib import pyplot as plt


class DendrogramNode:
    def __init__(
        self,
        cid: int,
        dist: float,
        left: DendrogramNode | None = None,
        right: DendrogramNode | None = None,
    ) -> None:
        self.cid = cid
        self.dist = dist
        self.left = left
        self.right = right

    def __str__(self) -> str:
        return (
            f"cid: {self.cid} dist: {self.dist} left: {self.left} right: {self.right}"
        )


def load_tree(fname: str) -> DendrogramNode:
    """Load in tree data"""

    with open(fname, "r") as f:
        data = json.load(f)
    return build_tree(data)


def build_tree(root: dict) -> DendrogramNode:
    cid = root.get("cid", -1)
    dist = root.get("distance", 0.0)
    left = root.get("left")
    right = root.get("right")

    node_left = build_tree(left) if left is not None else None
    node_right = build_tree(right) if right is not None else None

    return DendrogramNode(cid=cid, dist=dist, left=node_left, right=node_right)


def get_leaf_order(node: DendrogramNode) -> list[DendrogramNode]:
    if node.left is None and node.right is None:
        return [node]
    leaves = []

    if node.left:
        leaves.extend(get_leaf_order(node.left))
    if node.right:
        leaves.extend(get_leaf_order(node.right))
    return leaves


def assign_positions(root: DendrogramNode) -> dict[int, tuple[float, float]]:
    """
    Assign (x,y) positions to each node in tree

    x-coord: node's dist (leaves = 0)
    y-coord: position based on leaf order
    """

    positions: dict[int, tuple[float, float]] = {}

    leaves: list[DendrogramNode] = get_leaf_order(root)

    for i, leaf in enumerate(leaves):
        positions[leaf.cid] = (0, i)

    def assign_node_pos(node: DendrogramNode) -> tuple[float, float]:
        if node.left is None and node.right is None:
            return positions[node.cid]

        left_pos: tuple[float, float] = (
            assign_node_pos(node.left) if node.left else (0.0, 0.0)
        )
        right_pos: tuple[float, float] = (
            assign_node_pos(node.right) if node.right else (0.0, 0.0)
        )

        x = node.dist
        y = (left_pos[1] + right_pos[1]) / 2.0

        positions[node.cid] = (x, y)

        return positions[node.cid]

    assign_node_pos(root)
    return positions


def plot_dendrogram(
    root: DendrogramNode, positions: dict[int, tuple[float, float]], ax
) -> None:
    if root.left is None and root.right is None:
        return None

    x, y = positions[root.cid]
    if root.left:
        xl, yl = positions[root.left.cid]

        ax.plot([x, xl], [y, yl], c="blue")
        plot_dendrogram(root=root.left, positions=positions, ax=ax)
    if root.right:
        xr, yr = positions[root.right.cid]

        ax.plot([x, xr], [y, yr], c="blue")
        plot_dendrogram(root=root.right, positions=positions, ax=ax)

    if root.left and root.right:
        xr, yr = positions[root.right.cid]
        xl, yl = positions[root.left.cid]
        ax.plot([xl, xr], [yl, yr], c="blue", linestyle="dashed")

    return None


def main():
    fp = sys.argv[1]

    tree = load_tree(fp)

    pos = assign_positions(root=tree)

    fig, ax = plt.subplots(figsize=(8, 6))

    plot_dendrogram(root=tree, positions=pos, ax=ax)

    ax.set_xlabel("distance")
    ax.set_ylabel("leaf order")

    plt.title("dendrogram")
    fig.savefig("test.png")


if __name__ == "__main__":
    main()
