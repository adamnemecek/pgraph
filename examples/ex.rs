use pgraph::prelude::*;

fn main() {
    let mut gr = Graph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    let edge = gr.add_edge(c, b, 2);
    let edge = gr.add_edge(a, b, 2);

    for e in gr.edges_incoming(b) {
        let a = &gr[e.from()];
        let b = &gr[e.to()];
        println!("{:?} {:?}", a.weight, b.weight);
    }
}