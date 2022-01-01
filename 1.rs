// structs are basically named tuples, next to enum the only 'custom types'

struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 28;
    let peter = Person { name, age };

    println!("person is named {:?} and is {:?} year so age", peter.name, peter.age);
}
