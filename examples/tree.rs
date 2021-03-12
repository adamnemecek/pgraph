use pgraph::Tree;
fn main() {
    let mut tree = Tree::new();
    let root = tree.add_node(None, 0);
    let a = tree.add_node(root, 1);
}
