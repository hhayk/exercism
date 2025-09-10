#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

fn count_and_score(dice: Dice, value: u8) -> u8 {
    dice.iter()
        .filter(|&&d| d == value)
        .count()
        .try_into()
        .unwrap_or(0)
        * value
}

pub fn score(dice: Dice, category: Category) -> u8 {
    //todo!("determine the score for {dice:?} in the {category:?}");
    match category {
        Category::Ones => count_and_score(dice, 1),
        Category::Twos => count_and_score(dice, 2),
        Category::Threes => count_and_score(dice, 3),
        Category::Fours => count_and_score(dice, 4),
        Category::Fives => count_and_score(dice, 5),
        Category::Sixes => count_and_score(dice, 6),
        Category::FullHouse => {
            let mut dice = dice;
            dice.sort();
            let set_with_count: Vec<(usize, u8)> = dice
                .chunk_by(|a, b| a == b)
                .map(|iter| (iter.iter().len(), iter.iter().sum::<u8>()))
                .collect();

            let [(s1, c1), (s2, c2)] = set_with_count[..] else {
                return 0;
            };

            let arr = [2, 3];
            if arr.contains(&s1) && arr.contains(&s2) {
                c1 + c2
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let set_with_count: Vec<(usize, u8)> = dice
                .chunk_by(|a, b| a == b)
                .map(|iter| (iter.iter().len(), iter.iter().sum::<u8>()))
                .collect();

            if set_with_count.len() == 1 {
                dice[0] * 4
            } else {
                let [(s1, _), (s2, c2)] = set_with_count[..] else {
                    return 0;
                };

                let arr = [1, 4];
                if arr.contains(&s1) && arr.contains(&s2) {
                    c2 * 4
                } else {
                    0
                }
            }
        }
        Category::LittleStraight => {
            let mut dice = dice;
            dice.sort();
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            let mut dice = dice;
            dice.sort();
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if dice.iter().all(|&d| d == dice[0]) {
                50
            } else {
                0
            }
        }
    }
}
