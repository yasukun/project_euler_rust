fn isprime(n: u32) -> bool {
    let r = match n {
        x if x < 2 => false, 
        x if x == 2 => true,
        x if x % 2 == 0 => false,
        _ => {
            let limit = (n as f32).sqrt() as u32 + 1;
            (3..limit).all(|x| n % x != 0)
        } 
    };
    r
}

println!("{:?}",
         (2..).filter(|n| isprime(*n)).take_while(|x| *x < 2000000).sum::<u32>());
