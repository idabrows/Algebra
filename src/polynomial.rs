use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Polynomial {
    pub variables: Vec<String>, // variable names, e.g. ["x", "y", "z"]
    pub terms: HashMap<Vec<usize>, f64>, // key: exponent vector, value: coefficient
}

impl Polynomial {
    pub fn new(variables: Vec<String>, terms: HashMap<Vec<usize>, f64>) -> Self {
        // Remove zero terms
        let terms = terms.into_iter().filter(|(_, &c)| c != 0.0).collect();
        Polynomial { variables, terms }
    }

    // Helper to create from list of (exponents, coeff)
    pub fn from_terms(variables: Vec<String>, term_list: Vec<(Vec<usize>, f64)>) -> Self {
        let mut terms = HashMap::new();
        for (exp, coeff) in term_list {
            if coeff != 0.0 {
                terms.insert(exp, coeff);
            }
        }
        Polynomial::new(variables, terms)
    }

    pub fn add(&self, other: &Polynomial) -> Polynomial {
        assert_eq!(self.variables, other.variables, "Variable lists must match");
        let mut result = self.terms.clone();
        for (exp, coeff) in &other.terms {
            *result.entry(exp.clone()).or_insert(0.0) += coeff;
        }
        Polynomial::new(self.variables.clone(), result)
    }

    pub fn mul(&self, other: &Polynomial) -> Polynomial {
        assert_eq!(self.variables, other.variables, "Variable lists must match");
        let mut result = HashMap::new();
        for (exp1, coeff1) in &self.terms {
            for (exp2, coeff2) in &other.terms {
                let exp = exp1.iter().zip(exp2.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<_>>();
                *result.entry(exp).or_insert(0.0) += coeff1 * coeff2;
            }
        }
        Polynomial::new(self.variables.clone(), result)
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut terms = vec![];
        for (exp, &c) in &self.terms {
            let mut term = format!("{c}");
            for (i, &e) in exp.iter().enumerate() {
                if e > 0 {
                    if e == 1 {
                        term += &format!("{}", self.variables[i]);
                    } else {
                        term += &format!("{}^{}", self.variables[i], e);
                    }
                }
            }
            terms.push(term);
        }
        if terms.is_empty() {
            write!(f, "0")
        } else {
            write!(f, "{}", terms.join(" + "))
        }
    }
}
