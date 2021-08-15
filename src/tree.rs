use generational_arena::prelude::TypedArena;

pub struct TreeNode<N> {
    pub weight: N,
    depth: usize,
    // this is analogous to incoming
    parent: Option<TreeNodeIndex<N>>,
    sibling: Option<TreeNodeIndex<N>>,
    first_child: Option<TreeNodeIndex<N>>,
}

impl<N> TreeNode<N> {
    pub fn depth(&self) -> usize {
        self.depth
    }
}

pub type TreeNodeIndex<N> = generational_arena::TypedIndex<TreeNode<N>>;

pub struct Siblings<'a, N> {
    current: Option<TreeNodeIndex<N>>,
    tree: &'a Tree<N>,
}

// impl<'a, N> Siblings<'a, N> {
//     fn s(&self) {
//         // let current = &self.tree[self.current];
//     }
// }

impl<'a, N: std::fmt::Debug> Iterator for Siblings<'a, N> {
    // type Item = &'a TreeNode<N>;
    type Item = TreeNodeIndex<N>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
        // if let Some(idx) = self.current {
        //     self.current = self.tree[idx].sibling;
        //     Some(idx)
        // } else {
        //     None
        // }
    }
}

pub struct Children<'a, N> {
    current: Option<TreeNodeIndex<N>>,
    tree: &'a Tree<N>,
}

impl<'a, N: std::fmt::Debug> Iterator for Children<'a, N> {
    type Item = TreeNodeIndex<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.current {
            self.current = self.tree[idx].sibling;
            Some(idx)
        } else {
            None
        }
    }
}

pub struct Tree<N> {
    root: Option<TreeNodeIndex<N>>,
    nodes: TypedArena<TreeNode<N>>,
}

impl<N: std::fmt::Debug> Tree<N> {
    pub fn new() -> Self {
        Self {
            root: None,
            nodes: Default::default(),
        }
    }

    pub fn root(&self) -> Option<TreeNodeIndex<N>> {
        self.root
    }

    #[must_use]
    pub fn add_node(
        &mut self,
        parent: impl Into<Option<TreeNodeIndex<N>>>,
        weight: N,
    ) -> TreeNodeIndex<N> {
        let parent = parent.into();

        if let Some(p) = parent {
            let parent_node = &self[p];
            let first_child = parent_node.first_child;
            let depth = parent_node.depth;

            let node = TreeNode {
                weight,
                parent: Some(p),
                sibling: first_child,
                first_child: None,
                depth: depth + 1,
            };
            let idx = self.nodes.insert(node);
            let parent_node = &mut self[p];
            parent_node.first_child = Some(idx);
            idx
        } else {
            // todo!()
            assert!(self.root.is_none());
            assert!(self.nodes.is_empty());
            // let node = TreeNode {
            //     weight,
            //     parent,
            //     sibling: None,
            //     first_child: None,
            // };
            // self.nodes.insert(node);

            let node = TreeNode {
                weight,
                parent: None,
                sibling: None,
                first_child: None,
                depth: 0,
            };
            let idx = self.nodes.insert(node);
            self.root = Some(idx);
            idx
        }
        // idx
    }

    pub fn children(&self, node: TreeNodeIndex<N>) -> Children<'_, N> {
        let current = self[node].first_child;
        Children {
            current,
            tree: self,
        }
    }

    pub fn idx(&self, index: TreeNodeIndex<N>) -> &TreeNode<N> {
        &self[index]
    }

    pub fn siblings(&self, node: TreeNodeIndex<N>) -> Siblings<'_, N> {
        Siblings {
            current: Some(node),
            tree: self,
        }
    }

    pub fn reparent(&mut self) {
        //
    }

    fn replace_next_of_node(
        &mut self,
        node: TreeNodeIndex<N>,
        replace: TreeNodeIndex<N>,
        with: TreeNodeIndex<N>,
    ) {
        let next = &self.nodes[node];
    }

    pub fn remove_node(&mut self, index: TreeNodeIndex<N>) {
        if Some(index) == self.root {
            //
            todo!();
        } else {
            let node = &self[index];

            loop {
                let node = &self[index];
                if let Some(child) = node.first_child {
                    //
                    self.remove_node(child);
                }
            }
            self.nodes.remove(index);
            // let parent = &self[node.]
            // remove the node from the sibling list and from the
        }
        todo!()
    }

    // pub fn weight(&self) -
}

impl<N: std::fmt::Debug> std::ops::Index<TreeNodeIndex<N>> for Tree<N> {
    type Output = TreeNode<N>;
    fn index(&self, index: TreeNodeIndex<N>) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<N: std::fmt::Debug> std::ops::IndexMut<TreeNodeIndex<N>> for Tree<N> {
    fn index_mut(&mut self, index: TreeNodeIndex<N>) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

// impl<'a, N: std::fmt::Debug> std::ops::Index<TreeNodeIndex<N>> for &'a Tree<N> {
//     type Output = TreeNode<N>;
//     fn index(&self, index: TreeNodeIndex<N>) -> &Self::Output {
//         &self.nodes[index]
//     }
// }

// impl<'a, N: std::fmt::Debug> std::ops::IndexMut<TreeNodeIndex<N>> for &'a Tree<N> {
//     fn index_mut(&mut self, index: TreeNodeIndex<N>) -> &mut Self::Output {
//         &mut self.nodes[index]
//     }
// }
