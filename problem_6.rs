let nums = (1..101).collect::<Vec<u32>>();
let sum_of_squares =  nums.iter().map(|n| n * n).fold(0, |sum, i| sum + i);
let square_of_sums = nums.iter().fold(0, |sum, i| sum + i).pow(2);
println!("{}", square_of_sums - sum_of_squares);
