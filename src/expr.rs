use std::rc::Rc;
use crate::solver::*;

#[derive(Debug)]
pub enum BVExpr<'c> {
    Symb(Vec<isize>),
    Const(Vec<bool>),
    Add(&'c BVExpr<'c>, &'c BVExpr<'c>),
}

use BVExpr::*;

pub fn bvsym<'c>(c: &'c Context<'c>, name: &'c str, width: isize) -> BVExpr<'c> {
    Symb(c.fresh_bvsym(name, width))
}

pub fn bvconst<'c>(mut val: usize, width: isize) -> BVExpr<'c> {
    assert!(width > 0);
    let mut r = Vec::with_capacity(width as usize);
    for _ in 0..width {
        r.push((val & 1) == 1);
        val >>= 1;
    }
    Const(r)
}


#[derive(Debug)]
pub enum Bool<'c> {
    BVEq(&'c BVExpr<'c>, &'c BVExpr<'c>),
}

