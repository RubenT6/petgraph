use alloc::vec::Vec;

use proptest::prelude::*;

use crate::graph::{DirectedGraph, EditableGraph};

/// Strategy to generate random (directed) graphs
///
/// # Panics
/// If either adding a node or edge to the graph initialized using `graph_constructor` yields an
/// error
pub fn graph_strategy<
    G: DirectedGraph + EditableGraph<OwnedNodeData = N, OwnedEdgeData = E> + core::fmt::Debug,
    N: Arbitrary + Clone,
    E: Arbitrary + Clone,
>(
    max_nodes: usize,
    max_edges: usize,
    graph_constructor: impl Fn() -> G,
) -> impl Strategy<Value = G> {
    prop::collection::vec(any::<N>(), 0..max_nodes)
        .prop_flat_map(move |node_weights| {
            let num_nodes = node_weights.len();

            let mut node_pairs = Vec::new();
            for n1 in 0..num_nodes {
                for n2 in 0..num_nodes {
                    node_pairs.push((Just(n1), Just(n2)));
                }
            }

            let max_edges = core::cmp::min(max_edges, node_pairs.len());

            (
                Just(node_weights),
                node_pairs.prop_shuffle(),
                0..=max_edges,
                prop::collection::vec(any::<E>(), max_edges),
            )
        })
        .prop_map(
            move |(node_weights, node_pairs, edge_count, edge_weights)| {
                let mut graph = graph_constructor();
                let mut node_ids = Vec::with_capacity(node_weights.len());
                for weight in node_weights {
                    let node = graph.add_node(weight).unwrap();
                    node_ids.push(node);
                }

                for (i, &(source_idx, target_idx)) in node_pairs.iter().take(edge_count).enumerate()
                {
                    let source = node_ids[source_idx];
                    let target = node_ids[target_idx];
                    let edge_weight = edge_weights[i].clone();
                    let _unused = graph.add_edge(source, target, edge_weight).unwrap();
                }

                graph
            },
        )
}
