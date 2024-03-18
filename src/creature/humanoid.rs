pub mod humanoid {
    use crate::creature::body::body::*;
    use crate::creature::mind::mind::random_mind;
    use crate::creature::organs::organs::*;
    use crate::creature::creature::*;


    const LIMB_SIZE: u32 = 400;
    const HEAD_SIZE: u32 = 300;
    const BODY_SIZE: u32 = 2000;

    // fn limb_2pt(name: String, side: BodyPartTag, tags: Vec<BodyPartTag>) -> BodyPart {
    //     let prefix = if side == BodyPartTag::Left {"Left "} else if side == BodyPartTag::Right{"Right "} else {""};
    //     let upper_name = format!("{}Upper {}", prefix, name);
    //     let lower_name = format!("{}Lower {}", prefix, name);
    //     return BodyPart {
    //         name: upper_name.clone(),
    //         tags: vec![side],
    //         statuses: vec![],
    //         organs: vec![bone(LIMB_SIZE, upper_name.clone())].concat(),
    //         children: vec![
    //             BodyPart {
    //                 name: lower_name.clone(),
    //                 tags: vec![vec![side], tags].concat(),
    //                 statuses: vec![],
    //                 organs: vec![bone(LIMB_SIZE, lower_name.clone())].concat(),
    //                 children: vec![],
    //                 size: LIMB_SIZE / 2
    //             }
    //         ],
    //         size: LIMB_SIZE/2
    //     }
    // }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum LimbType {
        Leg,
        Arm
    }

    fn limb_2pt(limb_type: LimbType, side: BodyPartTag) -> BodyPart {
        let limb_type_name = if limb_type == LimbType::Leg { "Leg" } else { "Arm"};
        let prefix = if side == BodyPartTag::Left {"Left"} else if side == BodyPartTag::Right{"Right"} else {""};
        let upper_name = format!("{} Upper {}", prefix, limb_type_name);
        let lower_name = format!("{} Lower {}", prefix, limb_type_name);

        let limb_end = if limb_type == LimbType::Leg {
            BodyPart {
                name: format!("{} Foot", prefix),
                tags: vec![side, BodyPartTag::Stance],
                statuses: vec![],
                organs: bone(LIMB_SIZE / 4, format!("{} Foot", prefix)),
                children: vec![],
                size: LIMB_SIZE/3
            }
        } else {
            BodyPart {
                name: format!("{} Hand", prefix),
                tags: vec![side, BodyPartTag::Grasp],
                statuses: vec![],
                organs: bone(LIMB_SIZE / 4, format!("{} Foot", prefix)),
                children: vec![],
                size: LIMB_SIZE/3
            }
        };

        BodyPart {
            name: upper_name.clone(),
            tags: vec![side],
            statuses: vec![],
            organs: vec![bone(LIMB_SIZE, upper_name.clone())].concat(),
            children: vec![
                BodyPart {
                    name: lower_name.clone(),
                    tags: vec![side],
                    statuses: vec![],
                    organs: vec![bone(LIMB_SIZE, lower_name.clone())].concat(),
                    children: vec![limb_end],
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
                organs: vec![
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
                            organs: vec![
                                skull(HEAD_SIZE),
                                eyes(2, HEAD_SIZE),
                                ears(2, HEAD_SIZE),
                                nose(HEAD_SIZE),
                                brain(HEAD_SIZE)
                            ].concat(),
                            children: vec![],
                            size: HEAD_SIZE
                        },
                        limb_2pt(LimbType::Arm, BodyPartTag::Left),
                        limb_2pt(LimbType::Arm, BodyPartTag::Right),
                        limb_2pt(LimbType::Leg, BodyPartTag::Left),
                        limb_2pt(LimbType::Leg, BodyPartTag::Right)
                    ]
                ].concat(),
                size: BODY_SIZE
            },
            mind: random_mind()
        }
    }
}