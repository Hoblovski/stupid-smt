use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use crate::expr::*;
use crate::logic::*;
use crate::logic::Formula as fmla;

#[derive(Debug)]
struct BVSymInfo<'c> {
    width: isize,
    name: &'c str,
    low: isize,
    high: isize, // low..high are the isize's for Formula
}

/// holds variable definitions. all solvers, expressions, syms depend on it.
#[derive(Debug)]
pub struct Context<'c> {
    vars_tbl: RefCell<HashMap<&'c str, isize>>,
    vars_info: RefCell<HashMap<isize, BVSymInfo<'c>>>,
    var_cnt: RefCell<isize>,
}

impl<'c> Context<'c> {
    pub fn new<'a>() -> Context<'a> {
        Context {
            vars_tbl: RefCell::new(HashMap::new()),
            vars_info: RefCell::new(HashMap::new()),
            var_cnt: RefCell::new(1),
        }
    }

    pub fn fresh_bvsym(&'c self, name: &'c str, width: isize) -> Vec<isize>
    {
        let mut vars_tbl = self.vars_tbl.borrow_mut();
        let mut vars_info = self.vars_info.borrow_mut();
        let mut var_cnt = self.var_cnt.borrow_mut();
        if vars_tbl.contains_key(name) { panic!("not a fresh variable"); }
        let id = vars_tbl.len() as isize;
        let low = *var_cnt;
        let high = *var_cnt + width;
        vars_tbl.insert(name, id);
        vars_info.insert(id, BVSymInfo { width, name, low, high });
        let r = (low..high).collect();
        *var_cnt += width;
        r
    }

    pub fn solver(&'c self) -> Solver<'c> {
        let mut sat_solver = minisat::Solver::new();
        let mut props = HashMap::new();
        let var_cnt: isize = *self.var_cnt.borrow();
        for i in 1..var_cnt {
            props.insert(i, sat_solver.new_lit());
        }
        Solver {
            sat_solver: RefCell::new(sat_solver),
            ctx: self,
            props,
            constraints: RefCell::new(Rc::new(fmla::True)),
        }
    }
}

/// on top of a context, holds a formulae.
#[derive(Debug)]
pub struct Solver<'c> {
    sat_solver: RefCell<minisat::Solver>,
    ctx: &'c Context<'c>,
    /// props[i]
    props: HashMap<isize, minisat::Bool>,
    constraints: RefCell<Rc<Formula>>,
}


/// might outlive the context.
pub struct Model {
    m: HashMap<String, isize>,
}

impl Model {
}

#[derive(Debug)]
pub enum CheckResult {
    Sat, //(Model),
    Unsat,
}

impl<'c> Solver<'c> {
    pub fn assert(&'c self, constraint: &'c Bool<'c>) {
        let f = self.bool_encode(constraint);
        let mut constraints = self.constraints.borrow_mut();
        *constraints = fmla::and(&f, &*constraints);
    }

    pub fn check(&'c self) -> CheckResult {
        let mut constraints = self.constraints.borrow_mut();
        *constraints = fmla::simpl_demorgan(constraints.clone());
        let cnf = nnf_to_cnf(constraints.clone());
        let mut sat_solver = self.sat_solver.borrow_mut();
        for clause in cnf.get_inner() {
            sat_solver.add_clause(
                clause.iter().map(|x|
                    if x > &0 { self.props[x] } else { !self.props[&-*x] }));
        }
        match sat_solver.solve() {
            Ok(m) => CheckResult::Sat,
            Err(()) => CheckResult::Unsat
        }
    }

    fn bv_encode(&self, b: &BVExpr<'c>) -> Vec<Rc<Formula>> {
        use BVExpr::*;
        match b {
            Symb(v) => v.iter()
                .map(|&x| Rc::new(fmla::Var(x))).collect(),
            Const(v) => v.iter()
                .map(|&x| Rc::new(if x { fmla::True } else { fmla::False }))
                .collect(),
            _ => unimplemented!(),
        }
    }

    fn bool_encode(&self, b: &Bool<'c>) -> Rc<Formula> {
        use Bool::*;
        match b {
            BVEq(a, b) => {
                let a = self.bv_encode(a);
                let b = self.bv_encode(b);
                a.iter().zip(b.iter()).fold(Rc::new(fmla::True),
                    |st, (b1, b2)| { fmla::and(&st, &fmla::iff(b1, b2)) }
                )
            },
            _ => unimplemented!()
        }
    }
}
