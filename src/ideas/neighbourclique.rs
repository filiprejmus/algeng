use crate::*;

#[derive(Clone)]
pub struct NeighbourClique(HashSet<Vx>);

impl Idea for NeighbourClique {
    fn new(g: &Graph) -> Self {
        NeighbourClique(g.vertices().collect())
    }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if self.0.is_empty() && dirty.is_empty() { return 0; }

        5
    }

    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()> {
        self.0.extend(dirty.iter().cloned());
        'outer: loop {
            let x: Vx = if let Some(&x) = self.0.iter().next() { x }
                        else { return Some(()); };
            self.0.remove(&x);
            let neighbours: Vec<Vx> = state.graph.neighbours(x).collect();
            if neighbours.is_empty() { continue; }
            for i in 0..neighbours.len() {
                for j in (i+1)..neighbours.len() {
                    if !state.graph.get_edge(neighbours[i], neighbours[j]) {
                       self.0.remove(&x);
                       continue 'outer;
                    }
                }
            }
            state.extend_solution(neighbours.into_iter())?;
            state.remove_vertex(x);
            return Some(())
        }
    }
}
