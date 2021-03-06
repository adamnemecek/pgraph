// mod shared;
use pgraph::prelude::*;

#[test]
fn test_remove_node() {
    let mut gr = Graph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");
    let g = gr.add_node("G");
    let _ = gr.add_edge(a, b, 7.);
    let _ = gr.add_edge(a, d, 5.);
    let _ = gr.add_edge(d, b, 9.);
    let _ = gr.add_edge(b, c, 8.);
    let _ = gr.add_edge(b, e, 7.);
    let _ = gr.add_edge(c, e, 5.);
    let _ = gr.add_edge(d, e, 15.);
    let _ = gr.add_edge(d, f, 6.);
    let _ = gr.add_edge(f, e, 8.);
    let _ = gr.add_edge(f, g, 11.);
    let _ = gr.add_edge(e, g, 9.);

    assert!(gr.node_count() == 7);
    assert!(gr.edge_count() == 11);

    gr.remove_node(e);

    assert!(gr.node_count() == 6);
    assert!(gr.edge_count() == 6);

    gr.remove_node(a);

    // assert!(gr.node_count() == 5);
    // assert!(gr.edge_count() == 4);
}
