// Flow Control
// if-else: all conditions are branches and must return same type

fn is_tall(h:i32){
    if h > 190 {
        println!("person is tall");
    } else if h == 0 {
        println!("person is nonexistent");
    } else {
        println!("person is not tall");
    }
}

fn main() {

    let h = 0;
    let txt = String::from(is_tall(h))
    println!(txt);
}
