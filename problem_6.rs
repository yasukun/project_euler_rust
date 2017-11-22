let sum_of_squares =  (1..101).map(|n| n * n).sum::<u64>();
let square_of_sums = (1..101).sum::<u64>().pow(2);
println!("{}", square_of_sums - sum_of_squares);
