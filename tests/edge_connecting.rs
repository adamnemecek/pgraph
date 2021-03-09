use pgraph::prelude::*;

#[test]
fn test_edge_connecting() {
    let mut gr = Graph::new();
    let a = gr.add_node("A"); // 0
    let b = gr.add_node("B"); // 1
    let c = gr.add_node("C"); // 2
    let d = gr.add_node("D"); // 3
    let e = gr.add_node("E"); // 4
    let f = gr.add_node("F"); // 5
    let g = gr.add_node("G"); // 6

    let ab = gr.add_edge(a, b, 0).unwrap(); // 0
    let ad = gr.add_edge(a, d, 1).unwrap(); // 1
    let db = gr.add_edge(d, b, 2).unwrap(); // 2
    let bc = gr.add_edge(b, c, 3).unwrap(); // 3
    let be = gr.add_edge(b, e, 4).unwrap(); // 4
    let ce = gr.add_edge(c, e, 5).unwrap(); // 5
    let de = gr.add_edge(d, e, 6).unwrap(); // 6
    let df = gr.add_edge(d, f, 7).unwrap(); // 7
    let fe = gr.add_edge(f, e, 8).unwrap(); // 8
    let fg = gr.add_edge(f, g, 9).unwrap(); // 9
    let eg = gr.add_edge(e, g, 10).unwrap(); // 10

    assert!(gr.edge_connecting(a, b) == Some(ab));
    assert!(gr.edge_connecting(a, d) == Some(ad));
    assert!(gr.edge_connecting(d, b) == Some(db));
    assert!(gr.edge_connecting(b, c) == Some(bc));
    assert!(gr.edge_connecting(b, e) == Some(be));
    assert!(gr.edge_connecting(c, e) == Some(ce));
    assert!(gr.edge_connecting(d, e) == Some(de));
    assert!(gr.edge_connecting(d, f) == Some(df));
    assert!(gr.edge_connecting(f, e) == Some(fe));
    assert!(gr.edge_connecting(f, g) == Some(fg));
    assert!(gr.edge_connecting(e, g) == Some(eg));

    assert!(gr.outgoing_edge_connecting(a, b).unwrap() == ab);
    assert!(gr.incoming_edge_connecting(b, a).unwrap() == ab);
}
