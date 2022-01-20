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
            0 => { ; }
            21 => { println!("21 is the number we were looking for");}
            _ => { println!("not what we are looking for: {}", x); }
        }
    }
}
