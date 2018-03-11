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

impl Ability {
    pub fn skills(&self) -> impl Iterator<Item = Skill> {
        match *self {
            Ability::Strength => vec![Skill::Athletics].into_iter(),
            Ability::Dexterity => {
                vec![Skill::Acrobatics, Skill::SleightOfHand, Skill::Stealth].into_iter()
            }
            Ability::Constitution => vec![].into_iter(),
            Ability::Intelligence => vec![
                Skill::Arcana,
                Skill::History,
                Skill::Investigation,
                Skill::Nature,
                Skill::Religion,
            ].into_iter(),
            Ability::Wisdom => vec![
                Skill::AnimalHandling,
                Skill::Insight,
                Skill::Medicine,
                Skill::Perception,
                Skill::Survival,
            ].into_iter(),
            Ability::Charisma => vec![
                Skill::Deception,
                Skill::Intimidation,
                Skill::Performance,
                Skill::Persuasion,
            ].into_iter(),
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
