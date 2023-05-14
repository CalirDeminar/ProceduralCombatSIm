pub mod names;
pub mod relations;
pub mod mind {
    use std::{fs::File, io::Write};

    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use uuid::Uuid;

    use super::{names::names::{NameDictionary, random_name, gen_name_dict}, relations::relations::{add_partners_to_population, add_parents_to_population, link_friends_within_population}};
    #[derive(PartialEq, Debug, Clone)]
    pub enum RelationVerb {
        // family
        Parent,
        Child,
        Partner,
        ExPartner,
        Spouse,
        ExSpouse,
        // business
        Employer,
        Employee,
        Coleague,
        // social
        Acquaintance,
        Friend,
        CloseFriend,
        Grudge,
        // religion
        Diety,
        Priest,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum Gender {
        Male,
        Female,
        Ambiguous
    }

    // #[derive(PartialEq, Debug, Clone)]
    // pub struct Relation<'a> {
    //     relation_type: RelationVerb,
    //     entity: &'a Mind<'a>
    // }

    pub type Relation = (RelationVerb, Uuid);

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        pub id: Uuid,
        pub first_name: String,
        pub last_name: String,
        pub gender: Gender,
        pub age: u32,
        pub relations: Vec<Relation>
    }

    pub fn get_name_from_id(id: &Uuid, population: &Vec<Mind>) -> String {
        let result = population.iter().find(|m| m.id.eq(id));
        if result.is_some() {
            return format!("{} {}", String::from(&result.unwrap().first_name), String::from(&result.unwrap().last_name));
        }
        return format!("Missing ID: {}", id);
    }
    fn print_mind(mind: &Mind, population: &Vec<Mind>) -> String {
        let mut output = String::from("");
        output.push_str("===========\n");
        let relations: Vec<(&RelationVerb, String)> = mind.relations.iter().map(|(verb, id)| (verb, get_name_from_id(&id, &population))).collect();
        // println!("ID: {}", mind.id);
        output.push_str(&format!("Name: {} {}\n", mind.first_name, mind.last_name));
        output.push_str(&format!("Gender: {:?}\n", mind.gender));
        output.push_str(&format!("Age: {}\n", mind.age));
        output.push_str(&format!("Relations:\n"));
        if relations.len() < 1 {
            println!("  None");
            output.push_str(&format!("  None\n"));
        } else {
            for (verb, name) in relations {
                output.push_str(&format!("  {:?}: {}\n", verb, name));
            }
        }
        output.push_str(&format!("==========\n"));
        return output;
    }
    fn print_population(population: &Vec<Mind>) -> String {
        let mut output = String::from("");
        for mind in population {
            output.push_str(&print_mind(&mind, &population));
        }
        return output;
    }

    pub fn random_char<'a>(name_dict: &NameDictionary) -> Mind {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let mut gender = Gender::Ambiguous;
        if roll > 0.6 {
            gender = Gender::Male;
        }
        if roll > 0.2 {
            gender = Gender::Female;
        }
        let (first_name, last_name) = random_name(&name_dict, &gender);
        let distribution = Normal::new(5.0, 10.0).unwrap();
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender,
            relations: Vec::new(),
            age: (rng.gen::<f32>() * 40.0) as u32 + 15 + distribution.sample(&mut rand::thread_rng()) as u32
        }
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
        let mut file = File::create("./export.json").unwrap();
        let pop_log = print_population(&pop);
        file.write_all(pop_log.into_bytes().as_slice()).unwrap();
    }

    // relation generation

    // generate core population
    // add generated parents to the population
    //   for each entity, roll to see if their parents will be in the population
    //  add spousal + partner relationships
    //  roll and generate children and add to population

    #[test]
    fn generate_population_test() {
        // let population = generate_population(50);
        output_population(50);
    }
}