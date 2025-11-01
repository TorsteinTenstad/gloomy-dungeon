#![allow(dead_code)]
#![allow(unreachable_code)]

use std::{borrow::Cow, collections::HashMap};

struct Pos {
    r: isize,
    q: isize,
}

// All conditions stack and are decrease by 1 at the start of the character's turn
enum Condition {
    Poison,      // At the end of your turn, take X damage.
    Regen,       // At the end of your turn, heal X damage.
    Weak,        // Your attacks do X less damage.
    Strong,      // Your attacks do X more damage.
    Empowered,   // Your attacks deal double damage.
    Enfeebled,   // Your attacks deal half damage.
    Frail,       // Attacks against you deal X additional damage.
    Fortified,   // Attacks against you deal X less damage.
    Slow,        // -X to all movement actions.
    Fast,        // +X to all movement actions.
    Stunned,     // You can't perform any actions.
    Disarmed,    // You can't perform any attack actions.
    Immobilized, // You can't perform any movement actions.
    Retaliate,   // Attackers take X damage.
    Invisible,   // You can't be targeted.

    // Below conditions are not user-facing. They are seen as conditions to the engine, but not presented as such.
    Fury, // Melee attacks target all adjacent enemies.
}

struct ConditionEffect {
    condition_type: Condition,
    value: usize,
}

struct CardData {
    description: Cow<'static, str>,
    stamina_cost: usize,
    // Playing a card allow the player to do a series of ActionWithPrecondition. Each of these are cancelable
    actions: Vec<ActionWithPrecondition>,
}

struct ActionWithPrecondition {
    precondition: Option<Precondition>,
    actions: Actions,
}

// Once the first Action in the vector is confirmed by the player, the rest must be executed.
type Actions = Vec<Action>;

enum Action {
    OnSelf(ActionOnSelf),
    Targeted(ActionTargeted),
    Move(ActionMove),
}

struct ActionTargeted {
    reach: Reach,
    effects: Vec<AreaEffect>,
}

struct ActionOnSelf {
    effects: Vec<AreaEffect>,
}

struct ActionMove {
    spaces: usize,
    jump: bool,
}

enum Reach {
    Melee,
    Ranged { range: usize },
}

#[derive(Default)]
struct AreaEffect {
    area: Area,
    filter: Option<CharacterFilter>,
    effects: Vec<Effect>,
}

enum Effect {
    Damage(usize),
    DamageWithLifesteal(usize),
    Heal(usize),
    Condition(ConditionEffect),
    Push(usize),
    Pull(usize),
    GainStamina(usize),
}

struct Disk {
    inner_radius: usize,
    outer_radius: usize,
}

enum Area {
    Disk(Disk),
}

impl Default for Area {
    fn default() -> Self {
        Area::Disk(Disk {
            inner_radius: 0,
            outer_radius: 1,
        })
    }
}

enum Precondition {
    FilteredCount {
        filter: CharacterFilter,
        comparison: Comparison,
        value: usize,
    },
    RoundStat {
        round_index_relative: usize,
        stat: RoundStat,
        comparison: Comparison,
        value: usize,
    },
}

enum Comparison {
    Equal,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
}

enum RoundStat {
    SpacesMoved,
    AttackActions,
}

enum CharacterFilter {
    IsEnemy,
    IsSelf,
    NoAdjacentEnemies,
    WithCondition(Condition),
    WithoutCondition(Condition),
    And(Vec<CharacterFilter>),
    Or(Vec<CharacterFilter>),
}
struct Character {
    pos: Pos,
    health_current: usize,
    health_max: usize,
    passive_shield: usize,
    conditions: HashMap<Condition, usize>,
}

struct Player {
    character: Character,
    stamina_current: usize,
    stamina_max: usize,
    hand: Vec<CardData>,
}

struct ItemData {
    description: Cow<'static, str>,
    effects_passive: Vec<ItemEffectPassive>,
    effects_triggered: Vec<ItemEffectTriggered>,
}

struct ModifyGainedConditions {
    applies_only_to: Option<Condition>,
    transform_into: Option<Condition>,
    additive_factor: isize,
    multiplicative_factor: f32,
}

