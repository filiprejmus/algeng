use crate::*;

#[derive(Clone)]
pub struct CycleLB;

impl Idea for CycleLB {
    fn new(_: &Graph) -> Self { CycleLB }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        3
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        let mut g = state.graph.clone();
        let mut k = 0;
        loop {
            let opt = g.vertices().find(|&v| g.get_degree(v) == 1);
            if let Some(v) = opt {
                let v2 = g.neighbours(v).next().unwrap();
                g.rm_vertex(v2);
                k += 1;
                continue;
            }
            if g.is_empty() { break; }

            k += (find_greedy_cycle(&mut g) + 1) / 2;
        }

        return state.lb(k);
    }
}

fn find_greedy_cycle(g: &mut Graph) -> usize {

/* // idea was to prefer triangles - didn't scale though
    let mut opti: Option<[Vx; 3]> = None;
    'outi: for v in g.vertices() {
        let neigh: HashSet<Vx> = g.neighbours(v).collect();
        for &x1 in &neigh {
            for &x2 in &neigh {
                if x1 != x2 && g.get_edge(x1, x2) {
                    opti = Some([v, x1, x2]);
                    break 'outi;
                }
            }
        }
    }

    if let Some(l) = opti {
        for &x in &l {
            g.rm_vertex(x);
        }
        return 3;
    }
*/

    let mut vec = Vec::new();
    vec.push(g.vertices().next().unwrap());
    'outer: loop {
        let len = vec.len();
        let x = *vec.last().unwrap();
        let mut opt: Option<usize> = None;
        for y in g.neighbours(x) {
            if len >= 2 && y == vec[len-2] { continue; }
            match vec.iter().position(|&z| z == y) {
                Some(p) => {
                    opt = Some(p);
                    break;
                },
                None => {
                    vec.push(y);
                    continue 'outer;
                }
            }
        }
        if let Some(p) = opt {
            let cycle = &vec[p..];
            for &x in cycle { g.rm_vertex(x); }
            return cycle.len();
        }
    }
}


