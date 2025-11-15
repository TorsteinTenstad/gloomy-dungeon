#![allow(dead_code)]

use crate::{
    character_filter::CharacterFilter,
    data_model::{
        Ability, Action, ActionMovement, ActionOnSelf, ActionTargeted, AreaEffect, CardData,
        Comparison, Condition, ConditionEffect, EffectOnCharacter, Reach,
    },
    hex_grid::{Area, DistanceRange},
    precondition::Precondition,
};

#[derive(Debug)]
pub enum Card {
    Step,
    Dash,
    Sprint,
    Cut,
    Strike,
    LargeStrike,
    SteadyShot,
    RainOfArrows,
    DrainLife,
    PoisonCloud,
    Preparation,
    ShadowStep,
    Backstab,
    Whirlwind,
    Sting,
    Brawl,
    Calm,
    Adrenaline,
    Fury,
    // Move, deal damage equal to move
    // Fortified acts like strength
}

impl Card {
    pub fn data(self) -> CardData {
        match self {
            Self::Step => basic_move(1, 2),
            Self::Dash => basic_move(2, 4),
            Self::Sprint => basic_move(3, 6),
            Self::Cut => basic_melee_attack(1, 2),
            Self::Strike => basic_melee_attack(5, 5),
            Self::LargeStrike => basic_melee_attack(10, 8),
            Self::SteadyShot => basic_ranged_attack(3, 3, 5),
            Self::RainOfArrows => CardData {
                description: "Deal 2 damage (Range 3). Also affects enemies adjacent to the target".into(),
                stamina_cost: 8,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 3 },
                        effects: vec![AreaEffect {
                            effects: vec![EffectOnCharacter::Damage(2)],
                            area: Area::Disk(DistanceRange {
                                from: 0,
                                to: 2,
                            }),
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::DrainLife => CardData {
                description: "Deal 3 damage (Range 3). Restore health equal to the damage done.".into(),
                stamina_cost: 3,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 3 },
                        effects: vec![AreaEffect {
                            effects: vec![EffectOnCharacter::DamageWithLifesteal(3)],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::PoisonCloud => CardData {
                description: "Range 1. Apply Poison(3) to the target. Apply Poison(2) to all characters in range exactly 1 of the target. Apply Poison(1) to all characters in range exactly 2 of the target".into(),
                stamina_cost: 8,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 1 },
                        effects: vec![
                            AreaEffect {
                                area: Area::Disk(DistanceRange {
                                    from: 0,
                                    to: 1,
                                }),
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Poison,
                                    value: 3,
                                })],
                                ..Default::default()
                            },
                            AreaEffect {
                                area: Area::Disk(DistanceRange {
                                    from: 1,
                                    to: 2,
                                }),
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Poison,
                                    value: 2,
                                })],
                                ..Default::default()
                            },
                            AreaEffect {
                                area: Area::Disk(DistanceRange {
                                    from: 2,
                                    to: 3,
                                }),
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Poison,
                                    value: 1,
                                })],
                                ..Default::default()
                            },
                        ],
                    })],
                }],
            },
            Self::Preparation => CardData {
                description: "If you are Invisible, gain Empowered(1)".into(),
                stamina_cost: 2,
                abilities: vec![Ability {
                    precondition: Some(Precondition::FilteredCount {
                        filter: CharacterFilter::And(vec![
                            CharacterFilter::IsSelf,
                            CharacterFilter::WithCondition(Condition::Invisible),
                        ]),
                        comparison: Comparison::Greater,
                        value: 0,
                    }),
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                condition_type: Condition::Empowered,
                                value: 1,
                            })],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::ShadowStep => CardData {
                description: "Move 1.\nGain Invisible(1)".into(),
                stamina_cost: 7,
                abilities: vec![
                    Ability {
                        precondition: None,
                        actions: vec![Action::Movement(ActionMovement {
                            spaces: 1,
                            jump: false,
                        })],
                    },
                    Ability {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Invisible,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        })],
                    },
                ],
            },
            Self::Backstab => CardData {
                description: "Move 2.\nMelee. Deal 2 damage.\nMove 2.".into(),
                stamina_cost: 3,
                abilities: vec![
                    Ability {
                        precondition: None,
                        actions: vec![Action::Movement(ActionMovement {
                            spaces: 2,
                            jump: false,
                        })],
                    },
                    Ability {
                        precondition: None,
                        actions: vec![Action::Targeted(ActionTargeted {
                            reach: Reach::Melee,
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Damage(2)],
                                ..Default::default()
                            }],
                        })],
                    },
                    Ability {
                        precondition: None,
                        actions: vec![Action::Movement(ActionMovement {
                            spaces: 2,
                            jump: false,
                        })],
                    },
                ],
            },
            Self::Whirlwind => CardData {
                description: "Deal 1 damage to all characters".into(),
                stamina_cost: 3,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            area: Area::Disk(DistanceRange {
                                from: 0,
                                to: usize::MAX,
                            }),
                            effects: vec![EffectOnCharacter::Damage(1)],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::Sting => CardData {
                description: "Deal 5 damage to all characters with Stunned".into(),
                stamina_cost: 6,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            area: Area::Disk(DistanceRange {
                                from: 0,
                                to: usize::MAX,
                            }),
                            filter: Some(CharacterFilter::WithCondition(Condition::Stunned)),
                            effects: vec![EffectOnCharacter::Damage(5)],
                        }],
                    })],
                }],
            },
            Self::Brawl => CardData {
                description: "Pull all enemies in within 2 hexes towards you.\nApply Weak(1) to all adjacent enemies.".into(),
                stamina_cost: 3,
                abilities: vec![
                    Ability {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                area: Area::Disk(DistanceRange {
                                    from: 2,
                                    to: 3,
                                }),
                                effects: vec![/*EffectOnCharacter::Pull(1)*/],
                                ..Default::default()
                            }],
                        })],
                    },
                    Ability {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                area: Area::Disk(DistanceRange {
                                    from: 1,
                                    to: 2,
                                }),
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Weak,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        })],
                    },
                ],
            },
            Self::Calm => CardData {
                description: "Gain Fortified(3).\nGain Immobilized(2).".into(),
                stamina_cost: 5,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![
                                EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Fortified,
                                    value: 3,
                                }),
                                EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Immobilized,
                                    value: 2,
                                }),
                            ],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::Adrenaline => CardData {
                description: "Restore 3 health.\nGain Strong(2).\nGain Fragile(2).".into(),
                stamina_cost: 3,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![
                                EffectOnCharacter::Heal(3),
                                EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Strong,
                                    value: 2,
                                }),
                                EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Fragile,
                                    value: 2,
                                }),
                            ],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::Fury => CardData {
                description: "All melee attacks this turn targets all adjacent enemies".into(),
                stamina_cost: 5,
                abilities: vec![Ability {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                condition_type: Condition::Fury,
                                value: 1,
                            })],
                            ..Default::default()
                        }],
                    })],
                }],
            },
        }
    }
}

