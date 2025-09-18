pub struct RailFence {
    rails: i32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        //todo!("Construct a new fence with {rails} rails")
        Self {
            rails: rails as i32,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        //todo!("Encode this text: {text}")
        let mut rails = vec![String::new(); self.rails as usize];
        let mut current_rail = 0;
        let mut direction = 1;

        for ch in text.chars() {
            rails[current_rail as usize].push(ch);

            if current_rail == 0 {
                direction = 1;
            } else if current_rail == self.rails - 1 {
                direction = -1;
            }

            current_rail += direction;
        }

        rails.join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        //todo!("Decode this ciphertext: {cipher}")
        let cipher_len = cipher.len();
        let num_rails = self.rails as usize;

        let mut grid = vec![vec!['\0'; cipher_len]; num_rails];

        let mut row = 0i32;
        let mut direction = 1i32;
        let mut positions = Vec::with_capacity(cipher_len);

        for col in 0..cipher_len {
            positions.push((row as usize, col));
            grid[row as usize][col] = '*';

            if row == 0 {
                direction = 1;
            } else if row == self.rails - 1 {
                direction = -1;
            }

            row += direction;
        }

        let mut cipher_chars = cipher.chars();
        for r in 0..num_rails {
            for c in 0..cipher_len {
                if grid[r][c] == '*' {
                    if let Some(ch) = cipher_chars.next() {
                        grid[r][c] = ch;
                    }
                }
            }
        }

        let mut decoded_text = String::with_capacity(cipher_len);
        for (r, c) in positions {
            decoded_text.push(grid[r][c]);
        }

        decoded_text
    }
}
