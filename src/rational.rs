use std::num::NonZeroU64;
use gcd::Gcd;

#[derive(Debug, Copy, Clone, Hash)]
pub struct Rational {
    numerator: i64,
    denominator: NonZeroU64,
    normalized: bool,
}

impl Rational {
    pub const ZERO: Rational = Rational::new_from_int(0);
    
    pub fn new(numerator: i64, denominator: NonZeroU64) -> Self {
        let mut rational = Rational {
            numerator,
            denominator: if numerator != 0 { denominator } else { NonZeroU64::new(1).unwrap() },
            normalized: false,
        };

        rational.normalized = rational.gcd() == 1;

        rational
    }
    
    pub const fn new_from_int(numerator: i64) -> Self {
        Rational {
            numerator,
            denominator: NonZeroU64::new(1).unwrap(),
            normalized: true,
        }
    }
    
    pub fn new_normalize(numerator: i64, denominator: NonZeroU64) -> Rational {
        let mut rat = Rational::new(numerator, denominator);
        rat.normalize();
        rat
    }

    fn gcd(&self) -> u64 {
        self.denominator.get().gcd(self.numerator.unsigned_abs())
    }

    pub fn normalize(&mut self) {
        if self.normalized {
            return;
        }
        
        let gcd = self.gcd();
        self.numerator /= gcd as i64;
        self.denominator = NonZeroU64::new(self.denominator.get() / gcd).unwrap();
        self.normalized = true;
    }

    pub fn into_normal(self) -> Rational {
        let mut same = self;
        same.normalize();
        same
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        let self_normalized = self.into_normal();
        let other_normalized = other.into_normal();
        
        self_normalized.numerator == other_normalized.numerator 
            && self_normalized.denominator == other_normalized.denominator
    }
}

impl Eq for Rational {}

impl std::ops::Add for Rational {
    type Output = Self;

    /// ```
    /// # use std::num::NonZeroU64;
    /// # use horner_solver::rational::Rational;
    /// 
    /// let lhs = Rational::new(2, NonZeroU64::new(3).unwrap());
    /// let rhs = Rational::new(3, NonZeroU64::new(6).unwrap());
    /// 
    /// assert_eq!(Rational::new(7, NonZeroU64::new(6).unwrap()), lhs + rhs)
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        use num::integer;
        
        let lcm = integer::lcm(self.denominator.get(), rhs.denominator.get());
        let numerator = self.numerator * (lcm / self.denominator.get()) as i64 + rhs.numerator * (lcm / rhs.denominator.get()) as i64;
        let denominator = NonZeroU64::new(lcm).unwrap();
        
        Rational {
            numerator,
            denominator,
            normalized: false,
        }.into_normal()
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;
    
    /// ```
    /// # use std::num::NonZeroU64;
    /// # use horner_solver::rational::Rational;
    ///
    /// let lhs = Rational::new(2, NonZeroU64::new(3).unwrap());
    /// let rhs = Rational::new(3, NonZeroU64::new(6).unwrap());
    ///
    /// assert_eq!(Rational::new(1, NonZeroU64::new(3).unwrap()), lhs * rhs)
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Rational {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator.checked_mul(rhs.denominator).unwrap(),
            normalized: false,
        }.into_normal()
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denominator.get() != 1 {
            f.write_fmt(format_args!("{}/{}", self.numerator, self.denominator))
        } else {
            f.write_fmt(format_args!("{}", self.numerator))
        }
    }
}