enum ItemEffectPassive {
    ModifyGainedConditions(ModifyGainedConditions),
    ActionsConsumeHealthInsteadOfStamina,
}

struct ItemEffectTriggered {
    trigger: TriggerEvent,
    precondition: Option<Precondition>,
    item_actions: ItemAction,
}

enum TriggerEvent {
    DamageTaken,
    AttackAction,
    MovementAction,
    StartOfTurn,
    EndOfTurn,
}

enum ItemAction {
    AutomaticOnSelf(ActionOnSelf),
    PlayerExecuted(Actions),
}

enum Item {
    ShroudOfThePoisonFeeder, // Applied Poison is converted to Regen. Applied Regen is converted to Poison.
    CloakOfInvisibility, // At the end of your turn, if you are not adjacent to an enemy, gain Invisible(1). After every attack action, gain Frail(1).
    ChestplateOfTheEnraged, // Every time damage is taken, gain Strong(1).
    StillrootPlate, // At the start of you turn, if you didn't move last turn, gain one Stamina.
    MonksRobe, // After every movement action, you may gain Disarmed(1) to apply Stunned(1) to an adjacent enemy.
    ThorngrownVest, // At the end of your turn, if you didn't attack, gain Retaliate(2).
    BoodboundHarness, // Your actions consume Health instead of Stamina.
}

impl Item {
    fn data(self) -> ItemData {
        match self {
            Self::ShroudOfThePoisonFeeder => ItemData {
                description: "Applied Poison is converted to Regen. Applied Regen is converted to Poison.".into(),
                effects_passive: vec![
                    ItemEffectPassive::ModifyGainedConditions(ModifyGainedConditions {
                        applies_only_to: Some(Condition::Poison),
                        transform_into: Some(Condition::Regen),
                        additive_factor: 0,
                        multiplicative_factor: 1.0,
                    }),
                    ItemEffectPassive::ModifyGainedConditions(ModifyGainedConditions {
                        applies_only_to: Some(Condition::Regen),
                        transform_into: Some(Condition::Poison),
                        additive_factor: 0,
                        multiplicative_factor: 1.0,
                    }),
                ],
                effects_triggered: vec![],
            },
            Self::CloakOfInvisibility => ItemData {
                description: "At the end of your turn, if you are not adjacent to an enemy, gain Invisible(1). After every attack action, gain Frail(1).".into(),
                effects_passive: vec![],
                effects_triggered: vec![
                    ItemEffectTriggered {
                        trigger: TriggerEvent::EndOfTurn,
                        precondition: Some(Precondition::FilteredCount {
                            filter: CharacterFilter::And(vec![
                                CharacterFilter::IsSelf,
                                CharacterFilter::NoAdjacentEnemies,
                            ]),
                            comparison: Comparison::Equal,
                            value: 0,
                        }),
                        item_actions: ItemAction::AutomaticOnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Invisible,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        }),
                    },
                    ItemEffectTriggered {
                        trigger: TriggerEvent::AttackAction,
                        precondition: None,
                        item_actions: ItemAction::AutomaticOnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Frail,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        }),
                    },
                ],
            },
            Self::ChestplateOfTheEnraged => ItemData {
                description: "Every time damage is taken, gain Strong(1).".into(),
                effects_passive: vec![],
                effects_triggered: vec![ItemEffectTriggered {
                    trigger: TriggerEvent::DamageTaken,
                    precondition: None,
                    item_actions: ItemAction::AutomaticOnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![Effect::Condition(ConditionEffect {
                                condition_type: Condition::Strong,
                                value: 1,
                            })],
                            ..Default::default()
                        }],
                    }),
                }],
            },
            Self::StillrootPlate => ItemData {
                description: "At the start of you turn, if you didn't move last turn, gain one Stamina.".into(),
                effects_passive: vec![],
                effects_triggered: vec![ItemEffectTriggered {
                    trigger: TriggerEvent::StartOfTurn,
                    precondition: Some(Precondition::RoundStat {
                        round_index_relative: 1,
                        stat: RoundStat::SpacesMoved,
                        comparison: Comparison::Equal,
                        value: 0,
                    }),
                    item_actions: ItemAction::AutomaticOnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![Effect::GainStamina(1)],
                            ..Default::default()
                        }],
                    }),
                }],
            },
            Self::MonksRobe => ItemData {
                description: "After every movement action, you may gain Disarmed(1) to apply Stunned(1) to an adjacent enemy.".into(),
                effects_passive: vec![],
                effects_triggered: vec![ItemEffectTriggered {
                    trigger: TriggerEvent::MovementAction,
                    precondition: None,
                    item_actions: ItemAction::PlayerExecuted(vec![
                        Action::Targeted(ActionTargeted {
                            reach: Reach::Melee,
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Stunned,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        }),
                        Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Disarmed,
                                    value: 1,
                                })],
                                ..Default::default()
                            }],
                        }),
                    ]),
                }],
            },
            Self::ThorngrownVest => ItemData {
                description: "At the end of your turn, if you didn't attack, gain Retaliate(2).".into(),
                effects_passive: vec![],
                effects_triggered: vec![ItemEffectTriggered {
                    trigger: TriggerEvent::EndOfTurn,
                    precondition: Some(Precondition::RoundStat {
                        round_index_relative: 0,
                        stat: RoundStat::AttackActions,
                        comparison: Comparison::Equal,
                        value: 0,
                    }),
                    item_actions: ItemAction::AutomaticOnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![Effect::Condition(ConditionEffect {
                                condition_type: Condition::Retaliate,
                                value: 2,
                            })],
                            ..Default::default()
                        }],
                    }),
                }],
            },
            Self::BoodboundHarness => ItemData {
                description: "Your actions consume Health instead of Stamina.".into(),
                effects_passive: vec![ItemEffectPassive::ActionsConsumeHealthInsteadOfStamina],
                effects_triggered: vec![],
            },
        }
    }
}

