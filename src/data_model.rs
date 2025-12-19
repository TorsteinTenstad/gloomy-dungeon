#![allow(dead_code)]
use crate::{
    character_filter::CharacterFilter,
    enum_map::EnumMap,
    hex_grid::{Area, PosAxial},
    items::Item,
    precondition::Precondition,
    turn_stats::TurnStats,
};
use std::borrow::Cow;

// All conditions stack and are decrease by 1 at the start of the character's turn
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Condition {
    Poison,      // At the end of your turn, take X damage.
    Regen,       // At the end of your turn, heal X damage.
    Weak,        // Your attacks do X less damage.
    Strong,      // Your attacks do X more damage.
    Empowered,   // Your attacks deal double damage.
    Enfeebled,   // Your attacks deal half damage.
    Fragile,     // Attacks against you deal X additional damage.
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionEffect {
    pub condition_type: Condition,
    pub value: isize,
}

pub struct CardData {
    pub description: Cow<'static, str>,
    pub stamina_cost: usize,
    pub abilities: Vec<Ability>,
}

// Abilities consists of 0 or more actions. Abilities can always be canceled.
// Each individual action can not be canceled. Once the first is confirmed, the next must be executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ability {
    pub precondition: Option<Precondition>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    OnSelf(ActionOnSelf),
    Targeted(ActionTargeted),
    Movement(ActionMovement),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOnSelf {
    pub effects: Vec<AreaEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionTargeted {
    pub reach: Reach,
    pub effects: Vec<AreaEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMovement {
    pub spaces: usize,
    pub jump: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reach {
    Melee,
    Ranged { range: usize },
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AreaEffect {
    pub area: Area,
    pub filter: Option<CharacterFilter>,
    pub effects: Vec<EffectOnCharacter>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectOnCharacter {
    Damage(usize),
    DamageWithLifesteal(usize),
    Heal(usize),
    Condition(ConditionEffect),
    GainStamina(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
}

impl Comparison {
    pub fn to_fn<T: PartialOrd + PartialEq>(&self) -> fn(&T, &T) -> bool {
        type CmpFn<T> = fn(&T, &T) -> bool;

        match self {
            Comparison::Equal => T::eq as CmpFn<T>,
            Comparison::Less => T::lt as CmpFn<T>,
            Comparison::Greater => T::gt as CmpFn<T>,
            Comparison::LessOrEqual => T::le as CmpFn<T>,
            Comparison::GreaterOrEqual => T::ge as CmpFn<T>,
        }
    }

    pub fn compare<T: PartialOrd + PartialEq>(&self, lhs: &T, rhs: &T) -> bool {
        let cmp = self.to_fn();
        cmp(lhs, rhs)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CharacterTeam {
    Player,
    #[default]
    Monster,
}

pub type Conditions = EnumMap<Condition>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Character {
    pub pos: PosAxial,
    pub team: CharacterTeam,
    pub health_current: usize,
    pub health_max: usize,
    pub stamina_current: usize,
    pub stamina_max: usize,
    pub equipped_items: Vec<Item>,
    pub conditions: Conditions,
    pub turn_stats: TurnStats,
    pub remaining_abilities: Vec<Ability>,
}

pub struct Player {
    pub character: Character,
    pub stamina_current: usize,
    pub stamina_max: usize,
    pub hand: Vec<CardData>,
}

#[derive(Default, Debug, Clone)]
pub struct Passives {
    pub actions_consume_health_instead_of_mana: bool,
    pub modify_gained_conditions: Vec<ModifyGainedConditions>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbilities {
    pub damage_taken: Vec<Ability>,
    pub attack_action: Vec<Ability>,
    pub movement_action: Vec<Ability>,
    pub beginning_of_turn: Vec<Ability>,
    pub end_of_turn: Vec<Ability>,
}

#[derive(Debug, Clone)]
pub struct ItemData {
    pub description: Cow<'static, str>,
    pub passives: Passives,
    pub triggered_abilities: TriggeredAbilities,
}

#[derive(Debug, Clone)]
pub struct ModifyGainedConditions {
    pub applies_only_to: Option<Condition>,
    pub transform_into: Option<Condition>,
    pub additive_factor: isize,
    pub multiplicative_factor: f32,
}
