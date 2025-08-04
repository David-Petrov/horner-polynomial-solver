use std::collections::HashSet;
use std::num::NonZeroU64;
use crate::rational::Rational;

#[derive(Debug)]
pub struct Polynomial {
    coefficients: Vec<i64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<i64>) -> Self {
        let coefficients = if coefficients.is_empty() {
            vec![0]
        } else { 
            coefficients 
        };
        
        Polynomial { coefficients }
    }
    
    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }
    
    ///```
    /// # use horner_solver::polynomial::Polynomial;
    /// # use horner_solver::rational::Rational;
    /// let polynomial = Polynomial::new(vec![6, -7, 0, 1]);
    /// 
    /// assert_eq!(Rational::ZERO, polynomial.eval(Rational::new_from_int(1)))
    /// ```
    pub fn eval(&self, x: Rational) -> Rational {
        let mut result: Rational = Rational::new_from_int(0);
        
        for coeff in self.coefficients.iter().rev() {
            result = result * x + Rational::new_from_int(coeff.clone());
        }

        result
    }

    ///```
    /// # use std::collections::HashSet;
    /// # use horner_solver::polynomial::Polynomial;
    /// # use horner_solver::rational::Rational;
    /// let polynomial = Polynomial::new(vec![6, -7, 0, 1]);
    /// 
    /// let expected_roots = HashSet::from([Rational::new_from_int(1), Rational::new_from_int(2), Rational::new_from_int(-3)]);
    /// 
    /// assert_eq!(expected_roots, polynomial.find_rational_roots())
    /// ```
    pub fn find_rational_roots(&self) -> HashSet<Rational> {
        let mut candidates: HashSet<Rational> = HashSet::new();
        
        for p in divisors(self.coefficients.first().unwrap().unsigned_abs()).iter() {
            for q in divisors(self.coefficients.last().unwrap().unsigned_abs()).iter() {
                let denominator = NonZeroU64::new(*q).unwrap();
                candidates.insert(Rational::new_normalize(*p as i64, denominator));
                candidates.insert(Rational::new_normalize(-(*p as i64), denominator));
            }
        }
        
        candidates.into_iter()
            .filter(|x| 
                self.eval(*x) == Rational::ZERO)
            .collect()
    }
}

fn divisors(n: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = vec![];
    
    for i in 1..=n {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    
    divisors
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.coefficients[0]))?;
        
        for i in 1..self.coefficients.len() {
            if self.coefficients[i] == 0 {
                continue;
            }

            f.write_fmt(format_args!(
                " {} {}x^{}", 
                    if self.coefficients[i] < 0 { "-" } else { "+" }, 
                    {
                        let abs = self.coefficients[i].abs();
                        if abs == 1 { String::new() } else { format!("{}", abs) }
                    },
                    i))?;
        }
        
        Ok(())
    }
}