use pgraph::prelude::*;

fn main() {
    let mut gr = Graph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");

    let gr2 = gr.clone();
    // let c = gr.add_node("C");
    // let d = gr.add_node("D");
    // let edge = gr.add_edge(c, b, 2);
    let edge = gr.add_edge(a, b, 2);
    // let edge = gr.add_edge(d, b, 2);

    // for e in gr.edges_incoming(b) {
    //     let a = &gr[e.from()];
    //     let b = &gr[e.to()];
    //     println!("{:?} {:?}", a.weight, b.weight);
    // }
    gr.remove_edge(edge);

    // println!("graph {:?}", gr);
    // println!("graph2 {:?}", gr2);
    println!("{:?}", gr == gr2);
}
