use crate::*;

#[derive(Clone)]
pub struct TwoClique(HashSet<Vx>);

fn partition_neighbourhood(g: &Graph, v: Vx) -> Option<(HashSet<Vx>, HashSet<Vx>, HashMap<Vx, HashSet<Vx>>)> {
    let mut neigh: HashSet<Vx> = g.neighbours(v).collect();
    if neigh.len() < 2 { return None; }
    
    let mut m: HashMap<Vx, HashSet<Vx>> = Default::default();
    for &n in &neigh {
        m.insert(n, Default::default());
    }

    for &n1 in &neigh {
        for &n2 in &neigh {
            if n1 >= n2 { continue; }
            if g.get_edge(n1, n2) { continue; }

            m.get_mut(&n1).unwrap().insert(n2);
            m.get_mut(&n2).unwrap().insert(n1);
        }
    }

    let (mut c1, mut c2): (HashSet<Vx>, HashSet<Vx>) = (Default::default(), Default::default());

    // decompose neigh c1 and c2
    loop {
        if neigh.is_empty() { break; }

        let n = *neigh.iter().next().unwrap();
        neigh.remove(&n);

        let cnt = m.get(&n).unwrap().len();
        if cnt == 1 {
            c1.insert(n);

            let n_neighs: &HashSet<Vx> = &m.get(&n).expect("but we added this directly after the declaration of m!");
            let single_neigh: Vx = *n_neighs.iter().next().expect("but it has degree = 1!");

            // this enfcrces that single_neigh is in c2.
            if neigh.contains(&single_neigh) {
                neigh.remove(&single_neigh);
                c2.insert(single_neigh);
            }
            #[cfg(debug_assertions)]
            assert!(c2.contains(&single_neigh));
        } else {
            c2.insert(n);
        }
    }

    if c1.len() < c2.len() { return None; }
    for &a in &c2 {
        for &b in &c2 {
            if a != b && !g.get_edge(a, b) { return None; }
        }
    }

    // assertions
    #[cfg(debug_assertions)] {
        for &a in &c1 {
            for &b in &c1 {
                if a != b { assert!(g.get_edge(a, b)); }
            }
        }

        for &a in &c1 {
            let mut cnt = 0;
            for &b in &c2 {
                let bit = !g.get_edge(a, b);
                let bit2 = {
                    let l: HashSet<Vx> = m.get(&a).unwrap().clone();
                    assert!(l.len() == 1);
                    *l.iter().next().unwrap() == b
                };
                assert_eq!(bit, bit2);

                if bit { cnt += 1; }
            }
            assert!(cnt == 1);
        }
    }

    return Some((c1, c2, m));
}

impl Idea for TwoClique {
    fn new(g: &Graph) -> Self { TwoClique(g.vertices().collect()) }

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

            if let Some((c1, c2, m)) = partition_neighbourhood(&state.graph, x) {
                for &c1_v in &c1 {
                    let single_neigh = *m.get(&c1_v).unwrap().iter().next().unwrap();
                    let neinei: Vec<Vx> = state.graph.neighbours(single_neigh).collect();
                    for nn in neinei {
                        state.add_edge(c1_v, nn);
                    }
                }

                state.remove_vertex(x);
                for &c2_v in &c2 { state.add_to_solution(c2_v)?; }

                state.shallow_clear_constraints(x);
                for &s in &c1 { state.shallow_clear_constraints(s); }
                for &s in &c2 { state.shallow_clear_constraints(s); }

                state.cleanup_funcs.push(Box::new(move |solution: &mut Vec<Vx>| {
                    #[cfg(debug_assertions)]
                    assert!(c2.iter().all(|w| solution.contains(w)));

                    if let Some(&c1_left) = c1.iter().find(|w| !solution.contains(*w) ) { // if a vertex from c1 does not occur in the solution, then its single_neigh doesn't need to be in the solution, hence replace it with x
                        let single_neigh = *m.get(&c1_left).unwrap().iter().next().unwrap();
                        solution.retain(|w| *w != single_neigh); // remove single_neigh
                        solution.push(x);
                    }
                }));
            }
        }
    }
}
