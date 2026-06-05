#![forbid(unsafe_code)]

pub struct UnionFind { parent: Vec<usize>, rank: Vec<usize>, size: Vec<usize> }
impl UnionFind {
    pub fn new(n: usize) -> Self { Self { parent: (0..n).collect(), rank: vec![0;n], size: vec![1;n] } }
    pub fn find(&mut self, x: usize) -> usize { 
        let p = self.parent[x];
        if p != x { let r = self.find(p); self.parent[x] = r; r } else { x }
    }
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a); let rb = self.find(b);
        if ra == rb { return false; }
        let rank_a = self.rank[ra]; let rank_b = self.rank[rb];
        if rank_a < rank_b { self.parent[ra] = rb; self.size[rb] += self.size[ra]; }
        else if rank_a > rank_b { self.parent[rb] = ra; self.size[ra] += self.size[rb]; }
        else { self.parent[rb] = ra; self.size[ra] += self.size[rb]; self.rank[ra] += 1; }
        true
    }
    pub fn component_size(&mut self, x: usize) -> usize { let r = self.find(x); self.size[r] }
}

pub struct TernaryLattice { pub n: usize, pub edges: Vec<(usize, usize, i8)>, pub signs: Vec<i8> }
impl TernaryLattice {
    pub fn new(n: usize) -> Self { Self { n, edges: Vec::new(), signs: vec![0; n] } }
    pub fn add_edge(&mut self, from: usize, to: usize, sign: i8) { self.edges.push((from, to, sign)); }
    pub fn percolates(&self) -> bool {
        let mut uf = UnionFind::new(self.n);
        for &(a, b, _) in &self.edges { uf.union(a, b); }
        (0..self.n).map(|i| uf.component_size(i)).max().unwrap_or(0) as f64 / self.n as f64 > 0.5
    }
    pub fn critical_density(&self) -> f64 { if self.n <= 1 { 0.0 } else { self.edges.len() as f64 / (self.n * (self.n - 1) / 2) as f64 } }
    pub fn largest_cluster(&self) -> usize {
        let mut uf = UnionFind::new(self.n);
        for &(a, b, _) in &self.edges { uf.union(a, b); }
        (0..self.n).map(|i| uf.component_size(i)).max().unwrap_or(0)
    }
    pub fn count_components(&self) -> usize {
        let mut uf = UnionFind::new(self.n);
        for &(a, b, _) in &self.edges { uf.union(a, b); }
        (0..self.n).map(|i| uf.find(i)).collect::<std::collections::HashSet<usize>>().len()
    }
    pub fn signed_clusters(&self) -> (usize, usize) {
        let mut uf = UnionFind::new(self.n);
        for &(a, b, _) in &self.edges { uf.union(a, b); }
        let roots: Vec<usize> = (0..self.n).map(|i| uf.find(i)).collect();
        let unique: std::collections::HashSet<usize> = roots.iter().copied().collect();
        let signs = &self.signs;
        let pos = unique.iter().filter(|&&r| signs[r] > 0).count();
        (pos, unique.len() - pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_uf_basic() { let mut uf = UnionFind::new(5); uf.union(0,1); uf.union(1,2); assert_eq!(uf.find(0), uf.find(2)); }
    #[test] fn test_uf_separate() { let mut uf = UnionFind::new(5); assert_ne!(uf.find(0), uf.find(3)); }
    #[test] fn test_uf_size() { let mut uf = UnionFind::new(5); uf.union(0,1); uf.union(0,2); assert_eq!(uf.component_size(0), 3); }
    #[test] fn test_creation() { let lat = TernaryLattice::new(10); assert_eq!(lat.edges.len(), 0); }
    #[test] fn test_add_edge() { let mut lat = TernaryLattice::new(5); lat.add_edge(0, 1, 1); assert_eq!(lat.edges.len(), 1); }
    #[test] fn test_no_percolation() { let lat = TernaryLattice::new(10); assert!(!lat.percolates()); }
    #[test] fn test_full_percolation() { let mut lat = TernaryLattice::new(4); for i in 0..4 { for j in i+1..4 { lat.add_edge(i,j,1); } } assert!(lat.percolates()); }
    #[test] fn test_density() { let mut lat = TernaryLattice::new(4); lat.add_edge(0,1,1); assert!(lat.critical_density() > 0.0); }
    #[test] fn test_largest() { let mut lat = TernaryLattice::new(5); lat.add_edge(0,1,1); lat.add_edge(1,2,1); assert_eq!(lat.largest_cluster(), 3); }
    #[test] fn test_signed() { let mut lat = TernaryLattice::new(4); lat.add_edge(0,1,1); lat.signs[0]=1; let (p,n)=lat.signed_clusters(); assert!(p>=0&&n>=0); }
    #[test] fn test_empty() { let lat = TernaryLattice::new(0); assert_eq!(lat.n, 0); }
    #[test] fn test_single() { let lat = TernaryLattice::new(1); assert_eq!(lat.count_components(), 1); }
    #[test] fn test_components_separate() { let lat = TernaryLattice::new(5); assert_eq!(lat.count_components(), 5); }
    #[test] fn test_components_joined() { let mut lat = TernaryLattice::new(5); lat.add_edge(0,1,1); assert_eq!(lat.count_components(), 4); }
    #[test] fn test_chain() { let mut lat = TernaryLattice::new(100); for i in 0..99 { lat.add_edge(i,i+1,1); } assert!(lat.percolates()); }
    #[test] fn test_ring() { let mut lat = TernaryLattice::new(10); for i in 0..10 { lat.add_edge(i,(i+1)%10,1); } assert!(lat.percolates()); }
    #[test] fn test_disconnected() { let mut lat = TernaryLattice::new(10); lat.add_edge(0,1,1); lat.add_edge(2,3,1); assert_eq!(lat.largest_cluster(), 2); }
    #[test] fn test_density_zero() { let lat = TernaryLattice::new(5); assert_eq!(lat.critical_density(), 0.0); }
}
