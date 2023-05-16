pub mod institutions;
pub mod relations;
pub mod population {
    use std::{fs::File, io::Write};
    use crate::data::names::names::*; 
    use crate::creature::mind::mind::*;  
    use crate::population::institutions::institutions::*;
    use crate::population::relations::relations::*;

    pub struct Population {
        pub citizens: Vec<Mind>,
        pub institutions: Vec<Institution>
    }

    fn print_population(population: &Vec<Mind>) -> String {
        let mut output = String::from("");
        for mind in population {
            output.push_str(&print_mind(&mind, &population));
        }
        return output;
    }

    fn generate_base_population<'a>(i: usize, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut output: Vec<Mind> = vec![];
        for _i in 0..i {
            output.push(random_char(&name_dict));
        }
        return output;
    }

    pub fn generate_population(size: usize) -> Vec<Mind> {
        let name_dict = gen_name_dict();
        let mut population = generate_base_population(size, &name_dict);
        population = add_partners_to_population(population, &name_dict);
        population = add_parents_to_population(population, &name_dict);
        population = link_friends_within_population(population);
        return population
    }

    pub fn output_population(size: usize) {
        let pop = generate_population(size);
        // let output = format!("{:#?}", pop);
        let mut file = File::create("./export.txt").unwrap();
        let pop_log = print_population(&pop);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }
}