//! Operators for creating new graphs from existing ones.
use std::{ops::Mul, println};

use super::{
    EdgeType,
    graph::{Graph, IndexType},
};
use crate::{graph::NodeIndex, visit::IntoNodeReferences};

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

pub trait ProductRule {
    fn has_edge(a1_e_b1: bool, a2_e_b2: bool, a1_eq_b1: bool, a2_eq_b2: bool) -> bool;
}

pub trait WeightRule<N: Clone, E: Clone> {
    // specifies how to assign weight to edge
    // weight of ((u1,u2), (v1,v2)) is determined by w1 = weight(u1,v1) and w2 = weight(u2,v2)
    // weights can be None when there is no edge
    fn edge_weight(w1: Option<E>, w2: Option<E>) -> E;

    fn node_weight(w1: N, w2: N) -> N;
}

pub trait Cartesian {
    fn has_edge(a1_e_b1: bool, a2_e_b2: bool, a1_eq_b1: bool, a2_eq_b2: bool) -> bool {
        (a1_eq_b1 && a2_e_b2) || (a1_e_b1 && a2_eq_b2)
    }
}

impl<T: Cartesian> ProductRule for T {
    fn has_edge(a1_e_b1: bool, a2_e_b2: bool, a1_eq_b1: bool, a2_eq_b2: bool) -> bool {
        <T as Cartesian>::has_edge(a1_e_b1, a2_e_b2, a1_eq_b1, a2_eq_b2)
    }
}

pub struct CartesianUnweighted;

impl Cartesian for CartesianUnweighted {}

impl<E: Clone> WeightRule<(), E> for CartesianUnweighted {
    fn edge_weight(w1: Option<E>, w2: Option<E>) -> E {
        if let Some(w) = w1 { w } else { w2.unwrap() }
    }

    fn node_weight(_: (), _: ()) -> () {
        ()
    }
}

pub struct CartesianWeighted;

impl Cartesian for CartesianWeighted {}

impl<T: Clone + Mul<Output = T>, E: Clone> WeightRule<T, E> for CartesianWeighted {
    fn edge_weight(w1: Option<E>, w2: Option<E>) -> E {
        if let Some(w) = w1 { w } else { w2.unwrap() }
    }

    fn node_weight(w1: T, w2: T) -> T {
        w1 * w2
    }
}

pub struct Tensor;

impl ProductRule for Tensor {
    fn has_edge(a1_e_b1: bool, a2_e_b2: bool, _a1_eq_b1: bool, _a2_eq_b2: bool) -> bool {
        a1_e_b1 && a2_e_b2
    }
}

impl<N: Clone + Mul<Output = N>, E: Clone + Mul<Output = E>> WeightRule<N, E> for Tensor {
    fn edge_weight(w1: Option<E>, w2: Option<E>) -> E {
        w1.unwrap() * w2.unwrap()
    }

    fn node_weight(w1: N, w2: N) -> N {
        w1 * w2
    }
}

pub fn compute_graph_product<N, E, Ty, Ix, R>(
    g1: &Graph<N, E, Ty, Ix>,
    g2: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
    N: Clone,
    R: ProductRule + WeightRule<N, E>,
{
    let idx = |n1: NodeIndex<Ix>, n2: NodeIndex<Ix>| -> NodeIndex<Ix> {
        NodeIndex::new(n1.index() * g2.node_count() + n2.index())
    };

    for u1 in g1.node_indices() {
        for u2 in g2.node_indices() {
            output.add_node(R::node_weight(
                g1.node_weight(u1).unwrap().clone(),
                g2.node_weight(u2).unwrap().clone(),
            ));
        }
    }

    for u1 in g1.node_indices() {
        for u2 in g2.node_indices() {
            let source = idx(u1, u2);
            for v1 in g1.node_indices() {
                for v2 in g2.node_indices() {
                    let target = idx(v1, v2);
                    // TODO: should self-loops be skipped?
                    // if source == target {
                    //     continue;
                    // }

                    let u1_e_v1 = g1.neighbors(u1).any(|n| -> bool { n == v1 });
                    let w1 = if u1_e_v1 {
                        g1.edge_weight(g1.find_edge(u1, v1).unwrap())
                    } else {
                        None
                    };
                    let u2_e_v2 = g2.neighbors(u2).any(|n| -> bool { n == v2 });
                    let w2 = if u2_e_v2 {
                        g2.edge_weight(g2.find_edge(u2, v2).unwrap())
                    } else {
                        None
                    };

                    if R::has_edge(u1_e_v1, u2_e_v2, u1 == v1, u2 == v2) {
                        output.add_edge(source, target, R::edge_weight(w1.cloned(), w2.cloned()));
                    }
                }
            }
        }
    }
}

pub fn cartesian_product<N, E, Ty, Ix>(
    g1: &Graph<N, E, Ty, Ix>,
    g2: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
    N: Clone + Mul<Output = N>,
{
    compute_graph_product::<N, E, Ty, Ix, CartesianWeighted>(g1, g2, output);
}

pub fn cartesian_product_unweighted<E, Ty, Ix>(
    g1: &Graph<(), E, Ty, Ix>,
    g2: &Graph<(), E, Ty, Ix>,
    output: &mut Graph<(), E, Ty, Ix>,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone,
{
    compute_graph_product::<(), E, Ty, Ix, CartesianUnweighted>(g1, g2, output);
}

pub fn tensor_product<N, E, Ty, Ix>(
    g1: &Graph<N, E, Ty, Ix>,
    g2: &Graph<N, E, Ty, Ix>,
    output: &mut Graph<N, E, Ty, Ix>,
) where
    Ty: EdgeType,
    Ix: IndexType,
    E: Clone + Mul<Output = E>,
    N: Clone + Mul<Output = N>,
{
    compute_graph_product::<N, E, Ty, Ix, Tensor>(g1, g2, output);
}

// pub fn cartesian_product<N, E, Ty, Ix>(
//     g1: &Graph<N, E, Ty, Ix>,
//     g2: &Graph<N, E, Ty, Ix>,
//     output: &mut Graph<N, E, Ty, Ix>,
// ) where
//     Ty: EdgeType,
//     Ix: IndexType,
//     E: Clone,
//     N: Clone,
// {
// }
