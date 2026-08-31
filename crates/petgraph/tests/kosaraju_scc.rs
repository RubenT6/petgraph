use petgraph::{Graph, algo::kosaraju_scc, prelude::*};

#[test]
fn test_kosaraju_scc() {
    let mut dag: Graph<&str, (), Directed> = Graph::new();
    let a = dag.add_node("A");
    // let b = dag.add_node("B");
    // let c = dag.add_node("C");
    // dag.extend_with_edges(&[(a, b), (b, c)]);
    // A -> B -> C
    // for node in dag.node_indices() {
    //     dag.add_edge(node, node, ());
    // }
    let sccs = kosaraju_scc(&dag);
    assert_eq!(sccs.len(), 1); // Each node is its own SCC
    // Each SCC contains exactly one node
    for scc in &sccs {
        assert_eq!(scc.len(), 1);
    }
}
