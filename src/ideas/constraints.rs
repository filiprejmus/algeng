//Packing constr. reduction - see 5.4 here: https://arxiv.org/pdf/1411.2680.pdf
use crate::*;

#[derive(Clone)]
pub struct ConstraintReductions(HashSet<Vx>);

impl Idea for ConstraintReductions {
    fn new(_g: &Graph) -> Self { ConstraintReductions(Default::default())}

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        2
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        for i in 0..state.constraints_vec.len(){
            if self.0.contains(&i) {continue;} //TODO: Think about a smarter way
            let k = state.diff_vec[i];
            let set = state.constraints_vec[i].clone();


            if k == 0{
                self.0.insert(i);
                //we cannot include any more vertices from set to satisfy constraint!
                //prune search if remaining edge in set ...
                for v in &set{
                    if set.iter().filter(|&w| *v<*w).any(|&w| state.graph.get_edge(*v, w)){return None;}
                }

                //otherwise, delete set from the graph while including N(set)
                let mut n_s:HashSet<Vx> = Default::default();
                for v in &set{ n_s.extend(state.graph.neighbours(*v));}

                //introduce new constraint for u with N(u) ∩ S = {w}
                if !cfg!(feature = "exclude-constraints") {
                    let mut candidates: HashSet<Vx> = Default::default();
                    for v in &set {
                        candidates.extend(state.graph.neighbours(*v));//u has to be a neighbour of at least one vertex in S
                    }
                    candidates.retain(|&u| state.graph.neighbours_hashset(u).unwrap().intersection(&set).nth(1).is_none()); //at most one neighbour

                    for u in &candidates {
                        let mut neigh_private: HashSet<Vx> = state.graph.neighbours(*u).collect();
                        for v in &set {
                            neigh_private.remove(v);
                            for w in state.graph.neighbours(*v){neigh_private.remove(&w); }
                        }

                        let mut diff: usize = neigh_private.len().checked_sub(1)?;
                        for u in &state.solution {
                            if neigh_private.contains(u) {
                                diff = diff.checked_sub(1)?;
                            }
                        }
                        add_constraint(state, neigh_private, diff);
                    }
                }


                state.extend_solution(n_s.into_iter())?;
                state.remove_vertices(set.into_iter());
            }
            else {
                //find u not in S with |S ∩ N(u)|> k
                let mut candidates:HashSet<Vx> = Default::default();
                for v in &set {
                    candidates.extend(state.graph.neighbours(*v).filter(|u| !set.contains(u)));//u has to be a neighbour of at least one vertex in S
                }
                candidates.retain(|&u| state.graph.neighbours_hashset(u).unwrap().intersection(&set).nth(k).is_some());


                for u in &candidates{

                    if !cfg!(feature = "exclude-constraints") {
                        let neighbours: HashSet<Vx> = state.graph.neighbours(*u).collect();
                        let mut diff: usize = neighbours.len().checked_sub(2)?;

                        for v in &state.solution {
                            if neighbours.contains(v) {
                                diff = diff.checked_sub(1)?;
                            }
                        }
                        add_constraint(state, neighbours, diff);
                    }
                    state.add_to_solution(*u)?;


                }

            }
        }
        Some(())

    }
}
fn add_constraint(state: &mut State,s: HashSet<Vx>, diff: usize){ //TODO Move this to api
    if cfg!(feature = "exclude-constraints") { return; }

    let index = state.constraints_vec.len();
    for v in &s{
        state.fingertable.entry(*v).or_insert(Default::default()).insert(index);
    }
    state.constraints_vec.push(s);
    state.diff_vec.push(diff);

}
