use crate::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Temp{
    pair:Vec<Option<Vx>>,
    dist:Vec<Option<u32>>,
}

impl Temp{
    pub fn dist(&self, u:Option<Vx>)-> Option<u32>{
        let n = self.dist.len()-1;
        let index = u.unwrap_or(n);
        self.dist[index]
    }
    pub fn set_dist(&mut self,u:Option<Vx>, dist:Option<u32>){
        let n = self.dist.len()-1;
        let index = u.unwrap_or(n);
        self.dist[index] =  dist;

    }

}



pub fn max_matching_bipartite(g: &Graph, l: &HashSet<Vx>)->HashSet<Edge>{
    let n = g.n();
    let mut temp = Temp{
        pair: vec![None; n],
        dist: vec![None;n+1], //TODO: We can save space here since we only need dist for u in l
    };//TODO: can we reuse info here?

    hopcroft_karp(g,&mut temp, l);

    let mut matching: HashSet<Edge> = Default::default();
    for u_ in l.iter(){
        let u = *u_;
        if let Some(partner) = temp.pair[u]{
            if u<partner{
                matching.insert((u,partner));
            }
            else{
                matching.insert((partner,u));
            }
        }
    }
    matching


}

fn hopcroft_karp(g:&Graph, mut temp: &mut Temp, l: &HashSet<Vx>){
    //for val in temp.dist.iter_mut() {*val = None;}//reset distances TODO: Uncomment this when reusing previous results!
    while bfs(g,&mut temp,l){
        for u in l.iter(){
            if temp.pair[*u] == None {dfs(g,Some(*u), temp);}

        }
    }

}

fn bfs(g:&Graph,  temp:&mut Temp, l: &HashSet<Vx>) -> bool{
    let n = temp.dist.len()-1; //index of NIL in dist
    let mut queue: VecDeque<Vx> = VecDeque::new(); //TODO: Maybe re-use queue and only clear it
    for u_ in l.iter(){
        let u = *u_;
        if temp.pair[u] == None{
            temp.dist[u]=Some(0);
            queue.push_back(u);
        }
        else {
            temp.dist[u] = None;
        }
    }
    temp.dist[n] = None; //Dist[NIL] := ∞
    while let Some(u) = queue.pop_front(){
        for v in g.neighbours(u){
            if temp.dist(temp.pair[v]) == None{
                temp.set_dist(temp.pair[v],add(temp.dist(Some(u)),Some(1)));
                if let Some(partner) = temp.pair[v]  {queue.push_back(partner);}
            }
        }
    }
    temp.dist(None)!= None //return Dist[NIL] ≠ ∞

}
fn dfs(g:&Graph, u_:Option<Vx>, temp:&mut Temp) -> bool{
    if let Some (u) = u_ {
        for v in g.neighbours(u) {
            if temp.dist(temp.pair[v]) == add(temp.dist(u_),Some(1)){
                if dfs(g, temp.pair[v],temp ){
                    temp.pair[v] = Some(u);
                    temp.pair[u] = Some(v);
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
