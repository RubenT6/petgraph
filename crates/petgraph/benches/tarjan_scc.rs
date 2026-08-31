#![feature(test)]
extern crate petgraph;
extern crate test;

use petgraph::{
    algo::tarjan_scc,
    prelude::{Graph, NodeIndex},
};
use rand::{Rng, SeedableRng, StdRng};
use test::{Bencher, black_box};

#[bench]
fn tarjan_scc_bench(bench: &mut Bencher) {
    static NODE_COUNT: usize = 100_000;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for i in 0..NODE_COUNT - 1 {
        g.add_edge(nodes[i], nodes[i + 1], ());
    }
    bench.iter(|| {
        let _sccs = tarjan_scc(black_box(&g));
    });
}

#[bench]
fn tarjan_scc_bench_self_loops(bench: &mut Bencher) {
    static NODE_COUNT: usize = 100_000;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for i in 0..NODE_COUNT - 1 {
        g.add_edge(nodes[i], nodes[i + 1], ());
        g.add_edge(nodes[i], nodes[i], ());
    }
    bench.iter(|| {
        // using black_box tells the compiler not to optimize the argument
        let _sccs = tarjan_scc(black_box(&g));
    });
}

#[bench]
fn tarjan_scc_bench_two_way_chain(bench: &mut Bencher) {
    static NODE_COUNT: usize = 100_000;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for i in 0..NODE_COUNT - 1 {
        g.add_edge(nodes[i], nodes[i + 1], ());
        g.add_edge(nodes[i + 1], nodes[i], ());
    }
    bench.iter(|| {
        let _sccs = tarjan_scc(&g);
    });
}

#[bench]
fn tarjan_scc_bench_big_loop(bench: &mut Bencher) {
    static NODE_COUNT: usize = 100_000;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for i in 0..NODE_COUNT - 1 {
        g.add_edge(nodes[i], nodes[i + 1], ());
    }
    g.add_edge(nodes[NODE_COUNT - 1], nodes[0], ());
    bench.iter(|| {
        let _sccs = tarjan_scc(&g);
    });
}

#[bench]
fn tarjan_scc_bench_many_edges(bench: &mut Bencher) {
    static NODE_COUNT: usize = 100_001;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    for j in [1, 2, 4, 5, 10, 20, 25, 50] {
        for i in 0..(NODE_COUNT - 1) / j {
            g.add_edge(nodes[i], nodes[(i + 1) * j], ());
        }
    }
    bench.iter(|| {
        let _sccs = tarjan_scc(&g);
    });
}

#[bench]
fn tarjan_scc_bench_chain_sccs(bench: &mut Bencher) {
    static CLIQUE_SIZE: usize = 100;
    static CLIQUE_COUNT: usize = 100;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..(CLIQUE_SIZE * CLIQUE_COUNT))
        .map(|i| g.add_node(i))
        .collect();
    for i in 0..CLIQUE_COUNT {
        // create complete subgraph of order CLIQUE_SIZE
        // includes self-loops
        for j in (i * CLIQUE_SIZE)..((i + 1) * CLIQUE_SIZE) {
            for k in (i * CLIQUE_SIZE)..((i + 1) * CLIQUE_SIZE) {
                g.add_edge(nodes[j], nodes[k], ());
            }
        }
    }
    // connect cliques (in one direction)
    // for i in 0..CLIQUE_COUNT - 1 {
    //     g.add_edge(nodes[i * CLIQUE_SIZE], nodes[(i + 1) * CLIQUE_SIZE], ());
    // }
    bench.iter(|| {
        let _sscs = tarjan_scc(&g);
    })
}

#[bench]
fn tarjan_scc_bench_disjoint_sccs(bench: &mut Bencher) {
    static CLIQUE_SIZE: usize = 100;
    static CLIQUE_COUNT: usize = 100;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..(CLIQUE_SIZE * CLIQUE_COUNT))
        .map(|i| g.add_node(i))
        .collect();
    for i in 0..CLIQUE_COUNT {
        // create complete subgraph of order CLIQUE_SIZE
        // includes self-loops
        for j in (i * CLIQUE_SIZE)..((i + 1) * CLIQUE_SIZE) {
            for k in (i * CLIQUE_SIZE)..((i + 1) * CLIQUE_SIZE) {
                g.add_edge(nodes[j], nodes[k], ());
            }
        }
    }
    bench.iter(|| {
        let _sscs = tarjan_scc(&g);
    })
}

#[bench]
fn tarjan_scc_bench_random(bench: &mut Bencher) {
    // tarjan_scc seems faster than kosaraju_scc for denser graphs, i.e., |E| approaching |V|^2
    static NODE_COUNT: usize = 10_000;
    static EDGE_COUNT: usize = 100;
    let mut g: Graph<usize, ()> = Graph::new();
    let nodes: Vec<NodeIndex<_>> = (0..NODE_COUNT).map(|i| g.add_node(i)).collect();
    // match seeds for kosaraju_scc and tarjan_scc to compare
    let seed: [u8; 32] = [142; 32];
    let mut rng = StdRng::from_seed(seed);
    for _ in 0..EDGE_COUNT - 1 {
        let source: usize = rng.gen_range(0, NODE_COUNT - 1);
        let target: usize = rng.gen_range(0, NODE_COUNT - 1);
        g.add_edge(nodes[source], nodes[target], ());
    }
    bench.iter(|| {
        let _sccs = tarjan_scc(&g);
    })
}
