use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use crate::expr::*;
use crate::logic::*;

type BVSymIdInner = usize;

/// bit-vector variable identifier.
#[derive(Debug)]
pub struct BVSymId<'c> {
    id: BVSymIdInner,
    ctx: PhantomData<&'c Context<'c>>,
}

impl<'c> BVSymId<'c> {
    pub fn new(c: &'c Context<'c>, id: BVSymIdInner) -> BVSymId<'c> {
        BVSymId { id, ctx: PhantomData }
    }
}

#[derive(Debug)]
struct BVSymInfo<'c> {
    width: usize,
    name: &'c str,
}

/// holds variable definitions. all solvers, expressions, syms depend on it.
#[derive(Debug)]
pub struct Context<'c> {
    vars_tbl: RefCell<HashMap<&'c str, BVSymIdInner>>,
    vars_info: RefCell<HashMap<BVSymIdInner, BVSymInfo<'c>>>,
}

impl<'c> Context<'c> {
    pub fn new<'a>() -> Context<'a> {
        Context { vars_tbl: RefCell::new(HashMap::new()), vars_info: RefCell::new(HashMap::new()) }
    }

    pub fn fresh_bvsym(&'c self, name: &'c str, width: usize) -> BVSymId<'c>
    {
        let mut vars_tbl = self.vars_tbl.borrow_mut();
        let mut vars_info = self.vars_info.borrow_mut();
        if vars_tbl.contains_key(name) { panic!("not a fresh variable"); }
        let id = vars_tbl.len();
        vars_tbl.insert(name, id);
        vars_info.insert(id, BVSymInfo { width, name });
        BVSymId::new(self, id)
    }

    pub fn solver(&'c self) -> Solver<'c> {
        let mut sat_solver = minisat::Solver::new();
        let vars_info = self.vars_info.borrow();
        let mut props = HashMap::new();
        for (id, BVSymInfo { width, name }) in vars_info.iter() {
            let mut bits = Vec::with_capacity(*width);
            for _ in 0..*width { bits.push(sat_solver.new_lit()); }
            props.insert(*id, bits);
        }

        Solver { sat_solver, ctx: self, props, clauses: vec![] }
    }
}

/// on top of a context, holds a formulae.
#[derive(Debug)]
pub struct Solver<'c> {
    sat_solver: minisat::Solver,
    ctx: &'c Context<'c>,
    /// props[i][j]: the j-th bit of the symbol with id=i
    props: HashMap<BVSymIdInner, Vec<minisat::Bool>>,
    clauses: Vec<Bool<'c>>
}


/// might outlive the context.
pub struct Model {
    m: HashMap<String, usize>,
}

impl Model {
}

enum CheckResult {
    Sat(Model),
    Unsat,
}

impl<'c> Solver<'c> {
    fn assert(&self, clause: Bool) {

    }

    fn check(self) -> CheckResult {
        panic!("check");
    }

//    fn bv_encode(&self, b: &BVExpr<'c>) -> Vec<Prop> {
//        use BVExpr::*;
//        match b {
//            Const(len, val) => {
//                let mut val = *val;
//                let mut r = Vec::with_capacity(*len);
//                for _ in 0..*len { r.push((val & 1 != 0).into()); }
//                r
//            }, 
//            Add(box a, box b) => {
//                let a = self.bv_encode(a);
//                let b = self.bv_encode(b);
//                assert!(a.len() == b.len());
//                a.into_iter().zip(b.iter()).scan(false.into(),
//                    |carry, (b1, b2)| {
//                        let r = carry 
//                    }
//                ).collect()
//            }, 
//            _ => unimplemented!(), 
//        }
//    }
}
