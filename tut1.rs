// Working through https://aml3.github.io/RustTutorial/html/01.html and following

fn is_odd(n: u32) -> bool {
    return n % 2 == 1;
}


fn main() {
    // match instead of switch statements, powerful!
    // branches need to be extensive,so need to catch all cases
    // use '_' for a 'match-all'
    match is_odd(2) {
        true => println!("odd"),
        false => println!("even")
    }

    for x in 0..25 {
        match x {
            x if x < 21 => { println!("lil low"); }
            21 => { println!("21 is the number we were looking for");}
            _ => { println!("not what we are looking for: {}", x); }
        }
    }

    let combo = (21, true);
    match combo {
        (num, boo) if { num >= 20 && num <= 26  && boo} => { println!("true in range"); } //true in range
        (num, boo) if { num < 20 || num > 26 && boo } => { println!("true out of range"); }// true out of range
        (num, _) if { num >= 20 && num <= 26 } => { println!("range no bool"); } // range no bool
        _ => { println!("no condition met"); }// no condition met
    }
}
