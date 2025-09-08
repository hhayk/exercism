pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        //        todo!(
        //            "Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid."
        //        );
        let [a, b, c] = sides;
        if a > 0 && b > 0 && c > 00 && a + b >= c && a + c >= b && b + c >= a {
            Some(Self { a, b, c })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        // todo!("Determine if the Triangle is equilateral.");
        self.a == self.b && self.b == self.c && self.c == self.a
    }

    pub fn is_scalene(&self) -> bool {
        // todo!("Determine if the Triangle is scalene.");
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        // todo!("Determine if the Triangle is isosceles.");
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}
