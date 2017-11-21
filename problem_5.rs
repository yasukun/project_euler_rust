let nums = (1..21).collect::<Vec<u32>>();
let mut minnum = 1;

loop {
    if nums.iter().all(|v| minnum % v == 0) {
        println!("{}", minnum);
        break
    }
    minnum += 1
}
