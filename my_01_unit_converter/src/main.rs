use std::io;

fn main() {
    let mut input = String::new();
    println!("값을 입력하세요:");
    io::stdin().read_line(&mut input).expect("cant read input");

    let unit = parse_input(input.trim());
    let converted = unit.converter();
    println!("{:.2} {}", converted.value(), converted.label());
}

enum Unit {
    C(f64),
    F(f64),
    Kg(f64),
    Lb(f64),
}

impl Unit {
    fn label(&self) -> &'static str {
        match self {
            Unit::C(_) => "C",
            Unit::F(_) => "F",
            Unit::Kg(_) => "kg",
            Unit::Lb(_) => "lb",
        }
    }

    fn value(&self) -> f64 {
        match self {
            Unit::C(v) | Unit::F(v) | Unit::Kg(v) | Unit::Lb(v) => *v,
        }
    }

    fn converter(self) -> Unit {
        match self {
            Unit::C(v) => Unit::F(v * 9.0 / 5.0 + 32.0),
            Unit::F(v) => Unit::C((v - 32.0) * (5.0 / 9.0)),
            Unit::Kg(v) => Unit::Lb(v * 2.2045226218),
            Unit::Lb(v) => Unit::Kg(v / 2.2045226218),
        }
    }
}

fn parse_input(input: &str) -> Unit {
    let mut parts = input.split_ascii_whitespace();

    let value: f64 = parts
        .next()
        .expect("값을 입력하세요.")
        .parse()
        .expect("숫자여야 합니다.");

    let unit_raw = parts.next().expect("단위를 입력하세요.").to_lowercase();

    match unit_raw.as_str() {
        "c" => Unit::C(value),
        "f" => Unit::F(value),
        "kg" => Unit::Kg(value),
        "lb" => Unit::Lb(value),
        _ => panic!("지원하지 않는 단위입니다."),
    }
}
