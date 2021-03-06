use pgraph::prelude::*;

#[test]
fn test_incoming() {
    let mut gr = Graph::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");
    let c = gr.add_node("C");
    gr.add_edge(a, b, 2);
    gr.add_edge(c, b, 2);
    let expected = [a, b];
    let mut result = vec![];
    for e in gr.edges_incoming(c) {
        let a = &gr[e.from()];
        let b = &gr[e.to()];
        // println!("{:?} {:?}", a.weight, b.weight);
        result.push(b.weight);
    }
}


#[test]
fn test_remove_edge() {
    let mut gr = Graph::<&str, u32>::new();
    let a = gr.add_node("A");
    let b = gr.add_node("B");

    
}