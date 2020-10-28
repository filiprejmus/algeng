use std::iter::once;

use crate::*;

#[derive(Clone)]
pub struct CliqueLB;

impl Idea for CliqueLB {
    fn new(_: &Graph) -> Self {
        CliqueLB
    }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        2
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
         let mut cliques: Vec::<HashSet<_>> = Default::default();
         let g = &state.graph;
         let mut vert_arr: Vec::<usize> = g.vertices().collect();
         let n = vert_arr.len();

         vert_arr.sort_unstable_by_key(|&v| g.get_degree(v));
         for ver in vert_arr.iter() {
            if cliques.is_empty(){
                cliques.push(once(*ver).collect());
            }
            let hs: &HashSet<Vx> = &g.neighbours_hashset(*ver).unwrap();
            for i in 0..cliques.len() {
                if cliques[i].is_subset(hs) {
                    cliques[i].insert(*ver);
                    break;
                } else if i == (cliques.len() - 1) {
                    cliques.push(once(*ver).collect());
                }
            }
         }
         let k = n - cliques.len();

         return state.lb(k);
    }
}
