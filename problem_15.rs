extern crate num;

use num::bigint::{BigInt, ToBigInt};

// 40!
//-----
//20!20!

let n = (1..41)
             .rev()
             .map(|num| num.to_bigint().unwrap())
             .fold(1.to_bigint().unwrap(), |sum, i| {
                 sum.checked_mul(&i)
                     .unwrap()
             });

let k =  (1..21).rev().map(|num| num.to_bigint().unwrap()).fold(1.to_bigint().unwrap(), |sum, i| sum.checked_mul(&i).unwrap());
let k2 = k.checked_mul(&k).unwrap();
println!("{:?}", n.checked_div(&k2).unwrap().to_string());
