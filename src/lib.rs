use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Poly(pub Vec<PolyTerm>);

#[derive(Debug, PartialEq, Clone)]
pub struct PolyTerm {
    pub coeff: f64,
    pub vars: Vec<PolyVar>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct PolyVar {
    pub sym: String,
    pub deg: usize,
}

impl Poly {
    pub fn simplify(&mut self) {
        // remove terms with zero coeff
        self.0.retain(|term| term.coeff != 0.);

        // add together coeffs of like terms
        let mut m = HashMap::<Vec<PolyVar>, f64>::new();
        while let Some(term) = self.0.pop() {
            let entry = m.entry(term.vars).or_insert(0.);
            *entry += term.coeff;
        }
        for (vars, coeff) in m.into_iter() {
            self.0.push(PolyTerm { coeff, vars })
        }

        // remove vars with zero degree
        for term in &mut self.0 {
            term.vars.retain(|var| var.deg != 0);
        }

        // sort according to degree desc.
        self.0
            .sort_by_key(|term| term.vars.iter().map(|var| var.deg).sum::<usize>());
        self.0.reverse();
    }

    pub fn substitute(&mut self, sym: &str, val: f64) {
        for term in &mut self.0 {
            for var in &mut term.vars {
                if var.sym == sym {
                    term.coeff *= val.powi(var.deg as i32);
                    var.deg = 0;
                }
            }
        }
    }
}

impl Add for Poly {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.0);
        self
    }
}

impl AddAssign for Poly {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0)
    }
}

impl Neg for Poly {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for term in &mut self.0 {
            term.coeff *= -1.;
        }
        self
    }
}

impl Sub for Poly {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.neg().0);
        self
    }
}

impl SubAssign for Poly {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.neg().0);
    }
}

impl Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let [first, rest @ ..] = self.0.as_slice() {
            write!(f, "{}", first)?;

            for term in rest {
                write!(f, " ")?;
                if term.coeff > 0. {
                    write!(f, "+")?;
                }
                write!(f, "{}", term)?;
            }
        }

        Ok(())
    }
}

impl Display for PolyTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coeff != 1. {
            write!(f, "{}", self.coeff)?;
        }
        for var in &self.vars {
            write!(f, "{}", var)?;
        }
        Ok(())
    }
}

impl Display for PolyVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sym.len() == 1 {
            true => write!(f, "{}", self.sym)?,
            false => write!(f, "({})", self.sym)?,
        }
        if self.deg != 1 {
            write!(f, "{}", self.deg)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute() {
        let mut p = Poly(vec![
            PolyTerm {
                coeff: 1.,
                vars: vec![PolyVar {
                    sym: "x".to_string(),
                    deg: 2,
                }],
            },
            PolyTerm {
                coeff: -2.,
                vars: vec![
                    PolyVar {
                        sym: "x".to_string(),
                        deg: 3,
                    },
                    PolyVar {
                        sym: "yy".to_string(),
                        deg: 1,
                    },
                    PolyVar {
                        sym: "z".to_string(),
                        deg: 2,
                    },
                ],
            },
            PolyTerm {
                coeff: 10.,
                vars: vec![PolyVar {
                    sym: "y".to_string(),
                    deg: 2,
                }],
            },
        ]);

        println!("{}", p);
        p.substitute("x", 2.);
        println!("{}", p);
        p.simplify();
        println!("{}", p);

        let p2 = Poly(vec![]);

        p += p2;
        // let p3 = p + p2;
    }
}
