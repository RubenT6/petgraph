//! Operators for creating new graphs from existing ones.
use super::{
    EdgeType,
    graph::{Graph, IndexType},
};
use crate::{
    graph::NodeIndex,
    visit::{EdgeRef, IntoNodeReferences},
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
/// Computes in **O(|V1| + |V2| + |E1| + |E2|)**
/// where VX is the set of vertices of gX, and similarly for EX
pub fn union<N, E, Ty, Ix>(
    g1: &Graph<N, E, Ty, Ix>,
    g2: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
    N: Clone,
{
    for (_node, weight) in g1.node_references() {
        output.add_node(weight.clone());
    }
    for (_node, weight) in g2.node_references() {
        output.add_node(weight.clone());
    }
    for edge in g1.edge_references() {
        output.add_edge(edge.source(), edge.target(), edge.weight().clone());
    }
    let offset = g1.node_count();
    for edge in g2.edge_references() {
        output.add_edge(
            NodeIndex::new(edge.source().index() + offset),
            NodeIndex::new(edge.target().index() + offset),
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
/// Computes in **O(|V1| * |V2| + |E1| + |E2|)**
pub fn join<N, E, Ty, Ix, F>(
    g1: &Graph<N, E, Ty, Ix>,
    g2: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
    weights: F,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
    N: Clone,
    F: Fn(NodeIndex<Ix>, NodeIndex<Ix>) -> E,
{
    union(g1, g2, output);
    let offset = g1.node_count();
    for n1 in g1.node_indices() {
        for n2 in g2.node_indices() {
            output.add_edge(n1, NodeIndex::new(n2.index() + offset), weights(n1, n2));
        }
    }
}
