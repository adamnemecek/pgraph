pub struct TreeNode<N> {
    pub weight: N,
    // this is analogous to incoming
    parent: Option<TreeNodeIndex<N>>,
    sibling: Option<TreeNodeIndex<N>>,
    first_child: Option<TreeNodeIndex<N>>,
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
        // let current = &self.tree.idx(self.current);
        // if let Some(current) = self.current {
        //     let ret = &self.tree[current];
        //     Some(ret)
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
            let current = self.tree[idx].sibling;
            self.current = current;
            current
        } else {
            None
        }
    }
}

pub struct Tree<N> {
    root: Option<TreeNodeIndex<N>>,
    nodes: generational_arena::Arena<TreeNode<N>>,
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

            let node = TreeNode {
                weight,
                parent: Some(p),
                sibling: first_child,
                first_child: None,
            };
            let idx = self.nodes.typed_insert(node);
            let parent_node = &mut self[p];
            parent_node.first_child = Some(idx);
            idx
        } else {
            // todo!()
            assert!(self.root.is_none());
            // let node = TreeNode {
            //     weight,
            //     parent,
            //     sibling: None,
            //     first_child: None,
            // };
            // self.nodes.typed_insert(node);

            let node = TreeNode {
                weight,
                parent: None,
                sibling: None,
                first_child: None,
            };
            let idx = self.nodes.typed_insert(node);
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

    pub fn remove_node(&mut self, index: TreeNodeIndex<N>) {
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
