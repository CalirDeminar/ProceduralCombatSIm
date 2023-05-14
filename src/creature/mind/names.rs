pub mod data;
pub mod names {
    use std::fs::File;
    use rand::Rng;

    use crate::creature::mind::mind::Gender;

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDefinition {
        pub name: String,
        pub gender: Gender
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct NameDictionary {
        first_names: Vec<NameDefinition>,
        last_names: Vec<NameDefinition>,
    }

    pub fn gen_name_dict() -> NameDictionary {
        return NameDictionary {
            first_names: parse_file(String::from("./src/creature/mind/first_names.csv")),
            last_names: parse_file(String::from("./src/creature/mind/last_names.csv")),
        }
    }

    fn random_name_for_gender<'a>(input: &'a Vec<NameDefinition>, gender: &Gender) -> &'a str {
        let mut working: Vec<&'a NameDefinition> = vec![];
        for name in input {
            if name.gender.eq(&Gender::Ambiguous) {
                working.push(name);
            }
            if name.gender.eq(gender) {
                working.push(name);
            }
        }
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let result = working[(roll * working.len() as f32) as usize];
        return &result.name;
    }

    pub fn random_name<'a>(dict: &'a NameDictionary, gender: &Gender) -> (&'a str, &'a str) {
        return (random_name_for_gender(&dict.first_names, &gender), random_name_for_gender(&dict.last_names, &gender));
    }

    fn parse_file(filename: String) -> Vec<NameDefinition> {
        let mut output: Vec<NameDefinition> = vec![];
        let file = File::open(&filename).expect(&format!("Cannot open: {}", &filename));
        let mut csv_reader = csv::ReaderBuilder::new().from_reader(file);
        for l in csv_reader.records() {
            let line = l.unwrap();
            let mut gender = Gender::Ambiguous;
            let gender_str = line.get(1).unwrap().trim_start().to_lowercase();
            if gender_str.eq("male") {
                gender = Gender::Male;
            }
            if gender_str.eq("female") {
                gender = Gender::Female;
            }
            output.push(NameDefinition{
                name: String::from(line.get(0).unwrap().trim_start()),
                gender
            });
        }
        return output;
    }



    #[test]
    fn random_name_test() {
        let dict = gen_name_dict();
        for _i in 0..10 {
            println!("{:?}", random_name(&dict, &Gender::Male));
        }
    }
}