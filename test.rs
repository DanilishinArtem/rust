pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Значение догадки должно быть между 1 и 100, получено {}.", value);
        }
        Guess {
            value
        }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

