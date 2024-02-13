use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Sym(pub String);

#[derive(Debug, PartialEq, Clone)]
pub struct Deg(pub usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Poly(pub Vec<PolyTerm>);

#[derive(Debug, PartialEq, Clone)]
pub struct PolyTerm {
    pub coeff: f64,
    pub vars: Vec<PolyVar>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PolyVar {
    pub sym: Sym,
    pub deg: Deg,
}

impl Poly {
    pub fn simplify(&mut self) {
        // remove terms with zero coeff
        // add coeffs of like terms
    }

    pub fn substitute(&mut self, sym: &Sym, val: f64) {
        for term in &mut self.0 {
            for var in &mut term.vars {
                if &var.sym == sym {
                    term.coeff *= val.powi(var.deg.0 as i32);
                    var.deg.0 = 0;
                }
            }
        }
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
        match self.deg.0 {
            1 => write!(f, "{}", self.sym.0),
            _ => write!(f, "{}{}", self.sym.0, self.deg.0),
        }
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
                    sym: Sym("x".to_string()),
                    deg: Deg(2),
                }],
            },
            PolyTerm {
                coeff: -2.,
                vars: vec![
                    PolyVar {
                        sym: Sym("x".to_string()),
                        deg: Deg(3),
                    },
                    PolyVar {
                        sym: Sym("y".to_string()),
                        deg: Deg(1),
                    },
                    PolyVar {
                        sym: Sym("z".to_string()),
                        deg: Deg(2),
                    },
                ],
            },
            PolyTerm {
                coeff: 10.,
                vars: vec![PolyVar {
                    sym: Sym("y".to_string()),
                    deg: Deg(2),
                }],
            },
        ]);

        println!("{}", p);
        p.substitute(&Sym("x".to_string()), 2.);
        println!("{}", p);
    }
}
