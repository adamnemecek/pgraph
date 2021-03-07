use pgraph::prelude::*;

fn main() {
    let mut gr = Graph::new();
    let a = gr.add_node("A"); // 0
    let b = gr.add_node("B"); // 1
    let c = gr.add_node("C"); // 2
    let d = gr.add_node("D"); // 3
    let e = gr.add_node("E"); // 4
    let f = gr.add_node("F"); // 5
    let g = gr.add_node("G"); // 6

    let ab = gr.add_edge(a, b, 0); // 0
    let ad = gr.add_edge(a, d, 1); // 1
    let db = gr.add_edge(d, b, 2); // 2
    let bc = gr.add_edge(b, c, 3); // 3
    let be = gr.add_edge(b, e, 4); // 4
    let ce = gr.add_edge(c, e, 5); // 5
    let de = gr.add_edge(d, e, 6); // 6
    let df = gr.add_edge(d, f, 7); // 7
    let fe = gr.add_edge(f, e, 8); // 8
    let fg = gr.add_edge(f, g, 9); // 9
    let eg = gr.add_edge(e, g, 10); // 10

    assert!(gr.edge_connecting(a, b) == Some(ab.unwrap()));
    assert!(gr.edge_connecting(a, d) == Some(ad.unwrap()));
    assert!(gr.edge_connecting(d, b) == Some(db.unwrap()));
    assert!(gr.edge_connecting(b, c) == Some(bc.unwrap()));
    assert!(gr.edge_connecting(b, e) == Some(be.unwrap()));
    assert!(gr.edge_connecting(c, e) == Some(ce.unwrap()));
    assert!(gr.edge_connecting(d, e) == Some(de.unwrap()));
    assert!(gr.edge_connecting(d, f) == Some(df.unwrap()));
    assert!(gr.edge_connecting(f, e) == Some(fe.unwrap()));
    assert!(gr.edge_connecting(f, g) == Some(fg.unwrap()));
    assert!(gr.edge_connecting(e, g) == Some(eg.unwrap()));
}
