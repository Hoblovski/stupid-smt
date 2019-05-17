use std::rc::Rc;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::fmt;
use std::collections::BTreeSet as Set;

#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    True,
    False,
    Var(isize),
    Neg(Rc<Formula>),
    And(Vec<Rc<Formula>>),
    Or(Vec<Rc<Formula>>),
}
use Formula::*;

impl Formula {
    fn simpl_or_comps<T>(it: T) -> Formula
        where T: Iterator<Item=Rc<Formula>>
    {
        let mut comps = Vec::new();
        for v in it {
            match &*v {
                True =>
                    return True,
                False =>
                    continue,
                Var(..) | Neg(..) | And(..) =>
                    comps.push(v),
                Or(v) =>
                    comps.append(&mut v.clone()),
            }
        }
        comps.dedup();
        Or(comps)
    }

    fn simpl_and_comps<T>(it: T) -> Formula
        where T: Iterator<Item=Rc<Formula>>
    {
        let mut comps = Vec::new();
        for v in it {
            match &*v {
                True =>
                    continue,
                False =>
                    return False,
                Var(..) | Neg(..) | Or(..) =>
                    comps.push(v),
                And(v) =>
                    comps.append(&mut v.clone()),
            }
        }
        comps.dedup();
        And(comps)
    }

    /// simplify and demorgan
    pub fn simpl_demorgan(a: Rc<Formula>) -> Rc<Formula> {
        let r = match &*a {
            True | False | Var(..) => return a,
            Neg(v) => match &**v {
                True => False,
                False => True,
                Var(x) => Var(-*x),
                And(v) => Self::simpl_or_comps(v.iter()
                    .map(|x| Self::simpl_demorgan(Rc::new(Neg(x.clone()))))),
                Or(v) => Self::simpl_and_comps(v.iter()
                    .map(|x| Self::simpl_demorgan(Rc::new(Neg(x.clone()))))),
                Neg(v) => return Self::simpl_demorgan(v.clone()),
            },
            And(v) => Self::simpl_and_comps(v.iter()
                .map(|x| Self::simpl_demorgan(x.clone()))),
            Or(v) => Self::simpl_or_comps(v.iter()
                .map(|x| Self::simpl_demorgan(x.clone()))),
        };
        Rc::new(r)
    }
}

pub struct ConjNormalFormula(Set<Set<isize>>);

impl fmt::Display for ConjNormalFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.0.iter() {
            for v in v.iter() { write!(f, "{} ", v)? }
            writeln!(f, "0")?
        }
        Ok(())
    }
}


impl BitAnd for Formula {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut lhs = match self {
            And(v) => v, True => vec![], x => vec![Rc::new(x)],
        };
        let mut rhs = match rhs {
            And(v) => v, True => vec![], x => vec![Rc::new(x)],
        };
        lhs.append(&mut rhs);
        And(lhs)
    }
}

impl BitOr for Formula {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut lhs = match self {
            Or(v) => v, False => vec![], x => vec![Rc::new(x)],
        };
        let mut rhs = match rhs {
            Or(v) => v, False => vec![], x => vec![Rc::new(x)],
        };
        lhs.append(&mut rhs);
        Or(lhs)
    }
}

impl Not for Formula {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            True => False,
            False => True,
            _ => Neg(Rc::new(self))
        }
    }
}

/// Each element ss in sss is a Set<Set<T>>.
/// Select an element s from each ss, join them to get a resulting so.
/// Returns the set of all so.
///
/// TODO: optimize. current implementation is too Haskell.
///
/// flatperm {} = {{}}
/// flatperm ss:sss' = {s + so  |  s <- ss, so <- flatperm sss'}
pub fn flatperm<T: Ord + Copy>(mut sss: Set<Set<Set<T>>>) -> Set<Set<T>> {
    let ss = match sss.iter().next() {
        None => return btreeset!{btreeset!{}},
        Some(ss) => ss.clone()
    };
    sss.remove(&ss);
    let sso = flatperm(sss);
    let mut r = Set::new();
    for s in ss.iter() {
        for so in sso.iter() {
            r.insert(s.union(so).cloned().collect());
        }
    }
    r
}

fn cnf_simpl_inner(f: Set<Set<isize>>) -> Set<Set<isize>> {
    let r = f.into_iter().filter(|c| {
        for l in c.iter() {
            if l < &0 && c.contains(&-l) { return false }
        }
        true
    }).collect();
    r
}

pub fn nnf_to_cnf(a: Rc<Formula>) -> ConjNormalFormula {
    let inner: Set<Set<isize>> = match &*a {
        True =>
            btreeset!{},
        False =>
            btreeset!{btreeset!{}},
        Var(id) =>
            btreeset!{btreeset!{*id}},
        Neg(v) =>
            match &**v {
                Var(id) => btreeset!{btreeset!{-id}},
                _ => panic!("not nnf"),
            },
        And(v) =>
            v.iter().flat_map(|x| nnf_to_cnf(x.clone()).0).collect(),
        Or(v) =>
            flatperm(v.iter().map(|x| nnf_to_cnf(x.clone()).0).collect()),
    };
    ConjNormalFormula(cnf_simpl_inner(inner))
}

