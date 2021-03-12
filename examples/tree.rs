use pgraph::Tree;
fn main() {
    let mut tree = Tree::new();
    let root = tree.add_node(None, 0);
    let a = tree.add_node(root, 1);

    let b = tree.add_node(root, 2);

    for child in tree.children(root) {
        let node = tree[child].weight;
        println!("weight: {:?}", node);
    }
}
