#[derive(Debug)]
struct BigInt {
    value: String
}

impl BigInt {
    fn get_digits(&self) -> Vec<u32> {
        self.value.chars().flat_map(|c| c.to_digit(10)).collect()
    }
}

fn main() {
    let huge_number: BigInt = BigInt {value: "725935728739872".to_string()};
    let huge_number_digits: Vec<u32> = huge_number.get_digits();

    println!("Huge number: {:?}, digits: {:?}", huge_number, huge_number_digits);
}
