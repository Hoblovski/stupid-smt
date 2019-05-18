extern crate stupid_smt;
use stupid_smt::expr::*;
use stupid_smt::expr::Bool::*;
use stupid_smt::expr::BVExpr::*;
use stupid_smt::solver::*;
use stupid_smt::logic::*;
use stupid_smt::logic::Formula::*;
use std::rc::Rc;

#[macro_use] extern crate maplit;

fn main() {
    println!("Hello, world!");

//    let a = || Var(1);
//    let b = || Var(2);
//    let c = || Var(3);
//    let d = || Var(4);
//
//    let t = Rc::new((a() & b() & c()) | (a() & !b()) & !(a() & b())); println!("{:?}", t);
//    let t = Formula::simpl_demorgan(t); println!("{:?}", t);
//    let t = nnf_to_cnf(t); println!("\nCNF:\n{}\n", t);
//
//    let t = Rc::new(!((a()&b())|(c()|d()))); println!("{:?}", t);
//    let t = Formula::simpl_demorgan(t); println!("{:?}", t);
//    let t = nnf_to_cnf(t); println!("\nCNF:\n{}\n", t);
//
//    let t = Rc::new(!!!! (a()|b())); println!("{:?}", t);
//    let t = Formula::simpl_demorgan(t); println!("{:?}", t);
//    let t = nnf_to_cnf(t); println!("\nCNF:\n{}\n", t);


    let mut ctx = Context::new();
    let t1 = bvconst(3, 4);
    let t2 = bvconst(1, 4);
    let t3 = bvsym(&ctx, "a", 4);
    let t4 = Add(&t1, &t2);
    let s = ctx.solver();
    println!("{:#?}", s);
    let constraint = BVEq(&t1, &t3);
    s.assert(&constraint);
    // comment the two
    let constraint = BVEq(&t2, &t3);
    s.assert(&constraint);
    println!("{:?}", s.check());
    // TODO: forbid adding symbols to ctx after creation of solver
}
