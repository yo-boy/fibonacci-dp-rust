const NUM: usize = 1000;

fn main() {
    let mut found: [bool; NUM] = [false; NUM];
    let mut val: [usize; NUM] = [0; NUM];
    for i in 0..40 {
        println!("{}: {}", i, fib_db(i, found, val));
    }
}

fn fib_db(n: usize, mut found: [bool; NUM], mut val: [usize; NUM]) -> usize {
    if found[n] {
        return val[n];
    }

    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    found[n] = true;
    val[n] = fib_db(n - 1, found, val) + fib_db(n - 2, found, val);
    return val[n];
}
