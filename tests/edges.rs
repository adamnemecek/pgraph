// mod shared;
use pgraph::prelude::*;

#[test]
fn test_edges() {
    let mut gr = Graph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let d = gr.add_node("D");
    let e = gr.add_node("E");
    let f = gr.add_node("F");
    let g = gr.add_node("G");
    gr.add_edge(a, b, 7.);
    gr.add_edge(a, d, 5.);
    gr.add_edge(d, b, 9.);
    gr.add_edge(b, c, 8.);
    gr.add_edge(b, e, 7.);
    gr.add_edge(c, e, 5.);
    gr.add_edge(d, e, 15.);
    gr.add_edge(d, f, 6.);
    gr.add_edge(f, e, 8.);
    gr.add_edge(f, g, 11.);
    gr.add_edge(e, g, 9.);

    // test incoming
    {
        let expected = ["B", "C", "D", "F"];
        let mut result = vec![];
        for (_, edge) in gr.incoming_edges(e) {
            result.push(gr[edge.from()].weight);
        }
        result.sort();
        assert!(expected.iter().eq(result.iter()), "failed outgoing for e");
    }

    // test outgoing
    {
        let expected = ["B", "E", "F"];
        let mut result = vec![];
        for (_, edge) in gr.outgoing_edges(d) {
            result.push(gr[edge.to()].weight);
        }
        result.sort();
        assert!(expected.iter().eq(result.iter()), "failed outgoing for d");
    }
}
