use crate::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct ILPLB {
    pub temp:ILPTemp,
    pub bipartite_g:Graph,
}
#[derive(Clone, Debug)]
pub struct ILPTemp{
    pub pair_u:Vec<Option<Vx>>,
    pub pair_v:Vec<Option<Vx>>,
    pub dist:Vec<Option<u32>>,
    pub n:usize,
}

impl Idea for ILPLB {
    fn new(g: &Graph) -> Self {

        // TODO I think this needs to be g.n() instead of g.vertices().count()
        let n = g.vertices().count(); //number of vertices in each partition of bipartite graph
        ILPLB {
            temp: ILPTemp {
                pair_u: vec![None; n],
                pair_v: vec![None;n],
                dist: vec![None;n+1],
                n: n,
            },
            bipartite_g: Graph::new(2*n, None.into_iter()) // initially the bipartite_graph is empty, but with Init all vertices will be updated
        }
    }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        2
    }

    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()> {
        //Update partial matching and bipartite graph
        self.update(dirty, &state.graph);

        //return lb
        let mut lb = 0;
        for partner in self.temp.pair_v.iter(){ if *partner != None {lb+=1;}} //get size of matching
        lb = lb/2;

        return state.lb(lb);
    }
}

impl ILPLB {
    fn update(&mut self, dirty: &HashSet<Vx>, g: &Graph){
        let n = self.temp.n;

        // removes changed vertices
        for &u in dirty {
            //update bipartite graph
            self.bipartite_g.rm_vertex(u+n);
            self.bipartite_g.rm_vertex(u);


            //update partiel matching
            if let Some(partner) = self.temp.pair_u[u]{
                self.temp.pair_v[partner-n] = None;
                self.temp.pair_u[u]  = None;
            }

            if let Some(partner) = self.temp.pair_v[u]{
                self.temp.pair_u[partner] = None;
                self.temp.pair_v[u]  = None;
            }

        }

        // re-adds changed vertices
        let n = g.n();
        for &x in dirty {
            for y in g.neighbours(x) {
                self.bipartite_g.add_edge(x, y+n); //{l_u,r_v}
                self.bipartite_g.add_edge(y, x+n); //{l_v,r_u}
            }
        }

        //update the matching to an maximum matching
        hopcroft_karp(&self.bipartite_g,&mut self.temp);
    }
}

impl ILPTemp{
    pub fn dist(&self, u:Option<Vx>)-> Option<u32>{
        let index = u.unwrap_or(self.n);
        self.dist[index]
    }
    pub fn set_dist(&mut self,u:Option<Vx>, dist:Option<u32>){
        let index = u.unwrap_or(self.n);
        self.dist[index] =  dist;

    }
    pub fn pair_v(&self,v:Vx)->Option<Vx> {self.pair_v[v-self.n]}
    pub fn pair_u(&self,u:Vx)->Option<Vx> {self.pair_u[u]}
}

pub fn hopcroft_karp(g:&Graph, mut temp: &mut ILPTemp){
    //Contructs maximum matching using the matching_partial as an initial matching.
    //bipartite graph G=U ∪ V with U={0,...,n-1} and V={n,...,2n-1} constructed from LP

    let n = temp.n; //number of vertices in each partition (size is equal by construction)
    for val in temp.dist.iter_mut() {*val = None;}//reset distances

    while bfs(g,&mut temp){
        for u in 0..n{
            if temp.pair_u(u) == None {dfs(g,Some(u), &mut temp);}

        }
    }




}
fn bfs(g:&Graph,  temp:&mut ILPTemp) -> bool{

    let n = temp.n;
    let mut queue: VecDeque<Vx> = VecDeque::new(); //TODO: Maybe re-use queue and only clear it
    for u in 0..n{
        if temp.pair_u(u) == None{
            temp.dist[u]=Some(0);
            queue.push_back(u);
        }
        else {
            temp.dist[u] = None;
        }
    }
    temp.set_dist(None, None); //Dist[NIL] := ∞
    while let Some(u) = queue.pop_front(){
        for v in g.neighbours(u){
            if temp.dist(temp.pair_v(v)) == None{
                temp.set_dist(temp.pair_v(v),add(temp.dist(Some(u)),Some(1)));
                if let Some(partner) = temp.pair_v(v)  {queue.push_back(partner);}
            }
        }
    }
    temp.dist(None)!= None //return Dist[NIL] ≠ ∞

}
fn dfs(g:&Graph, u_:Option<Vx>, temp:&mut ILPTemp) -> bool{
    let n = temp.n;
    if let Some (u) = u_ {
        for v in g.neighbours(u) {
            if temp.dist(temp.pair_v(v)) == add(temp.dist(u_),Some(1)){
                if dfs(g, temp.pair_v(v),temp ){
                    temp.pair_v[v-n] = Some(u);
                    temp.pair_u[u] = Some(v);
                    return true;
                }
            }
        }
        temp.dist[u] = None;
        return false;
    }
    return true;

}

fn add(x_:Option<u32>,y_:Option<u32>)->Option<u32>{
    //Returns x+y
    if let (Some(x), Some(y)) = (x_, y_){ return Some(x+y); }
    None //at least one is inf. so result is infinite
}

/* // apparently this function is used anymore
fn larger(x_:Option<u32>,y_:Option<u32>)->bool{
    //Returns true iff. x>y and treats None as inf.
    if let Some(y) = y_{
        if let Some(x) = x_ { return x>y;}
        return true; //x is infinity but not y
    }
    return false; //y is infinity
}
*/
