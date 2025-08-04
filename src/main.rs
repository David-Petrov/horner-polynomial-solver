use horner_solver::rational::Rational;
use horner_solver::polynomial::Polynomial;

fn main() {
    println!("Enter space separated coefficients in ascending order:");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("polynomial coefficients expected");
    let coefficients: Vec<i64> = line
        .trim()
        .split(' ')
        .map(str::parse)
        .collect::<Result<Vec<i64>, _>>()
        .expect("valid integer coefficients expected");

    let polynomial = Polynomial::new(coefficients);
    let rational_roots = polynomial
        .find_rational_roots()
        .iter()
        .map(Rational::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    println!("Rational roots of the polynomial {} are:\n{}", polynomial, rational_roots);
}