/*
Balancing (early game):
- BoodboundHarness forces a relationship between Health and Stamina.
  Assuming 3 normal turns should drain about half heath, we need Health == 6*Stamina
- Basic abilities yield +-1 damage. This having a 20% impact feels good, meaning that early game attacks should deal 5 damage.
- Since we want combos and multi-card play, typical round damage will probably be around 12.
- It feels good for the enemies have only slightly weaker attacks. Assume average 5 damage per enemy per round.
- The player will on average fight 4 "normal strength" enemies per encounter.
- Encounters should average 8 rounds. Assuming linear killing, this means taking 4+3+3+2+2+1+1 = 20 hits
- With ignorant play, this is 100 damage. Assuming good plays, 60 Health should be good. This means 10 Stamina
- With 12 damage from the player per round, enemies should average 24 health.
*/

enum Card {
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
}

impl Card {
    fn data(self) -> CardData {
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
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 3 },
                        effects: vec![AreaEffect {
                            effects: vec![Effect::Damage(2)],
                            area: Area::Disk(Disk {
                                inner_radius: 0,
                                outer_radius: 2,
                            }),
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::DrainLife => CardData {
                description: "Deal 3 damage (Range 3). Restore health equal to the damage done.".into(),
                stamina_cost: 3,
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 3 },
                        effects: vec![AreaEffect {
                            effects: vec![Effect::DamageWithLifesteal(3)],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::PoisonCloud => CardData {
                description: "Range 1. Apply Poison(3) to the target. Apply Poison(2) to all characters in range exactly 1 of the target. Apply Poison(1) to all characters in range exactly 2 of the target".into(),
                stamina_cost: 8,
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::Targeted(ActionTargeted {
                        reach: Reach::Ranged { range: 1 },
                        effects: vec![
                            AreaEffect {
                                area: Area::Disk(Disk {
                                    inner_radius: 0,
                                    outer_radius: 1,
                                }),
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Poison,
                                    value: 3,
                                })],
                                ..Default::default()
                            },
                            AreaEffect {
                                area: Area::Disk(Disk {
                                    inner_radius: 1,
                                    outer_radius: 2,
                                }),
                                effects: vec![Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Poison,
                                    value: 2,
                                })],
                                ..Default::default()
                            },
                            AreaEffect {
                                area: Area::Disk(Disk {
                                    inner_radius: 2,
                                    outer_radius: 3,
                                }),
                                effects: vec![Effect::Condition(ConditionEffect {
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
                actions: vec![ActionWithPrecondition {
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
                            effects: vec![Effect::Condition(ConditionEffect {
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
                actions: vec![
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::Move(ActionMove {
                            spaces: 1,
                            jump: false,
                        })],
                    },
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Condition(ConditionEffect {
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
                actions: vec![
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::Move(ActionMove {
                            spaces: 2,
                            jump: false,
                        })],
                    },
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::Targeted(ActionTargeted {
                            reach: Reach::Melee,
                            effects: vec![AreaEffect {
                                effects: vec![Effect::Damage(2)],
                                ..Default::default()
                            }],
                        })],
                    },
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::Move(ActionMove {
                            spaces: 2,
                            jump: false,
                        })],
                    },
                ],
            },
            Self::Whirlwind => CardData {
                description: "Deal 1 damage to all characters".into(),
                stamina_cost: 3,
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            area: Area::Disk(Disk {
                                inner_radius: 0,
                                outer_radius: usize::MAX,
                            }),
                            effects: vec![Effect::Damage(1)],
                            ..Default::default()
                        }],
                    })],
                }],
            },
            Self::Sting => CardData {
                description: "Deal 5 damage to all characters with Stunned".into(),
                stamina_cost: 6,
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            area: Area::Disk(Disk {
                                inner_radius: 0,
                                outer_radius: usize::MAX,
                            }),
                            filter: Some(CharacterFilter::WithCondition(Condition::Stunned)),
                            effects: vec![Effect::Damage(5)],
                        }],
                    })],
                }],
            },
            Self::Brawl => CardData {
                description: "Pull all enemies in within 2 hexes towards you.\nApply Weak(1) to all adjacent enemies.".into(),
                stamina_cost: 3,
                actions: vec![
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                area: Area::Disk(Disk {
                                    inner_radius: 2,
                                    outer_radius: 3,
                                }),
                                effects: vec![Effect::Pull(1)],
                                ..Default::default()
                            }],
                        })],
                    },
                    ActionWithPrecondition {
                        precondition: None,
                        actions: vec![Action::OnSelf(ActionOnSelf {
                            effects: vec![AreaEffect {
                                area: Area::Disk(Disk {
                                    inner_radius: 1,
                                    outer_radius: 2,
                                }),
                                effects: vec![Effect::Condition(ConditionEffect {
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
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![
                                Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Fortified,
                                    value: 3,
                                }),
                                Effect::Condition(ConditionEffect {
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
                description: "Restore 3 health.\nGain Strong(2).\nGain Frail(2).".into(),
                stamina_cost: 3,
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![
                                Effect::Heal(3),
                                Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Strong,
                                    value: 2,
                                }),
                                Effect::Condition(ConditionEffect {
                                    condition_type: Condition::Frail,
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
                actions: vec![ActionWithPrecondition {
                    precondition: None,
                    actions: vec![Action::OnSelf(ActionOnSelf {
                        effects: vec![AreaEffect {
                            effects: vec![Effect::Condition(ConditionEffect {
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
        actions: vec![ActionWithPrecondition {
            precondition: None,
            actions: vec![Action::Move(ActionMove {
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
        actions: vec![ActionWithPrecondition {
            precondition: None,
            actions: vec![Action::Targeted(ActionTargeted {
                reach: Reach::Melee,
                effects: vec![AreaEffect {
                    effects: vec![Effect::Damage(damage)],
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
        actions: vec![ActionWithPrecondition {
            precondition: None,
            actions: vec![Action::Targeted(ActionTargeted {
                reach: Reach::Ranged { range },
                effects: vec![AreaEffect {
                    effects: vec![Effect::Damage(damage)],
                    ..Default::default()
                }],
            })],
        }],
    }
}

fn main() {
    println!("Hello, world!");
}