fn basic_move(stamina_cost: usize, spaces: usize) -> CardData {
    CardData {
        description: format!("Move {}", spaces).into(),
        stamina_cost,
        abilities: vec![Ability {
            precondition: None,
            actions: vec![Action::Movement(ActionMovement {
                spaces,
                jump: false,
            })],
        }],
    }
}

fn basic_melee_attack(stamina_cost: usize, damage: usize) -> CardData {
    CardData {
        description: format!("Deal {} damage (Melee)", damage).into(),
        stamina_cost,
        abilities: vec![Ability {
            precondition: None,
            actions: vec![Action::Targeted(ActionTargeted {
                reach: Reach::Melee,
                effects: vec![AreaEffect {
                    effects: vec![EffectOnCharacter::Damage(damage)],
                    ..Default::default()
                }],
            })],
        }],
    }
}

fn basic_ranged_attack(stamina_cost: usize, range: usize, damage: usize) -> CardData {
    CardData {
        description: format!("Deal {} damage (Range {})", damage, range).into(),
        stamina_cost,
        abilities: vec![Ability {
            precondition: None,
            actions: vec![Action::Targeted(ActionTargeted {
                reach: Reach::Ranged { range },
                effects: vec![AreaEffect {
                    effects: vec![EffectOnCharacter::Damage(damage)],
                    ..Default::default()
                }],
            })],
        }],
    }
}
