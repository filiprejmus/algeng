use crate::*;

impl State {
    pub fn full_clear(&mut self) {
        for h in &mut self.dirty {
            *h = (0..self.graph.n()).collect();
        }

        self.diff_vec = Default::default();
        self.constraints_vec = Default::default();
        self.fingertable = Default::default();
    }

    pub fn make_dirty(&mut self, x: Vx) {
        for h in &mut self.dirty {
            h.insert(x);
        }
    }

    pub fn add_edge(&mut self, a: Vx, b: Vx) {
        self.graph.add_edge(a, b);

        let mut changed: HashSet<Vx> = [a, b].iter().cloned().collect();

        if let Some(hs_a) = self.graph.neighbours_hashset(a) {
            if let Some(hs_b) = self.graph.neighbours_hashset(b) {
                changed.extend(hs_a.intersection(hs_b));
            }
        }

        for c in changed {
            self.make_dirty(c);
        }
    }

    pub fn clear_constraints(&mut self, x: Vx) {
        let indices: HashSet<usize> = match self.fingertable.remove(&x) {
            Some(ind) => ind,
            None => return,
        };

        for i in indices {
            self.diff_vec[i] = 1;

            let mut h = Default::default();
            std::mem::swap(&mut h, &mut self.constraints_vec[i]);

            for y in h {
                if let Some(ft) = self.fingertable.get_mut(&y) {
                    ft.remove(&i);
                }
            }
        }
    }

    pub fn shallow_clear_constraints(&mut self, x: Vx) {
        let indices: HashSet<usize> = match self.fingertable.remove(&x) {
            Some(ind) => ind,
            None => return,
        };

        for i in indices {
            self.constraints_vec[i].remove(&x);
        }
    }

    #[must_use]
    pub fn add_to_solution(&mut self, x: Vx) -> Option<()> {
        #[cfg(debug_assertions)]
        assert!(!self.solution.contains(&x));

        let h = self.graph.rm_vertex(x);
        self.make_dirty(x);
        self.make_iter_dirty(h.into_iter());

        self.solution.push(x);

        if self.solution.len() > self.upper_bound {
            return None;
        }

        // Update constraints
        if !cfg!(feature = "exclude-constraints") {
            if let Some(indices) = self.fingertable.get(&x) {
                for i in indices{
                    if self.diff_vec[*i] == 0 { return None; }
                    else { self.diff_vec[*i] -= 1; }
                }
            }
        }




        Some(())
    }

    pub fn remove_vertex(&mut self, x: Vx) {
        let h = self.graph.rm_vertex(x);
        self.make_dirty(x);
        self.make_iter_dirty(h.into_iter());
    }

    pub fn make_iter_dirty<I: Iterator<Item=Vx>>(&mut self, i: I) {
        for x in i {
            self.make_dirty(x);
        }
    }

    #[must_use]
    pub fn extend_solution<I: Iterator<Item=Vx>>(&mut self, i: I) -> Option<()> {
        for x in i { self.add_to_solution(x)?; }
        Some(())
    }

    pub fn remove_vertices<I: Iterator<Item=Vx>>(&mut self, i: I) {
        for x in i { self.remove_vertex(x); }
    }

    pub fn clear_constraints_iter<I: Iterator<Item=Vx>>(&mut self, i: I) {
        for x in i { self.clear_constraints(x); }
    }

    #[must_use]
    pub fn lb(&self, k: usize) -> Option<()> {
        if self.solution.len() + k > self.upper_bound { return None; }

        Some(())
    }
}
