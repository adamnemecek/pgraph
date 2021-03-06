// mod shared;
use pgraph::prelude::*;

#[test]
fn test_remove_node() {
    let mut gr = Graph::new();
    let a = gr.add_node("A"); // 0
    let b = gr.add_node("B"); // 1
    let c = gr.add_node("C"); // 2
    let d = gr.add_node("D"); // 3
    let e = gr.add_node("E"); // 4
    let f = gr.add_node("F"); // 5
    let g = gr.add_node("G"); // 6
    let _ = gr.add_edge(a, b, 0); // 0
    let _ = gr.add_edge(a, d, 1); // 1
    let _ = gr.add_edge(d, b, 2); // 2
    let _ = gr.add_edge(b, c, 3); // 3
    let _ = gr.add_edge(b, e, 4); // 4
    let _ = gr.add_edge(c, e, 5); // 5
    let _ = gr.add_edge(d, e, 6); // 6
    let _ = gr.add_edge(d, f, 7); // 7
    let _ = gr.add_edge(f, e, 8); // 8
    let _ = gr.add_edge(f, g, 9); // 9
    let _ = gr.add_edge(e, g, 10); // 10

    assert!(gr.node_count() == 7);
    assert!(gr.edge_count() == 11);

    gr.remove_node(e);

    assert!(gr.node_count() == 6);
    assert!(gr.edge_count() == 6);

    gr.remove_node(f);

    assert!(gr.node_count() == 5);
    assert!(gr.edge_count() == 4);

    gr.remove_node(a);

    assert!(gr.node_count() == 4);
    assert!(gr.edge_count() == 2);

    gr.remove_node(b);

    assert!(gr.node_count() == 3);
    assert!(gr.edge_count() == 0);

    // for (_, edge) in gr.edges().iter() {
    //     assert!(edge.is_disconnected());
    // }
}
