use pgraph::prelude::*;

#[test]
fn test_outgoing_neighbors() {
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

    for e in gr.outgoing_neighbors(a) {
        println!("{:?}", gr[e].weight);
    }
}