mod api;

fn main() {
    let mut uf = api::QuickUnionUF::new(10);
    uf.union(0, 5);
    uf.union(1, 6);
    uf.union(2, 7);
    uf.union(3, 8);
    uf.union(4, 9);
    uf.union(5, 6);
    uf.union(1, 2);
    uf.union(3, 4);
    println!("{}", uf.connected(0, 7)); //true
    println!("{}", uf.connected(1, 3)); //false
    println!("{}", uf.connected(8, 9)); //true
}