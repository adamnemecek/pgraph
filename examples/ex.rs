use pgraph::prelude::*;

// fn main() {
//     let mut gr = Graph::new();
//     let a = gr.add_node("A");
//     let b = gr.add_node("B");

//     let gr2 = gr.clone();
//     // let c = gr.add_node("C");
//     // let d = gr.add_node("D");
//     // let edge = gr.add_edge(c, b, 2);
//     let edge = gr.add_edge(a, b, 2).unwrap();
//     // let edge = gr.add_edge(d, b, 2);

//     // for e in gr.edges_incoming(b) {
//     //     let a = &gr[e.from()];
//     //     let b = &gr[e.to()];
//     //     println!("{:?} {:?}", a.weight, b.weight);
//     // }
//     gr.remove_edge(edge);

//     // println!("graph {:?}", gr);
//     // println!("graph2 {:?}", gr2);
//     println!("{:?}", gr == gr2);
// }

fn check_edges<'a>(
    graph: &Graph<&'a str, u32>,
    direction: Direction,
    node: NodeIndex<&'a str, u32>,
    expected: &[&str],
) {
    let edges = match direction {
        Direction::Outgoing => graph.outgoing_edges(node),
        Direction::Incoming => graph.incoming_edges(node),
    };

    let mut result: Vec<_> = edges.map(|(idx, edge)| {
        let node = match direction {
            Direction::Outgoing => &graph[edge.to()],
            Direction::Incoming => &graph[edge.from()],
        };
        node.weight.to_owned()
    }).collect();
    result.sort();

    let msg = format!("expected: {:?}, result: {:?}", expected, result);
    assert!(result.iter().eq(expected.iter()), msg)

}

fn main() {
    let mut gr = Graph::new();
    let a = gr.add_node("A"); // 0
    let b = gr.add_node("B"); // 1
    let c = gr.add_node("C"); // 2
    let d = gr.add_node("D"); // 3
    let e = gr.add_node("E"); // 4
    let f = gr.add_node("F"); // 5
    let g = gr.add_node("G"); // 6
    let _ = gr.add_edge(a, b, 7.); // 0
    let _ = gr.add_edge(a, d, 5.); // 1
    let _ = gr.add_edge(d, b, 9.); // 2
    let _ = gr.add_edge(b, c, 8.); // 3
    let _ = gr.add_edge(b, e, 7.); // 4
    let _ = gr.add_edge(c, e, 5.); // 5
    let _ = gr.add_edge(d, e, 15.); // 6
    let _ = gr.add_edge(d, f, 6.); // 7
    let _ = gr.add_edge(f, e, 8.); // 8
    let _ = gr.add_edge(f, g, 11.); // 9
    let _ = gr.add_edge(e, g, 9.); // 10

    // assert!(gr.node_count() == 7);
    // assert!(gr.edge_count() == 11);

    gr.remove_node(e);
    println!("graph: {:?}", gr);

    // assert!(gr.node_count() == 6);
    // assert!(gr.edge_count() == 6);

    gr.remove_node(f);
}

// the problem happens when you remove the node e,
// it doesn't remove the e -> f edge from the f node list
