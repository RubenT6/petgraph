/// Generates a suite of property tests for [`DirectedGraph`][crate::graph::DirectedGraph]
/// implementations.
///
/// For the expected invariants and the arguments of this macro, see
/// [`__test_directed_graph_inner`][crate::__test_directed_graph_inner], which can also be used to
/// run the suite on other test graphs.
///
/// For a reference usage, see the tests in [`crate::utils::test_graphs::directed`].
#[macro_export]
macro_rules! proptest_directed_graph {
    ($graph_type:ty, $node_ty:ty, $edge_ty:ty, $strategy:expr, $constructor:expr,) => {
        proptest::proptest! {
            #[test]
            fn proptest_cardinality(
                graph in $strategy(10, 100, $constructor),
                weights in proptest::collection::vec(proptest::arbitrary::any::<$node_ty>(), 0..20),
            ) {
                $crate::proptest::directed::__private_proptesting::test_cardinality_inner::<$graph_type, $node_ty, $edge_ty>(
                    graph,
                    &weights,
                )?;
            }
        }
    };
}

// WARNING: This module contains only internal testing helper functions and should not be considered
// part of the public API. It is subject to change at any time.
#[doc(hidden)]
pub mod __private_proptesting {
    use alloc::vec::Vec;

    use proptest::prelude::*;

    use crate::graph::{DirectedGraph, EditableGraph};

    pub fn test_cardinality_inner<
        G: DirectedGraph + EditableGraph<OwnedNodeData = N, OwnedEdgeData = E> + core::fmt::Debug,
        N: Arbitrary + Clone,
        E: Arbitrary + Clone,
    >(
        mut graph: G,
        weights: &[N],
    ) -> Result<(), TestCaseError> {
        let initial_order = graph.node_count();
        let nodes: Vec<G::NodeId> = weights
            .iter()
            .map(|weight| graph.add_node(weight.clone()).unwrap())
            .collect();
        prop_assert_eq!(graph.node_count(), initial_order + weights.len());
        for node in nodes {
            let _unused = graph.remove_node(node);
        }
        prop_assert_eq!(graph.node_count(), initial_order);
        Ok(())
    }
}
