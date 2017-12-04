extern crate num;

use num::bigint::ToBigInt;
use std::iter;

let target = iter::repeat(2).take(1000).map(|c| c.to_bigint().unwrap()).fold(1.to_bigint().unwrap(), |sum, i| sum.checked_mul(&i).unwrap());
println!("{:?}", target.to_string().chars().map(|c| c.to_string().parse::<u32>().unwrap()).sum::<u32>());
