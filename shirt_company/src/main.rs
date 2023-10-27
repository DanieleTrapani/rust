#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => blue += 1,
                ShirtColor::Red => red += 1,
            }
        }

        if red > blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    todo!()
}
