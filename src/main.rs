fn get_fib_sequence(nth_number: u32) {
    let mut fib_sequence: Vec<u128> = vec![0, 1];

    for i in 0..(nth_number - 1) {
        let fib_1 = fib_sequence[i as usize];
        let fib_2 = fib_sequence[(i + 1) as usize];

        let fib_3 = fib_1 + fib_2;
        fib_sequence.push(fib_3);
    }

    println!("{:?}", fib_sequence);
}

fn main() {
    get_fib_sequence(185);
}
