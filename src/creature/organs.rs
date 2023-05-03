pub mod organs {
    use crate::creature::{body::body::{BodyPartTag, BodyPart}};
    fn gen_organ_set(name: String, tags: Vec<BodyPartTag>, count: i32, size: i32) -> Vec<BodyPart> {
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
            output[0].tags.push(BodyPartTag::left);
            output[1].tags.push(BodyPartTag::right);
        }
        return output;
    }
    pub fn hearts(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Heart"), 
            vec![BodyPartTag::circulation], 
            count, 
            parent_size / 20
        );
    }
    pub fn lungs(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Lung"), 
            vec![BodyPartTag::breath], 
            count, 
            parent_size / 10
        );
    }
    pub fn spine(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Spine"),
            statuses: vec![],
            tags: vec![ BodyPartTag::nervous],
            internal: vec![],
            children: vec![],
            size: parent_size / 12
        }]
    }
    pub fn brain(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Brain"),
            statuses: vec![],
            tags: vec![ BodyPartTag::thought],
            internal: vec![],
            children: vec![],
            size: parent_size / 2
        }]
    }
    pub fn eyes(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Eye"), 
            vec![BodyPartTag::sight], 
            count, 
            parent_size / 60
        );
    }
    pub fn ears(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Ear"), 
            vec![BodyPartTag::hearing], 
            count, 
            parent_size / 60
        );
    }
    pub fn nose(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Nose"),
            statuses: vec![],
            tags: vec![BodyPartTag::smell],
            internal: vec![],
            children: vec![],
            size: parent_size / 60
        }]
    }
}