pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::organs::organs::*;
    use crate::creature::humanoid::*;
    use crate::creature::body::body::*;

    #[derive(Debug, Clone)]
    pub struct Creature {
        pub species: String,
        pub body: BodyPart
    }

}

#[cfg(test)]
mod tests {
    use crate::creature::{humanoid::humanoid::humanoid, body::body::random_weighted_part_with_internal};

    #[test]
    fn flatten() {
        let subject = humanoid();
        println!("{:#?}", subject);
        for _i in 0..100 {
            let part = random_weighted_part_with_internal(&subject.body);
            println!("{:?}", part.name);
        }
    }
}