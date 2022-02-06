mod collections;
fn main() {

    println!("welcome");
    collections::collections::run_print();
    let orange = collections::collections::city_contained("Orange".to_string());
    let disney = collections::collections::city_contained("Disney".to_string());
    if orange {
        println!("We deliver to Orange");
    }
    if disney {
        println!("We deliver to Disney")
    }
}
