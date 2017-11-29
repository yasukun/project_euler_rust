fn count_collatz(num: u64) -> u64 {
    let mut v = num;
    let mut count = 1;
    loop {
        if v < 2 {
            break;
        }
        match v {
            n if n % 2 == 0 => v = n / 2, 
            n if n % 2 != 0 => v = 3 * n + 1,
            _ => break,
        }
        count += 1;
        //println!("{}", v);
    }
    count
}

println!("count: {:?}",
         (13..1000000).map(|v| (v, count_collatz(v))).max_by(|a, b| a.1.cmp(&b.1)));
