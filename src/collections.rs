// vector is a resizable array
// hashmap, colleciton of key-value pairs with unique key -> lookup table

pub mod collections {
    // hashset set of unique vals of type T
    // Task: create hashset with name of cities from csv to do fast lookup
    use std::collections::HashSet;
    use std::error::Error;
    extern crate csv;
    use csv::Reader;
    use std::process;

    // done: import city names into a hashset
    // done: filter by city row, put that in hashset
    fn read_cities() -> Result<HashSet<String>, Box<dyn Error>> {
        let mut rdr = Reader::from_path("./src/uspop.csv")?;
        let headers = rdr.headers()?;
        println!("{:?}", headers);

        let mut cities_we_deliver_to:HashSet<String> = HashSet::new();

        for result in rdr.records() {
            let record = result?;

            let mut i = 0;
            for r in record.into_iter() {
                if i % 5 == 0 {
                    cities_we_deliver_to.insert(r.to_string());
                }
                i = i + 1;
            }
        }

        Ok(cities_we_deliver_to)
    }

    // todo: call from main with few examples, allow quering for city in hash
    // fn city_contained

    fn hash_cities() -> HashSet<String> {
        if let Err(err) = read_cities(){
            println!("error running read_cities(): {}", err);
            process::exit(1);
        } 
        if let Ok(res) = read_cities(){
            println!("we have just the hashset");
            return res;
        }

        println!("{:?}", read_cities());
        // get HashSet as return
        return HashSet::new();
    }

    pub fn run_print() {
        let h = hash_cities();
        println!{"{:?}", h};
    }

    pub fn city_contained(c: String) -> bool {
        // check in hashset
        let h = hash_cities();
        if h.contains(&c) { return true; }
        return false;
    }
}