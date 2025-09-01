pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    const VALUES: [Self; 8] = [
        Self::Eggs,
        Self::Peanuts,
        Self::Shellfish,
        Self::Strawberries,
        Self::Tomatoes,
        Self::Chocolate,
        Self::Pollen,
        Self::Cats,
    ];
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // todo!("Given the '{score}' score, construct a new Allergies struct.");
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        //todo!(
        //    "Return the list of allergens contained within the score with which the Allergies struct was made."
        //);
        Allergen::VALUES
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (idx, el)| {
                if (self.score >> idx) & 1 == 1 {
                    acc.push(el.clone());
                }
                acc
            })
    }
}
