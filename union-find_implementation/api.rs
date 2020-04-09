// Quick Find implementation
pub struct QuickFindUF {
    // Storage for the ids
    id: Vec<usize>,
}
impl QuickFindUF {
    // Constructor
    pub fn new(N: usize) -> Self {
        // Generate all numbers from 0 up until N and put it in id
        let id: Vec<usize> = (0..N).collect();
        Self { id }
    }
    // Check if two nodes are connected
    pub fn connected(&self, p: usize, q: usize) -> bool {
        return self.id[p] == self.id[q];
    }
    // Connect two nodes
    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.id[p];
        let qid = self.id[q];
        // Replace all pids with qids
        for id in self.id.iter_mut() {
            if *id == pid {
                *id = qid;
            }
        }
    }
}

// Quick Union implementation
pub struct QuickUnionUF {
    // Storage for the ids
    id: Vec<usize>,
    // Storage for the sizes of the trees
    sz: Vec<usize>,
}
impl QuickUnionUF {
    // Constructor
    pub fn new(N: usize) -> Self {
        // Generate all numbers from 0 up until N and put it in id
        let id: Vec<usize> = (0..N).collect();
        Self { id, sz: vec![1; N] }
    }
    // Trace a node back to it's root (or parent), and return the root
    fn root(&mut self, mut i: usize) -> usize {
        while i != self.id[i] {
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }
        return i;
    }
    // Check if two nodes are connected
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        return self.root(p) == self.root(q);
    }
    // Connect two nodes
    pub fn union(&mut self, p: usize, q: usize) {
        // Get the parents of the two nodes
        let i = self.root(p);
        let j = self.root(q);
        // If they are already in the same tree, then we don't have to do anything
        if i == j {
            return;
        }
        // Weighted union
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}