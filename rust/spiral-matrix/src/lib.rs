pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }

    let mut matrix = vec![vec![0; size as usize]; size as usize];

    let mut num: u32 = 1;
    let mut top: u32 = 0;
    let mut bottom: u32 = size - 1;
    let mut left: u32 = 0;
    let mut right: u32 = size - 1;

    while top <= bottom && left <= right {
        // Traverse from left to right along the top row
        for col in left..=right {
            matrix[top as usize][col as usize] = num;
            num += 1;
        }
        top += 1;

        // Traverse from top to bottom along the right column
        for row in top..=bottom {
            matrix[row as usize][right as usize] = num;
            num += 1;
        }
        // Ensure right doesn't underflow if size is 1
        if right == 0 {
            break;
        }
        right -= 1;

        // Traverse from right to left along the bottom row
        if top <= bottom {
            for col in (left..=right).rev() {
                matrix[bottom as usize][col as usize] = num;
                num += 1;
            }
            bottom -= 1;
        }

        // Traverse from bottom to top along the left column
        if left <= right {
            for row in (top..=bottom).rev() {
                matrix[row as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}
