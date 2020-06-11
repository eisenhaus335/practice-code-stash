use std::io;

struct Money {
    value: i32,
    currency: String
}

impl Money {
    fn new(value: i32, currency: &str) -> Money{
        let currency = String::from(currency.trim());
        Money {
            value,
            currency
        }
    }
    
    fn converse(&self, currency: &str) -> i32{
        match self.currency.as_str() {
            "USD" => match currency{
                    "USD" => self.value,
                    "IDR" => self.value * 15000,
                    _ => 0
                },
            "IDR" => match currency{
                "USD" => self.value / 15000,
                "IDR" => self.value,
                _ => 5
            },
            _ => {
                println!("ngentot");
                2
            },
        }
    }
}

fn main() {
        let mut currency = String::new();
        println!("Type your currency");

        io::stdin().read_line(&mut currency).expect("Something is wrong");

        let mut money = String::new();
        println!("Type your money");

        io::stdin().read_line(&mut money).expect("Error");

        let money = money.trim().parse().expect("Error");
        let m = Money::new(money, currency.as_str());

        let mut currency = String::new();
        println!("Type your converted currency");

        io::stdin().read_line(&mut currency).expect("Error");

        let result = m.converse(&currency.trim());
        println!("result is {}", result);
}
