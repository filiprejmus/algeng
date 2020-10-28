use crate::*;

pub struct Digraph {
    pub n: usize,
    pub succ: HashMap<Vx, HashSet<Vx>>,
    pub pred: HashMap<Vx, HashSet<Vx>>,
}

impl Digraph {
    pub fn new<E: Iterator<Item=Edge>>(n: usize, edges: E) -> Digraph {
        let mut d = Digraph {
            n: n,
            succ: Default::default(),
            pred: Default::default(),
        };

        for (u,v) in edges {
            d.succ.entry(u).or_insert(Default::default()).insert(v);
            d.pred.entry(v).or_insert(Default::default()).insert(u);
        }

        d
    }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item=Vx> + '_ {
        self.succ.keys().cloned()
    }

    pub fn edges(&self) -> impl Iterator<Item=Edge> + '_ {
        let n = self.n;
        (0..n).map(move |i|
            self.succ.get(&i)
                .map(move |hs|
                    hs.iter()
                        .map(move |&j| (i,j))
                ).into_iter()
                .flatten()
        ).flatten()
    }

    pub fn succ(&self, x: Vx) -> impl Iterator<Item=Vx> + '_ {
        self.succ.get(&x)
            .map(move |hs| hs.iter().cloned())
            .into_iter()
            .flatten()
    }

    pub fn pred(&self, x: Vx) -> impl Iterator<Item=Vx> + '_ {
        self.pred.get(&x)
            .map(move |hs| hs.iter().cloned())
            .into_iter()
            .flatten()
    }

    pub fn reachable(&self, from: Vx) -> HashSet<Vx> {
        let mut hs: HashSet<Vx> = Default::default();
        let mut todo: HashSet<Vx> = Default::default();

        todo.insert(from);

        loop {
            if todo.is_empty() { break; }

            let x = *todo.iter().next().unwrap();
            todo.remove(&x);

            hs.insert(x);

            todo.extend(self.succ(x)
                .filter(|y| !hs.contains(y))
            );
        }

        hs
    }
}
