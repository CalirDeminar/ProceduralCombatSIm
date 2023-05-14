pub mod names;
pub mod mind {
    use rand::Rng;

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

    #[derive(PartialEq, Debug, Clone)]
    pub struct Relation {
        relation_type: RelationVerb,
        entity: Mind
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Mind {
        first_name: String,
        last_name: String,
        gender: Gender,
        relations: Vec<Relation>
    }

    fn random_char(name_dict: &NameDictionary) -> Mind {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen();
        let mut gender = Gender::Ambiguous;
        if roll > 0.6 {
            gender = Gender::Male;
        }
        if roll > 0.2 {
            gender = Gender::Female;
        }
        let (first_name, last_name) = random_name(name_dict, &gender);
        return Mind {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            gender,
            relations: vec![]
        }
    }

    fn generate_base_population(i: usize, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut output: Vec<Mind> = vec![];
        for _i in 0..i {
            output.push(random_char(&name_dict));
        }
        return output;
    }

    pub fn generate_population() -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let name_dict = gen_name_dict();
        let mut population = generate_base_population(100, &name_dict);
        for (i, mind) in population.iter().enumerate() {
            let has_partner = rng.gen::<f32>() > 0.5;
            if !has_partner {
                continue;
            }
            let partner_type_roll = rng.gen::<f32>();
            let mut gender = Gender::Ambiguous;
            if partner_type_roll > 0.8 {
                gender = if mind.gender.eq(&Gender::Male) {Gender::Female} else {Gender::Male};
            } else {
                gender = mind.gender.clone();
            }
            let (f_name, l_name) = random_name(&name_dict, &gender);

            let relation = Mind { 
                first_name: String::from(f_name), 
                last_name: String::from(l_name), 
                gender, 
                relations: vec![] // to add relation 
            };
        }
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
        println!("{:#?}", population);
    }
}