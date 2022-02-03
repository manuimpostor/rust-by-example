// Lessons 1-7 from Rust by Example >
// https://doc.rust-lang.org/stable/rust-by-example/expression.html
// structs are basically named tuples, next to enum the only 'custom types'

struct Person {
    name: String,
    age: u8,
}

// lists are tuples with a pointer to next element

enum List {
    Pair(u32, Box<List>),
    Nil,
}

//add method to this new List enum
impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Pair(elem, Box::new(self))
    }

    fn stringify(&self) -> String {
        match *self {
            List::Pair(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
            format!("Nil")
            },
        }
    }
}

fn main() {

    // explicit type conversion (casting) can be performed using the as keyword. No implicit type
    // conversion is done. Inference is.
    let decimal = 128.1_f32;
    let num = decimal as u8;
    let texty_num = num as char; // limits to casting here

    println!("casting in action {} -> {} -> {}", decimal, num, texty_num);

    // converting is done by using std::convert::From and Into. Can be implemented on own types
    let name = String::from("Peter");
    let age = 28;
    let peter = Person { name, age };

    println!("person is named {:?} and is {:?} year so age", peter.name, peter.age);

    let mut meag_list = List::new();
    // prepend some elements
    meag_list = meag_list.prepend(3);
    meag_list = meag_list.prepend(2);
    meag_list = meag_list.prepend(1);

    println!("linked list looks like this {}", meag_list.stringify());
}
