use std::cmp;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    //todo!("calculate the maximum value achievable with the given {items:?} and {max_weight}");
    rec(max_weight, items, items.len())
}

fn rec(max_weight: u32, items: &[Item], n: usize) -> u32 {
    if n == 0 || max_weight == 0 {
        return 0;
    }

    let current_item = &items[n - 1];

    if current_item.weight > max_weight {
        rec(max_weight, items, n - 1)
    } else {
        let included_item =
            current_item.value + rec(max_weight - current_item.weight, items, n - 1);
        let excluded_item = rec(max_weight, items, n - 1);

        cmp::max(included_item, excluded_item)
    }
}
