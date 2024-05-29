use std::collections::HashMap;

fn main() {
    let mut cache: HashMap<usize, u128> = HashMap::new();
    for i in 0..186 {
        println!("{}: {}", i, fib_db(i, &mut cache));
    }
    for i in 2..186 {
        print!("{}, ", cache.get(&i).unwrap());
    }
}

fn fib_db(n: usize, cache: &mut HashMap<usize, u128>) -> u128 {
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let result = fib_db(n - 1, cache) + fib_db(n - 2, cache);
    cache.insert(n, result);
    return result;
}
