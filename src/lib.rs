#![feature(conservative_impl_trait)]

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    Neutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
    Unaligned,
}

impl Ability {
    pub fn skills(&self) -> &[Skill] {
        match *self {
            Ability::Strength => &[Skill::Athletics],
            Ability::Dexterity => &[Skill::Acrobatics, Skill::SleightOfHand, Skill::Stealth],
            Ability::Constitution => &[],
            Ability::Intelligence => &[
                Skill::Arcana,
                Skill::History,
                Skill::Investigation,
                Skill::Nature,
                Skill::Religion,
            ],
            Ability::Wisdom => &[
                Skill::AnimalHandling,
                Skill::Insight,
                Skill::Medicine,
                Skill::Perception,
                Skill::Survival,
            ],
            Ability::Charisma => &[
                Skill::Deception,
                Skill::Intimidation,
                Skill::Performance,
                Skill::Persuasion,
            ],
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Skill {
    Athletics,
    Acrobatics,
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

impl Skill {
    pub fn ability(&self) -> Ability {
        match *self {
            Skill::Athletics => Ability::Strength,
            Skill::Acrobatics => Ability::Dexterity,
            Skill::SleightOfHand => Ability::Dexterity,
            Skill::Stealth => Ability::Dexterity,
            Skill::Arcana => Ability::Intelligence,
            Skill::History => Ability::Intelligence,
            Skill::Investigation => Ability::Intelligence,
            Skill::Nature => Ability::Intelligence,
            Skill::Religion => Ability::Intelligence,
            Skill::AnimalHandling => Ability::Wisdom,
            Skill::Insight => Ability::Wisdom,
            Skill::Medicine => Ability::Wisdom,
            Skill::Perception => Ability::Wisdom,
            Skill::Survival => Ability::Wisdom,
            Skill::Deception => Ability::Charisma,
            Skill::Intimidation => Ability::Charisma,
            Skill::Performance => Ability::Charisma,
            Skill::Persuasion => Ability::Charisma,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
