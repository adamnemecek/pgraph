use pgraph::prelude::*;

fn main() {
    let mut gr = Graph::new();
    let a = gr.add_node(2);
    let b = gr.add_node(2);
    let edge = gr.add_edge(a, b, 2);
}
