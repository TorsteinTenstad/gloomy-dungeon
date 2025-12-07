use crate::{
    character_filter::CharacterFilter,
    data_model::{
        Ability, Action, ActionOnSelf, ActionTargeted, AreaEffect, Comparison, Condition,
        ConditionEffect, EffectOnCharacter, ItemData, ModifyGainedConditions, Passives, Reach,
        TriggeredAbilities,
    },
    hex_grid::DistanceRange,
    precondition::Precondition,
    turn_stats::TurnStat,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item {
    ShroudOfThePoisonFeeder, // Applied Poison is converted to Regen. Applied Regen is converted to Poison.
    CloakOfInvisibility, // At the end of your turn, if you are not adjacent to an enemy, gain Invisible(1). After every attack action, gain Fragile(1).
    ChestplateOfTheEnraged, // Every time damage is taken, gain Strong(1).
    MonksRobe, // After every movement action, you may gain Disarmed(1) to apply Stunned(1) to an adjacent enemy.
    ThorngrownVest, // At the end of your turn, if you didn't attack, gain Retaliate(2).
    BoodboundHarness, // Your actions consume Health instead of Stamina.
    // VestOfRecklessness, // At the start of you turn, if you are below 30% health, gain Fortified(2)
    // HolyRobe, // At the end of you turn, if you are at full health, ?

    // BootsOfSpeed, // At the start of your turn, gain Speed(1)
    // HeavyGreaves, // At the start of your turn, gain Fortified(1) and Slow(1)
    // HolySandals, // After every movement action, restore 1 health
    // SpringyShoes, // All your movement actions have Jump
    // SpikedBoots, // After every movement action, deal 1 damage to an adjacent enemy
    StillrootLegs, // At the start of you turn, if you didn't move last turn, gain one Stamina.
}

impl Item {
    pub fn data(self) -> ItemData {
        match self {
            Self::ShroudOfThePoisonFeeder => ItemData {
                description: "Applied Poison is converted to Regen. Applied Regen is converted to Poison.".into(),
                passives: Passives {
                    modify_gained_conditions: vec![
                        ModifyGainedConditions {
                            applies_only_to: Some(Condition::Poison),
                            transform_into: Some(Condition::Regen),
                            additive_factor: 0,
                            multiplicative_factor: 1.0,
                        },
                        ModifyGainedConditions {
                            applies_only_to: Some(Condition::Regen),
                            transform_into: Some(Condition::Poison),
                            additive_factor: 0,
                            multiplicative_factor: 1.0,
                        },
                    ],
                    ..Default::default()
                },
                triggered_abilities: Default::default(),
            },
            Self::CloakOfInvisibility => ItemData {
                description: "At the end of your turn, if you are not adjacent to an enemy, gain Invisible(1). After every attack action, gain Fragile(1).".into(),
                passives: Default::default(),
                triggered_abilities: TriggeredAbilities {
                    end_of_turn: vec![Ability {
                        precondition: Some(Precondition::FilteredCount {
                            filter: CharacterFilter::And(vec![
                                CharacterFilter::IsEnemy,
                                CharacterFilter::WithinDistance(DistanceRange{from: 1, to: 2}),
                            ]),
                            comparison: Comparison::Equal,
                            value: 0,
                        }),
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Invisible,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        })],
                    }],
                    attack_action: vec![Ability {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Fragile,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        })],
                    }],
                    ..Default::default()
                },
            },
            Self::ChestplateOfTheEnraged => ItemData {
                description: "Every time damage is taken, gain Strong(1).".into(),
                passives: Default::default(),
                triggered_abilities: TriggeredAbilities {
                    damage_taken: vec![Ability {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Strong,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        })],
                    }],
                    ..Default::default()
                },
            },
            Self::StillrootLegs => ItemData {
                description: "At the start of you turn, if you didn't move last turn, gain one Stamina.".into(),
                passives: Default::default(),
                triggered_abilities: TriggeredAbilities {
                    beginning_of_turn: vec![Ability {
                        precondition: Some(Precondition::TurnStat {
                            turn_index_relative: 1,
                            stat: TurnStat::SpacesMoved,
                            comparison: Comparison::Equal,
                            value: 0,
                        }),
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::GainStamina(1)],
                                ..Default::default()
                            }],
                        })],
                    }],
                    ..Default::default()
                },
            },
            Self::MonksRobe => ItemData {
                description: "After every movement action, you may gain Disarmed(1) to apply Stunned(1) to an adjacent enemy.".into(),
                passives: Default::default(),
                triggered_abilities: TriggeredAbilities {
                    movement_action: vec![
                        Ability {
                            precondition: None,
                            actions: vec![
                                Action::OnSelf(ActionOnSelf {
                                    effects: vec![AreaEffect {
                                        effects: vec![EffectOnCharacter::Condition(
                                            ConditionEffect {
                                                condition_type: Condition::Disarmed,
                                                value: 1,
                                            },
                                        )],
                                        ..Default::default()
                                    }],
                                }),
                                Action::Targeted(ActionTargeted {
                                    reach: Reach::Melee,
                                    effects: vec![AreaEffect {
                                        effects: vec![EffectOnCharacter::Condition(
                                            ConditionEffect {
                                                condition_type: Condition::Stunned,
                                                value: 1,
                                            },
                                        )],
                                        ..Default::default()
                                    }],
                                }),
                            ],
                        },
                        Ability {
                            precondition: None,
                            actions: vec![],
                        },
                    ],
                    ..Default::default()
                },
            },
            Self::ThorngrownVest => ItemData {
                description: "At the end of your turn, if you didn't attack, gain Retaliate(2).".into(),
                passives: Default::default(),
                triggered_abilities: TriggeredAbilities {
                    end_of_turn: vec![Ability {
                        precondition: Some(Precondition::TurnStat {
                            turn_index_relative: 1, // The engine processes turn stats before end of turn abilities, so we refer to last round
                            stat: TurnStat::AttackActions,
                            comparison: Comparison::Equal,
                            value: 0,
                        }),
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![EffectOnCharacter::Condition(ConditionEffect {
                                    condition_type: Condition::Retaliate,
                                    value: 2,
                                })],
                                ..Default::default()
                            }],
                        })],
                    }],
                    ..Default::default()
                },
            },
            Self::BoodboundHarness => ItemData {
                description: "Your actions consume Health instead of Stamina.".into(),
                passives: Passives {
                    actions_consume_health_instead_of_mana: true,
                    ..Default::default()
                },
                triggered_abilities: Default::default(),
            },
        }
    }
}
