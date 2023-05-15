pub mod institutions {

    use crate::creature::mind::{names::names::{NameDictionary, random_name, gen_name_dict}, mind::Gender};

    #[derive(PartialEq, Debug, Clone)]
    pub enum InstituteType {
        // Public Infra
        PowerStation,
        WaterTreatmentWorks,
        SewageWorks,
        Library,
        School,
        University,
        Court,
        CityHall,
        Prison,
        PoliceStation,
        Hospital,
        // Corporate Infra
        FoodService, // Restarants, Bars, Pubs
        GeneralRetail, // Most "general" shops, cornerShops, supermarkets, etc
        SpecialistRetail, // Specialist Retailers, jewelers, tailors, mechanics
        EntertainmentVenue, // Thearters, cinemas, nightclubs
        IndustrialManufacturers, // Goods manufacturers
        SpecialistService, // "Office" businesses
        Publishers
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct Institution {
        name: String,
        public: bool,
        institute_type: InstituteType
    }

    const PUBLIC_INSTITUTES: [InstituteType; 11] = [
        InstituteType::PowerStation,
        InstituteType::WaterTreatmentWorks,
        InstituteType::SewageWorks,
        InstituteType::Library,
        InstituteType::School,
        InstituteType::University,
        InstituteType::Court,
        InstituteType::CityHall,
        InstituteType::Prison,
        InstituteType::PoliceStation,
        InstituteType::Hospital
    ];

    fn label_insitute_type(i: &InstituteType) -> String {
        return String::from(match i {
            InstituteType::PowerStation => "Power Station",
            InstituteType::WaterTreatmentWorks => "Water Treatment Works",
            InstituteType::SewageWorks => "Sewage Works",
            InstituteType::CityHall => "City Hall",
            InstituteType::PoliceStation => "Police Station",
            _ => { let r = format!("{:?}", i); return r},
        });
    }

    pub fn generate_population_institutions(name_dict: &NameDictionary) -> Vec<Institution>{
        let mut output: Vec<Institution> = Vec::new();
        for i in PUBLIC_INSTITUTES {
            let (_, prefix) = random_name(&name_dict, &Gender::Ambiguous);
            output.push(Institution { 
                name: format!("{} {}", prefix, label_insitute_type(&i)), 
                public: true, 
                institute_type: i
             });
        }
        return output;
    }

    #[test]
    fn generate_population_institutions_test() {
        let name_dict = gen_name_dict();
        println!("{:#?}", generate_population_institutions(&name_dict));
    }
}