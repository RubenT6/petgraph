use petgraph::{
    Graph,
    operator::{complement, join, union},
    prelude::*,
};

#[test]
fn test_complement() {
    let mut graph: Graph<(), (), Directed> = Graph::new();
    let a = graph.add_node(());
    let b = graph.add_node(());
    let c = graph.add_node(());
    let d = graph.add_node(());

    graph.extend_with_edges([(a, b), (b, c), (c, d)]);
    let mut output: Graph<(), (), Directed> = Graph::new();

    complement(&graph, &mut output, ());

    let mut expected_res: Graph<(), (), Directed> = Graph::new();
    let a = expected_res.add_node(());
    let b = expected_res.add_node(());
    let c = expected_res.add_node(());
    let d = expected_res.add_node(());
    expected_res.extend_with_edges([
        (a, c),
        (a, d),
        (b, a),
        (b, d),
        (c, a),
        (c, b),
        (d, a),
        (d, b),
        (d, c),
    ]);

    for x in graph.node_indices() {
        for y in graph.node_indices() {
            assert_eq!(output.contains_edge(x, y), expected_res.contains_edge(x, y));
        }
    }
}

#[test]
fn test_union() {
    let mut g1: Graph<&str, (), Directed> = Graph::new();
    let a = g1.add_node("A");
    let b = g1.add_node("B");
    let c = g1.add_node("C");
    let d = g1.add_node("D");
    g1.extend_with_edges([(a, b), (b, c), (c, d)]);
    let mut g2: Graph<&str, (), Directed> = Graph::new();
    let e = g2.add_node("A");
    let f = g2.add_node("B");
    let g = g2.add_node("C");
    let h = g2.add_node("D");
    g2.extend_with_edges([(e, f), (f, g), (g, h)]);
    let mut result: Graph<&str, (), Directed> = Graph::new();
    union(&g1, &g2, &mut result);

    let mut expected_result: Graph<&str, (), Directed> = Graph::new();
    let a = expected_result.add_node("A");
    let b = expected_result.add_node("B");
    let c = expected_result.add_node("C");
    let d = expected_result.add_node("D");
    let e = expected_result.add_node("A");
    let f = expected_result.add_node("B");
    let g = expected_result.add_node("C");
    let h = expected_result.add_node("D");
    expected_result.extend_with_edges([(a, b), (b, c), (c, d), (e, f), (f, g), (g, h)]);

    assert_eq!(format!("{:?}", result), format!("{:?}", expected_result));
}

#[test]
fn test_join() {
    let mut g1: Graph<usize, usize, Directed> = Graph::new();
    let a = g1.add_node(1);
    let b = g1.add_node(2);
    g1.extend_with_edges([(a, b, 2)]);
    let mut g2: Graph<usize, usize, Directed> = Graph::new();
    let e = g2.add_node(3);
    let f = g2.add_node(4);
    g2.extend_with_edges([(e, f, 3)]);
    let mut result: Graph<usize, usize, Directed> = Graph::new();
    join(&g1, &g2, &mut result, |n1, n2| {
        g1.node_weight(n1).unwrap() + g2.node_weight(n2).unwrap()
    });

    let mut expected_result: Graph<usize, usize, Directed> = Graph::new();
    let a = expected_result.add_node(1);
    let b = expected_result.add_node(2);
    let e = expected_result.add_node(3);
    let f = expected_result.add_node(4);
    expected_result.extend_with_edges([
        (a, b, 2),
        (e, f, 3),
        (a, e, 4),
        (a, f, 5),
        (b, e, 5),
        (b, f, 6),
    ]);

    assert_eq!(format!("{:?}", result), format!("{:?}", expected_result));
}
