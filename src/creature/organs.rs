pub mod organs {
    use crate::creature::body::body::{StatusTag, LocationTag};

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum OrganFunction {
        // Core
        Breath,
        Thought,
        Nervous,
        Circulation,
        Structural,
        Artery,
        // Muscle
        Motor,
        // Sense
        Sight,
        Hearing,
        Smell,
        // Other
        Retch,
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct Organ {
        pub name: String,
        pub functions: Vec<OrganFunction>,
        pub locations: Vec<LocationTag>,
        pub conditions: Vec<StatusTag>,
        pub size: u32,
        // pub parent: &'a BodyPart,
    }
    fn gen_organ_set(name: String, functions: Vec<OrganFunction>, count: u32, size: u32) -> Vec<Organ> {
        let mut output: Vec<Organ> = vec![];
        for _i in 0..count {
            output.push(Organ {
                name: name.clone(),
                functions: functions.clone(),
                conditions: vec![],
                locations: vec![],
                size
            })
        }
        if count == 2 {
            output[0].locations.push(LocationTag::Left);
            output[0].name = format!("{} {}", "Left", name.clone());
            output[1].locations.push(LocationTag::Right);
            output[1].name = format!("{} {}", "Right", name.clone());
        }
        output
    }
    pub fn hearts(count: u32, parent_size: u32) -> Vec<Organ> {
        gen_organ_set(
            String::from("Heart"), 
            vec![OrganFunction::Circulation], 
            count, 
            parent_size / 20
        )
    }
    pub fn lungs(count: u32, parent_size: u32) -> Vec<Organ> {
        gen_organ_set(
            String::from("Lung"), 
            vec![OrganFunction::Breath], 
            count, 
            parent_size / 10
        )
    }
    pub fn spine(parent_size: u32) -> Vec<Organ> {
        vec![Organ {
            name: String::from("Spine"),
            conditions: vec![],
            functions: vec![ OrganFunction::Nervous, OrganFunction::Structural],
            size: parent_size / 12,
            locations: vec![]
        }]
    }
    pub fn brain(parent_size: u32) -> Vec<Organ> {
        vec![Organ {
            name: String::from("Brain"),
            conditions: vec![],
            functions: vec![ OrganFunction::Thought, OrganFunction::Nervous],
            size: parent_size / 2,
            locations: vec![]
        }]
    }
    pub fn eyes(count: u32, parent_size: u32) -> Vec<Organ> {
        gen_organ_set(
            String::from("Eye"), 
            vec![OrganFunction::Sight], 
            count, 
            parent_size / 60
        )
    }
    pub fn ears(count: u32, parent_size: u32) -> Vec<Organ> {
        gen_organ_set(
            String::from("Ear"), 
            vec![OrganFunction::Hearing], 
            count, 
            parent_size / 60
        )
    }
    pub fn nose(parent_size: u32) -> Vec<Organ> {
        vec![Organ {
            name: String::from("Nose"),
            conditions: vec![],
            functions: vec![OrganFunction::Smell],
            size: parent_size / 60,
            locations: vec![]
        }]
    }
    pub fn bone(parent_size: u32, parent_name: String) -> Vec<Organ> {
        vec![Organ {
            name: format!("{} Bone", parent_name),
            conditions: vec![],
            functions: vec![OrganFunction::Structural],
            size: parent_size / 4,
            locations: vec![]
        }]
    }
    pub fn skull(parent_size: u32) -> Vec<Organ> {
        vec![Organ {
            name: String::from("Skull"),
            conditions: vec![],
            functions: vec![OrganFunction::Structural],
            size: (parent_size as f32 / 0.9) as u32,
            locations: vec![]
        }]
    }
}