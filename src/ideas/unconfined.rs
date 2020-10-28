use crate::*;

#[derive(Clone)]
pub struct Unconfined(bool);

fn is_unconfined(g: &Graph, v: Vx) -> bool {
    let mut s: HashSet<Vx> = Default::default();
    let mut n_s: HashSet<Vx> = Default::default();
    let mut w_: Option<usize> = Some(v); //init s={v}

    loop {
        let w = w_.expect("w is None?");
        s.insert(w);
        if let Some(h) = g.neighbours_hashset(w) { n_s.extend(h); }

        n_s.insert(w); //closed neighbourhood


        w_ = None;
        for u in &n_s {
            if s.contains(u) { continue; }
            if let Some(n_u) = g.neighbours_hashset(*u){
                let mut intersection = n_u.intersection(&s);
                if intersection.next() != None && intersection.next() == None { //intersection has exactly one elem.
                    let mut diff = n_u.difference(&n_s);
                    let temp = diff.next();
                    if temp == None { return true; }  //diff. is empty
                    if diff.next() == None {//diff. has size exactly 1
                        w_ = Some(*temp.expect("temp is None?"));
                    }
                }

            }

        }
        if w_ == None { return false; }
    }
}

impl Idea for Unconfined {
    fn new(_: &Graph) -> Self {
        Unconfined(true)
    }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if !self.0 && dirty.is_empty() { return 0; }

        3
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        self.0 = true;

        let v_ = state.graph.vertices().find(|&v| is_unconfined(&state.graph, v));
        if let Some(v) = v_ {
            state.add_to_solution(v)?;
        } else {
            self.0 = false;
        }

        Some(())
    }
}

