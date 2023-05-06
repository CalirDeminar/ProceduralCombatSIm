pub mod humanoid {
    use crate::creature::body::body::*;
    use crate::creature::organs::organs::*;
    use crate::creature::creature::*;


    const LIMB_SIZE: u32 = 400;
    const HEAD_SIZE: u32 = 300;
    const BODY_SIZE: u32 = 2000;

    fn limb_2pt(name: String, side: BodyPartTag, tags: Vec<BodyPartTag>) -> BodyPart {
        let prefix = if side == BodyPartTag::Left {"Left "} else if side == BodyPartTag::Right{"Right "} else {""};
        return BodyPart {
            name: format!("{}Upper {}", prefix, name),
            tags: vec![side],
            statuses: vec![],
            internal: vec![],
            children: vec![
                BodyPart {
                    name: format!("{}Lower {}", prefix, name),
                    tags: vec![vec![side], tags].concat(),
                    statuses: vec![],
                    internal: vec![],
                    children: vec![],
                    size: LIMB_SIZE / 2
                }
            ],
            size: LIMB_SIZE/2
        }
    }

    pub fn humanoid() -> Creature {
        return Creature {
            species: String::from("Human"),
            health_stats: base_health_stats(),
            body: BodyPart {
                name: String::from("Body"),
                tags: vec![],
                statuses: vec![],
                internal: vec![
                    hearts(1, BODY_SIZE),
                    spine(BODY_SIZE),
                    lungs(2, BODY_SIZE),
                ].concat(),
                children: vec![
                    vec![
                        BodyPart {
                            name: String::from("Head"),
                            tags: vec![],
                            statuses: vec![],
                            internal: vec![
                                eyes(2, HEAD_SIZE),
                                ears(2, HEAD_SIZE),
                                nose(HEAD_SIZE),
                            ].concat(),
                            children: vec![],
                            size: HEAD_SIZE
                        },
                        limb_2pt(String::from("Arm"), BodyPartTag::Left, vec![BodyPartTag::Grasp]),
                        limb_2pt(String::from("Arm"), BodyPartTag::Right, vec![BodyPartTag::Grasp]),
                        limb_2pt(String::from("Leg"), BodyPartTag::Right, vec![BodyPartTag::Stance]),
                        limb_2pt(String::from("Leg"), BodyPartTag::Left, vec![BodyPartTag::Stance]),
                    ]
                ].concat(),
                size: BODY_SIZE
            }
        }
    }
}