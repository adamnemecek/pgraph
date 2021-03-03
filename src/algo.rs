#[derive(Clone, Debug)]
pub struct Dfs<N, VM> {
    /// The stack of nodes to visit
    pub stack: Vec<N>,
    /// The map of discovered nodes
    pub discovered: VM,
}
#[derive(Clone, Debug)]
pub struct DfsSpace<N, VM> {
    dfs: Dfs<N, VM>,
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


// pub fn toposort<G>(
//     g: G,
//     space: Option<&mut DfsSpace<G::NodeId, G::Map>>,
// ) -> Result<Vec<G::NodeId>, Cycle<G::NodeId>>
// where
//     G: IntoNeighborsDirected + IntoNodeIdentifiers + Visitable,
// {
//     // based on kosaraju scc
//     with_dfs(g, space, |dfs| {
//         dfs.reset(g);
//         let mut finished = g.visit_map();

//         let mut finish_stack = Vec::new();
//         for i in g.node_identifiers() {
//             if dfs.discovered.is_visited(&i) {
//                 continue;
//             }
//             dfs.stack.push(i);
//             while let Some(&nx) = dfs.stack.last() {
//                 if dfs.discovered.visit(nx) {
//                     // First time visiting `nx`: Push neighbors, don't pop `nx`
//                     for succ in g.neighbors(nx) {
//                         if succ == nx {
//                             // self cycle
//                             return Err(Cycle(nx));
//                         }
//                         if !dfs.discovered.is_visited(&succ) {
//                             dfs.stack.push(succ);
//                         }
//                     }
//                 } else {
//                     dfs.stack.pop();
//                     if finished.visit(nx) {
//                         // Second time: All reachable nodes must have been finished
//                         finish_stack.push(nx);
//                     }
//                 }
//             }
//         }
//         finish_stack.reverse();

//         dfs.reset(g);
//         for &i in &finish_stack {
//             dfs.move_to(i);
//             let mut cycle = false;
//             while let Some(j) = dfs.next(Reversed(g)) {
//                 if cycle {
//                     return Err(Cycle(j));
//                 }
//                 cycle = true;
//             }
//         }

//         Ok(finish_stack)
//     })
// }
