pub mod data {
    use std::fs::File;

    use regex::Regex;

    use crate::creature::mind::{names::names::NameDefinition, mind::Gender};

    fn write_data(data: &Vec<NameDefinition>, filename: String) {
        let mut wtr = csv::Writer::from_writer(File::create(filename).unwrap());
        wtr.write_record(&["Name", "Gender"]).unwrap();
        println!("Writing: ");
        let mut i = 0;
        for r in data {
            println!("Writing: {}", i);
            i+=1;
            let mut gender = "";
            if r.gender.eq(&Gender::Male) {
                gender = "m";
            }
            if r.gender.eq(&Gender::Female) {
                gender = "f";
            }
            wtr.write_record(&[&r.name, &String::from(gender)]).unwrap();
            wtr.flush().unwrap();
        }
    }

    pub fn build_dataset() {
        let mut first_names: Vec<NameDefinition> = vec![];
        let mut last_names: Vec<NameDefinition> = vec![];
        let file = File::open("./data/GB.csv").expect("Cannot Open DataFile");
        let mut csv_reader = csv::ReaderBuilder::new().from_reader(file);
        println!("Reading File");
        for (i, l) in csv_reader.records().enumerate() {
            println!("Reading: {}", i);
            let row = l.unwrap();
            let first_name = row.get(0).unwrap();
            let last_name = row.get(1).unwrap();
            let gender_str = row.get(2).unwrap().trim_start().to_lowercase();
            let mut gender = Gender::Ambiguous;
            if gender_str.eq("m") {
                gender = Gender::Male;
            }
            if gender_str.eq("f") {
                gender = Gender::Female;
            }
            let valid_name_regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
            if valid_name_regex.is_match(first_name) {
                let exists = first_names.iter().any(|i| i.name.eq(first_name));
                if !exists {
                    first_names.push(NameDefinition { name: String::from(first_name), gender: gender.clone() });
                }
            }
            if valid_name_regex.is_match(last_name) {
                let exists = last_names.iter().any(|i| i.name.eq(last_name));
                if !exists {
                    last_names.push(NameDefinition { name: String::from(last_name), gender: gender.clone() });
                }
            }
        }
        write_data(&first_names, String::from("./src/creature/mind/first_names.csv"));
        write_data(&last_names, String::from("./src/creature/mind/last_names.csv"));
    }

    #[test]
    fn gen_data() {
        build_dataset();
    }
}