pub mod organs {
    use crate::creature::{body::body::{BodyPartTag, BodyPart}};
    fn gen_organ_set(name: String, tags: Vec<BodyPartTag>, count: u32, size: u32) -> Vec<BodyPart> {
        let mut output: Vec<BodyPart> = vec![];
        for _i in 0..count {
            output.push(BodyPart {
                name: name.clone(),
                tags: tags.clone(),
                statuses: vec![],
                internal: vec![],
                children: vec![],
                size
            })
        }
        if count == 2 {
            output[0].tags.push(BodyPartTag::Left);
            output[0].name = format!("{} {}", "Left", name.clone());
            output[1].tags.push(BodyPartTag::Right);
            output[1].name = format!("{} {}", "Right", name.clone());
        }
        return output;
    }
    pub fn hearts(count: u32, parent_size: u32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Heart"), 
            vec![BodyPartTag::Circulation], 
            count, 
            parent_size / 20
        );
    }
    pub fn lungs(count: u32, parent_size: u32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Lung"), 
            vec![BodyPartTag::Breath], 
            count, 
            parent_size / 10
        );
    }
    pub fn spine(parent_size: u32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Spine"),
            statuses: vec![],
            tags: vec![ BodyPartTag::Nervous],
            internal: vec![],
            children: vec![],
            size: parent_size / 12
        }]
    }
    pub fn brain(parent_size: u32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Brain"),
            statuses: vec![],
            tags: vec![ BodyPartTag::Thought],
            internal: vec![],
            children: vec![],
            size: parent_size / 2
        }]
    }
    pub fn eyes(count: u32, parent_size: u32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Eye"), 
            vec![BodyPartTag::Sight], 
            count, 
            parent_size / 60
        );
    }
    pub fn ears(count: u32, parent_size: u32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Ear"), 
            vec![BodyPartTag::Hearing], 
            count, 
            parent_size / 60
        );
    }
    pub fn nose(parent_size: u32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Nose"),
            statuses: vec![],
            tags: vec![BodyPartTag::Smell],
            internal: vec![],
            children: vec![],
            size: parent_size / 60
        }]
    }
}