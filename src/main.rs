mod polynomial;

fn main() {
    use polynomial::Polynomial;
    use std::collections::HashMap;

    // Example: 2x^2y + 3xy^2 + 1
    let variables = vec!["x".to_string(), "y".to_string()];
    let p = Polynomial::from_terms(
        variables.clone(),
        vec![
            (vec![2,1], 2.0), // 2x^2y
            (vec![1,2], 3.0), // 3xy^2
            (vec![0,0], 1.0), // 1
        ]
    );
    // Example: x + y
    let q = Polynomial::from_terms(
        variables.clone(),
        vec![
            (vec![1,0], 1.0), // x
            (vec![0,1], 1.0), // y
        ]
    );
    let sum = p.add(&q);
    let prod = p.mul(&q);
    println!("p(x, y) = {}", p);
    println!("q(x, y) = {}", q);
    println!("p(x, y) + q(x, y) = {}", sum);
    println!("p(x, y) * q(x, y) = {}", prod);
}
