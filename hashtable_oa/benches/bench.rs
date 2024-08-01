#![feature(test)]
extern crate test;

use test::Bencher;

use hashtable::HashTable;

fn benchmark() {
    let mut hash = HashTable::<usize, usize>::new();

    for _ in 0..1000 {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
    // hash.debug_dump();
}

#[bench]
fn benchmark_test(b: &mut Bencher) {
    b.iter(|| benchmark());
}
