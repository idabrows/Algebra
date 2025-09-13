
# Algebra Rust Project with Docker Compose

This project demonstrates a Rust application for algebraic computations, including multivariate polynomials, containerized with Docker and orchestrated using Docker Compose.

## Structure
- `src/main.rs`: Rust application entry point and usage examples
- `src/polynomial.rs`: Multivariate polynomial ring implementation
- `Dockerfile`: Container build instructions
- `docker-compose.yml`: Service orchestration

## Features
- Multivariate polynomial ring over real numbers
	- Addition and multiplication of polynomials in any number of variables
	- Custom variable names and term construction

## Usage

### Build and Run with Docker Compose

```sh
docker-compose up --build
```

### Stopping the Service

```sh
docker-compose down
```

## Example: Multivariate Polynomials

The app demonstrates polynomials in multiple variables. Example from `main.rs`:

```rust
let variables = vec!["x".to_string(), "y".to_string()];
let p = Polynomial::from_terms(
	variables.clone(),
	vec![
		(vec![2,1], 2.0), // 2x^2y
		(vec![1,2], 3.0), // 3xy^2
		(vec![0,0], 1.0), // 1
	]
);
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
```

---

Feel free to extend the polynomial module for more advanced algebraic operations.
