use crate::*;

#[derive(Clone)]
pub struct Deg1(HashSet<Vx>);

impl Idea for Deg1 {
    fn new(_g: &Graph) -> Self { Deg1(Default::default()) }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() && self.0.is_empty() { return 0; }

        4
    }

    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()> {
        self.0.extend(dirty.iter().cloned());
        loop {
            let x: Vx = if let Some(&x) = self.0.iter().next() { x }
                        else { return Some(()); };
            self.0.remove(&x);

            if state.graph.get_degree(x) == 1 {
                let neighbour = state.graph.neighbours(x).next().expect("it says deg=1!");
                state.add_to_solution(neighbour)?;
            }
        }
    }
}
