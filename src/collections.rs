// vector is a resizable array
// hashmap, colleciton of key-value pairs with unique key -> lookup table

pub mod collections {
    // hashset set of unique vals of type T
    // Task: create hashset with name of cities from csv to do fast lookup
    //use std::collections::hash_set;
    use std::error::Error;
    extern crate csv;
    use csv::Reader;
    use std::process;

    // TODO: import city names into a hashset
    // next: filter by city row, put that in hashset, call from main with few examples, renume fn
    fn read_cities() -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path("./src/uspop.csv")?;
        let headers = rdr.headers()?;
        println!("{:?}", headers);

        for result in rdr.records() {
            let record = result?;

            let mut i = 0;
            for r in record.into_iter() {
                if i % 5 == 0 {println!("{:?}", r);}
                i = i + 1;
            }
        }
        Ok(())
    }
    pub fn run_print()  {
        if let Err(err) = read_cities(){
            println!("error running read_cities(): {}", err);
            process::exit(1);
        }
    }

    //let mut _cities_we_deliver_to = HashSet::new("wow");[]
}