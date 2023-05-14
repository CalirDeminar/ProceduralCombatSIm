pub mod names;
pub mod mind {
    use rand::Rng;
    use uuid::Uuid;
    use std::cell::RefCell;

    use super::names::names::{NameDictionary, random_name, gen_name_dict};
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

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        id: Uuid,
        first_name: String,
        last_name: String,
        gender: Gender,
        relations: Vec<(RelationVerb, Uuid)>
    }

    fn get_name_from_id(id: &Uuid, population: &Vec<Mind>) -> String {
        let result = population.iter().find(|m| m.id.eq(id)).unwrap();
        return format!("{} {}", String::from(&result.first_name), String::from(&result.last_name));
    }
    fn print_mind(mind: &Mind, population: &Vec<Mind>) {
        println!("==========");
        let relations: Vec<(&RelationVerb, String)> = mind.relations.iter().map(|(verb, id)| (verb, get_name_from_id(&id, &population))).collect();
        println!("Name: {} {}", mind.first_name, mind.last_name);
        println!("Gender: {:?}", mind.gender);
        println!("Relations: ");
        if relations.len() < 1 {
            println!("  None");
        } else {
            for (verb, name) in relations {
                println!("  {:?}: {}", verb, name);
            }
        }
        println!("==========");
    }
    fn print_population(population: &Vec<Mind>) {
        for mind in population {
            print_mind(&mind, &population);
        }
    }

    fn random_char<'a>(name_dict: &NameDictionary) -> Mind {
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
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender,
            relations: Vec::new()
        }
    }

    fn generate_base_population<'a>(i: usize, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut output: Vec<Mind> = vec![];
        for _i in 0..i {
            output.push(random_char(&name_dict));
        }
        return output;
    }

    fn gen_partner_gender(input_gender: &Gender) -> Gender {
        let mut rng = rand::thread_rng();
        let partner_type_roll = rng.gen::<f32>();
        let parnet_gender;
        if partner_type_roll > 0.8 {
            parnet_gender = if input_gender.eq(&Gender::Male) {Gender::Female} else {Gender::Male};
        } else {
            parnet_gender = input_gender.clone();
        }
        return parnet_gender;
    }

    fn add_partners_to_population(population: Vec<Mind>, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Mind> = Vec::new();
        for mind in population {
            let has_partner = rng.gen::<f32>() > 0.5;
            if !has_partner {
                output.push(mind.clone());
                continue;
            } else {
                let partner_gender = gen_partner_gender(&mind.gender);
                let (first_name, last_name) = random_name(&name_dict, &partner_gender);
                let mut target = mind.clone();
                let relation = Mind {
                    id: Uuid::new_v4(),
                    first_name,
                    last_name,
                    gender: partner_gender,
                    relations: vec![(RelationVerb::Partner, target.id.clone())]
                };
                target.relations.push((RelationVerb::Partner, relation.id.clone()));
                output.push(target);
                output.push(relation);

                output.push(mind.clone());
            }
        }
        return output;
    }

    pub fn generate_population() -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let name_dict = gen_name_dict();
        let mut population = generate_base_population(100, &name_dict);
        population = add_partners_to_population(population, &name_dict);
        return population
    }

    // relation generation

    // generate core population
    // add generated parents to the population
    //   for each entity, roll to see if their parents will be in the population
    //  add spousal + partner relationships
    //  roll and generate children and add to population

    #[test]
    fn generate_population_test() {
        let population = generate_population();
        print_population(&population);
    }
}