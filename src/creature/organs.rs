pub mod organs {
    use crate::creature::{body::body::{BodyPartTags, BodyPart}};
    fn gen_organ_set(name: String, tags: Vec<BodyPartTags>, count: i32, size: i32) -> Vec<BodyPart> {
        let mut output: Vec<BodyPart> = vec![];
        for _i in 0..count {
            output.push(BodyPart {
                name: name.clone(),
                tags: tags.clone(),
                internal: vec![],
                children: vec![],
                size
            })
        }
        if count == 2 {
            output[0].tags.push(BodyPartTags::left);
            output[1].tags.push(BodyPartTags::right);
        }
        return output;
    }
    pub fn hearts(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Heart"), 
            vec![BodyPartTags::circulation], 
            count, 
            parent_size / 20
        );
    }
    pub fn lungs(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Lung"), 
            vec![BodyPartTags::breath], 
            count, 
            parent_size / 10
        );
    }
    pub fn spine(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Spine"),
            tags: vec![ BodyPartTags::nervous],
            internal: vec![],
            children: vec![],
            size: parent_size / 12
        }]
    }
    pub fn brain(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Brain"),
            tags: vec![ BodyPartTags::thought],
            internal: vec![],
            children: vec![],
            size: parent_size / 2
        }]
    }
    pub fn eyes(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Eye"), 
            vec![BodyPartTags::sight], 
            count, 
            parent_size / 60
        );
    }
    pub fn ears(count: i32, parent_size: i32) -> Vec<BodyPart> {
        return gen_organ_set(
            String::from("Ear"), 
            vec![BodyPartTags::hearing], 
            count, 
            parent_size / 60
        );
    }
    pub fn nose(parent_size: i32) -> Vec<BodyPart> {
        return vec![BodyPart {
            name: String::from("Nose"),
            tags: vec![BodyPartTags::smell],
            internal: vec![],
            children: vec![],
            size: parent_size / 60
        }]
    }
}