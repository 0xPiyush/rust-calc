use std::env::args;

fn main() {
    let mut args = args();

    let first = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator = args.next().unwrap().chars().next().unwrap();
    let second = args.next().unwrap().parse::<f32>().unwrap();

    let result = operate(first, operator, second);

    println!("{}", output(first, operator, second, result));
}

fn operate(first: f32, operator: char, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' | 'x' | 'X' => first * second,
        '^' => first.powf(second),
        '/' => first / second,
        _ => panic!("Unknown operator"),
    }
}

fn output(first: f32, operator: char, second: f32, result: f32) -> String {
    return format!("{} {} {} = {}", first, operator, second, result);
}
