//! Operators for creating new graphs from existing ones.
use std::collections::HashMap;

use super::{
    EdgeType,
    graph::{Graph, IndexType},
};
use crate::{
    data::Build,
    graph::NodeIndex,
    visit::{Data, EdgeRef, IntoEdgeReferences, IntoNodeReferences, NodeRef},
};

/// \[Generic\] complement of the graph
///
/// Computes the graph complement of the input Graph and stores it
/// in the provided empty output Graph.
///
/// The function does not create self-loops.
///
/// Computes in **O(|V|^2*log(|V|))** time (average).
///
/// Returns the complement.
///
/// # Example
/// ```rust
/// use petgraph::{Graph, operator::complement, prelude::*};
///
/// let mut graph: Graph<(), (), Directed> = Graph::new();
/// let a = graph.add_node(()); // node with no weight
/// let b = graph.add_node(());
/// let c = graph.add_node(());
/// let d = graph.add_node(());
///
/// graph.extend_with_edges(&[(a, b), (b, c), (c, d)]);
/// // a ----> b ----> c ----> d
///
/// let mut output: Graph<(), (), Directed> = Graph::new();
///
/// complement(&graph, &mut output, ());
///
/// let mut expected_res: Graph<(), (), Directed> = Graph::new();
/// let a = expected_res.add_node(());
/// let b = expected_res.add_node(());
/// let c = expected_res.add_node(());
/// let d = expected_res.add_node(());
/// expected_res.extend_with_edges(&[
///     (a, c),
///     (a, d),
///     (b, a),
///     (b, d),
///     (c, a),
///     (c, b),
///     (d, a),
///     (d, b),
///     (d, c),
/// ]);
///
/// for x in graph.node_indices() {
///     for y in graph.node_indices() {
///         assert_eq!(output.contains_edge(x, y), expected_res.contains_edge(x, y));
///     }
/// }
/// ```
pub fn complement<N, E, Ty, Ix>(
    input: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
    weight: E,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
    N: Clone,
{
    for (_node, weight) in input.node_references() {
        output.add_node(weight.clone());
    }
    for x in input.node_indices() {
        for y in input.node_indices() {
            if x != y && !input.contains_edge(x, y) {
                output.add_edge(x, y, weight.clone());
            }
        }
    }
}

/// Union of two graphs
///
/// Computes the (disjoint) union of the two input graphs
/// and stores it in the (empty) output graph
///
/// **Panics** if the number of nodes or edges does not fit with
/// the given output graph's index type.
///
/// Computes in **O(|V1| + |V2| + |E1| + |E2|)**
/// where VX is the set of vertices of gX, and similarly for EX
// pub fn union<N, E, Ty, Ix>(
//     g1: &Graph<N, E, Ty, Ix>,
//     g2: &Graph<N, E, Ty, Ix>,
//     output: &mut Graph<N, E, Ty, Ix>,
// ) where
//     Ty: EdgeType,
//     Ix: IndexType,
//     E: Clone,
//     N: Clone,
// {
//     *output = g1.clone();
//     for (_node, weight) in g2.node_references() {
//         output.add_node(weight.clone());
//     }
//     let offset = g1.node_count();
//     for edge in g2.edge_references() {
//         output.add_edge(
//             NodeIndex::new(edge.source().index() + offset),
//             NodeIndex::new(edge.target().index() + offset),
//             edge.weight().clone(),
//         );
//     }
// }
pub fn union<G>(g1: &G, g2: &G, output: &mut G)
where
    G: Build + Clone + Data,
    for<'a> &'a G: IntoNodeReferences + IntoEdgeReferences,
    for<'a> <&'a G as IntoNodeReferences>::NodeRef:
        NodeRef<Weight = G::NodeWeight, NodeId = G::NodeId>,
    for<'a> <&'a G as IntoEdgeReferences>::EdgeRef:
        EdgeRef<Weight = G::EdgeWeight, NodeId = G::NodeId>,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone,
    G::NodeId: Eq + core::hash::Hash,
{
    *output = g1.clone();
    let g2_nodes_map: HashMap<G::NodeId, G::NodeId> = g2
        .node_references()
        .map(|n| (n.id(), output.add_node((*n.weight()).clone())))
        .collect();
    for edge in g2.edge_references() {
        output.add_edge(
            g2_nodes_map[&edge.source()],
            g2_nodes_map[&edge.target()],
            edge.weight().clone(),
        );
    }
}

/// Graph join
///
/// Computes the join of the two input graphs
/// and stores it in the (empty) output graph
///
/// Adds edges from all nodes from g1 to all nodes from g2
/// Graph join for directed graphs is thus uni-directional
///
/// The `weights` function should specify how to give new edges a weight
/// E.g., if you have no edge weights (E = ()) then you can provide `|_,_| ()`
///
/// **Panics** if the number of nodes or edges does not fit with
/// the given output graph's index type.
///
/// Computes in **O(|V1| * |V2| + |E1| + |E2|)**
pub fn join<G, F>(g1: &G, g2: &G, output: &mut G, weights: F)
where
    G: Build + Clone + Data,
    for<'a> &'a G: IntoNodeReferences + IntoEdgeReferences,
    for<'a> <&'a G as IntoNodeReferences>::NodeRef:
        NodeRef<Weight = G::NodeWeight, NodeId = G::NodeId>,
    for<'a> <&'a G as IntoEdgeReferences>::EdgeRef:
        EdgeRef<Weight = G::EdgeWeight, NodeId = G::NodeId>,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone,
    G::NodeId: Eq + core::hash::Hash,
    F: Fn(G::NodeId, G::NodeId) -> G::EdgeWeight,
{
    *output = g1.clone();
    let g2_nodes_map: HashMap<G::NodeId, G::NodeId> = g2
        .node_references()
        .map(|n| (n.id(), output.add_node((*n.weight()).clone())))
        .collect();
    for edge in g2.edge_references() {
        output.add_edge(
            g2_nodes_map[&edge.source()],
            g2_nodes_map[&edge.target()],
            edge.weight().clone(),
        );
    }
    for n1 in g1.node_references() {
        for n2 in g2.node_references() {
            output.add_edge(n1.id(), g2_nodes_map[&n2.id()], weights(n1.id(), n2.id()));
        }
    }
}
