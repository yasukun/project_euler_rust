// cheating

let target = 1000;

for a in 3..target {
    for b in a..target {
        let c = target - a - b;
        let pythagora = a *a + b * b == c *c;
        if pythagora {
            println!("{}", a * b * c);
        }
    }
}
