use crate::{
    prelude::*,
    VisitMap,
};

#[derive(Clone, Debug)]
pub struct Dfs<N> {
    /// The stack of nodes to visit
    pub stack: Stack<N>,
    /// The map of discovered nodes
    pub discovered: VisitMap<N>,
}

impl<N: std::fmt::Debug, E: std::fmt::Debug> Dfs<NodeIndex<N, E>> {
    pub fn empty(graph: &Graph<N, E>) -> Self {
        Self {
            stack: Stack::new(),
            discovered: graph.visit_map(),
        }
    }

    pub fn reset(&mut self, graph: &Graph<N, E>) {
        graph.reset_map(&mut self.discovered);
        self.stack.clear();
    }

    pub fn move_to(&mut self, start: NodeIndex<N, E>) {
        self.stack.clear();
        self.stack.push(start);
    }

    pub fn next(&mut self, graph: impl GraphKind<N, E>) -> Option<NodeIndex<N, E>> {
        while let Some(node) = self.stack.pop() {
            if self.discovered.visit(node) {
                for succ in graph.neighbors(node) {
                    if !self.discovered.is_visited(succ) {
                        self.stack.push(succ);
                    }
                }
                return Some(node);
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct DfsSpace<N> {
    dfs: Dfs<N>,
}

// fn with_dfs<G, F, R>(g: G, space: Option<&mut DfsSpaceType<G>>, f: F) -> R
// where
//     G: GraphRef + Visitable,
//     F: FnOnce(&mut Dfs<G::NodeId, G::Map>) -> R,
// {
//     let mut local_visitor;
//     let dfs = if let Some(v) = space {
//         &mut v.dfs
//     } else {
//         local_visitor = Dfs::empty(g);
//         &mut local_visitor
//     };
//     f(dfs)
// }

fn with_dfs<N: std::fmt::Debug, E: std::fmt::Debug, F, R>(
    g: &Graph<N, E>,
    space: Option<&mut DfsSpace<NodeIndex<N, E>>>,
    f: F,
) -> R
where
    F: FnOnce(&mut Dfs<NodeIndex<N, E>>) -> R,
{
    let mut local_visitor;
    let dfs = if let Some(v) = space {
        &mut v.dfs
    } else {
        local_visitor = Dfs::empty(g);
        &mut local_visitor
    };
    f(dfs)
}

pub fn toposort<N: std::fmt::Debug, E: std::fmt::Debug>(
    g: &Graph<N, E>,
    space: Option<&mut DfsSpace<NodeIndex<N, E>>>,
) -> Result<Vec<NodeIndex<N, E>>, GraphError<N, E>>
// where
    // G: IntoNeighborsDirected + IntoNodeIdentifiers + Visitable,
{
    // based on kosaraju scc
    with_dfs(g, space, |dfs| {
        dfs.reset(&g);
        let mut finished = g.visit_map();

        let mut finish_stack = Vec::new();
        for (i, n) in g.nodes() {
            let i = i.into();
            if dfs.discovered.is_visited(i) {
                continue;
            }
            dfs.stack.push(i);
            while let Some(&nx) = dfs.stack.peek() {
                if dfs.discovered.visit(nx) {
                    // First time visiting `nx`: Push neighbors, don't pop `nx`
                    for succ in g.neighbors(nx) {
                        if succ == nx {
                            // self cycle
                            return Err(GraphError::WouldCycle(nx));
                        }
                        if !dfs.discovered.is_visited(succ) {
                            dfs.stack.push(succ);
                        }
                    }
                } else {
                    dfs.stack.pop();
                    if finished.visit(nx) {
                        // Second time: All reachable nodes must have been finished
                        finish_stack.push(nx);
                    }
                }
            }
        }
        finish_stack.reverse();
        dfs.reset(g);

        for &i in &finish_stack {
            dfs.move_to(i);
            let mut cycle = false;
            while let Some(j) = dfs.next(Reversed::new(g)) {
                if cycle {
                    return Err(GraphError::WouldCycle(j));
                }
                cycle = true;
            }
        }

        Ok(finish_stack)
    })
}
