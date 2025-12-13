fn main() {
    let range = [11..=22,95..=115, 998..=1012];
    let mut sum =0;
    for i in range { 
        for j in i {
            if is_invalid(j) {
               sum+=j;
            }
        }
    }
    println!("{sum}");
    for i in 3..=22 {
        print!("{i} ");
    }
}


fn is_invalid(n: i64) -> bool {
    let num_digits = n.checked_ilog10().map_or(1, |x| x+1);
    for i in 1..=num_digits/2 {
        let mut n =n;
        let mut collector:Vec<i64> = Vec::new();
        while n >0 {
            collector.push(n%10_i64.pow(i));
            n/=10_i64.pow(i); 
        }
        if collector.windows(2).all(|w| w[0]==w[1]) {
            return true
        }
    }
    false
}

//Guesses:
//25914039121 - too high
