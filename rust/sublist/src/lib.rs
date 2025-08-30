#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let (first_len, second_len) = (first_list.len(), second_list.len());

    if first_list == second_list {
        return Comparison::Equal;
    }

    if first_len < second_len && is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if first_len > second_len && is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist(small: &[i32], large: &[i32]) -> bool {
    if small.is_empty() {
        return true;
    }

    large.windows(small.len()).any(|window| window == small)
}
