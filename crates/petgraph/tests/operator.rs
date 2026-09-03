use petgraph::{
    Graph,
    operator::{cartesian_product, cartesian_product_unweighted, complement, tensor_product},
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
fn test_cartesian_product() {
    let mut g1: Graph<u32, (), Directed> = Graph::new();
    let a = g1.add_node(2);
    let b = g1.add_node(3);

    let mut g2: Graph<u32, (), Directed> = Graph::new();
    let c = g2.add_node(4);
    let d = g2.add_node(5);

    g1.extend_with_edges([(a, b)]);
    g2.extend_with_edges([(c, d)]);

    let mut output: Graph<u32, (), Directed> = Graph::new();

    cartesian_product(&g1, &g2, &mut output);

    let mut expected_res: Graph<u32, (), Directed> = Graph::new();
    let ac = expected_res.add_node(8);
    let ad = expected_res.add_node(10);
    let bc = expected_res.add_node(12);
    let bd = expected_res.add_node(15);
    expected_res.extend_with_edges([(ac, ad), (ac, bc), (ad, bd), (bc, bd)]);

    assert_eq!(format!("{:?}", output), format!("{:?}", expected_res));
}

#[test]
fn test_cartesian_product_unweighted() {
    let mut g1: Graph<(), (), Directed> = Graph::new();
    let a = g1.add_node(());
    let b = g1.add_node(());

    let mut g2: Graph<(), (), Directed> = Graph::new();
    let c = g2.add_node(());
    let d = g2.add_node(());

    g1.extend_with_edges([(a, b)]);
    g2.extend_with_edges([(c, d)]);

    let mut output: Graph<(), (), Directed> = Graph::new();

    cartesian_product_unweighted(&g1, &g2, &mut output);

    let mut expected_res: Graph<(), (), Directed> = Graph::new();
    let ac = expected_res.add_node(());
    let ad = expected_res.add_node(());
    let bc = expected_res.add_node(());
    let bd = expected_res.add_node(());
    expected_res.extend_with_edges([(ac, ad), (ac, bc), (ad, bd), (bc, bd)]);

    assert_eq!(format!("{:?}", output), format!("{:?}", expected_res));
}

#[test]
fn test_tensor_product() {
    let mut g1: Graph<u32, u32, Directed> = Graph::new();
    let a = g1.add_node(2);
    let b = g1.add_node(3);

    let mut g2: Graph<u32, u32, Directed> = Graph::new();
    let c = g2.add_node(4);
    let d = g2.add_node(5);

    g1.extend_with_edges([(a, b, 6)]);
    g2.extend_with_edges([(c, d, 7)]);

    let mut output: Graph<u32, u32, Directed> = Graph::new();

    tensor_product(&g1, &g2, &mut output);

    let mut expected_res: Graph<u32, u32, Directed> = Graph::new();
    let ac = expected_res.add_node(8);
    let _ad = expected_res.add_node(10);
    let _bc = expected_res.add_node(12);
    let bd = expected_res.add_node(15);
    expected_res.extend_with_edges([(ac, bd, 42)]);

    assert_eq!(format!("{:?}", output), format!("{:?}", expected_res));
}
