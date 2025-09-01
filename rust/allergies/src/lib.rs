pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
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
        (self.score & (*allergen as u32)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        //todo!(
        //    "Return the list of allergens contained within the score with which the Allergies struct was made."
        //);
        Allergen::VALUES
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .cloned()
            .collect()
    }
}
