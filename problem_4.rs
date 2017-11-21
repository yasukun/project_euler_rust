let threedigits = (100..1000).collect::<Vec<u32>>();

fn inversion(num: u32) -> u32 {
    num.to_string().chars().rev().collect::<String>().parse::<u32>().unwrap()
}

let mut maxnum =  0;

for i in threedigits.iter() {
    for j in threedigits.iter() {
        let product = i * j;
        if inversion(product) == product {
            if maxnum < product {
                maxnum = product;
            }
        }
    }
}

println!("{}", maxnum);
