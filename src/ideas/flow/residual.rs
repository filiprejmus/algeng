use crate::*;
use super::Digraph;

impl Digraph {
    pub fn residual<Cap: Fn(Edge) -> usize>(&self, f: &HashSet<Edge>, cap: Cap) -> Digraph {
        let edges = self.edges()
            .map(|(u,v)| {
                let current_flow = f.contains(&(u,v)) as usize;

                #[cfg(debug_assertions)]
                assert!(current_flow <= cap((u,v)));

                Some((u,v))
                    .filter(|&e| current_flow < cap(e))
                    .into_iter().chain(
                Some((v,u))
                    .filter(|_| current_flow > 0)
                    .into_iter()
                )
            }).flatten();
        Digraph::new(self.n, edges)
    }
}
