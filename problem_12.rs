fn count_divs(n: u32) -> u32 {
    let mut cnt = 0;
    let limit = (n as f32).sqrt() as u32 + 1;
    for i in 1..limit {
        if n % i == 0 {
            if n / i == i {
                cnt += 1;
            } else {
                cnt = cnt + 2;
            }
        }
    }
    cnt
}

let num = (2..).map(|n| {
    let t = (1..n).sum();
    let cnt = count_divs(t);
    (t, cnt)
}).filter(|&(_, cnt)| cnt > 500).nth(0);

println!("{:?}", num);
