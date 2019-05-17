use std::rc::Rc;
use crate::solver::*;

#[derive(Debug)]
pub enum BVExpr<'c> {
    Const(usize, usize), // currently only bitvectors <= 128 bits
    Concat(Box<BVExpr<'c>>, Box<BVExpr<'c>>), 
    Slice(Box<BVExpr<'c>>, usize, usize), 
    Add(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
    Sub(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
    Mul(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
    Rem(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
    Symb(Box<BVSymId<'c>>),
    ITE(Box<Bool<'c>>, Box<BVExpr<'c>>, Box<BVExpr<'c>>),
}

use BVExpr::*;
pub fn bvconst<'c>(len: usize, val: usize) -> BVExpr<'c>
{ Const(len, val) }

pub fn concat<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> BVExpr<'c>
{ Concat(Box::new(a), Box::new(b)) }

pub fn slice<'c>(a: BVExpr<'c>, low: usize, high: usize) -> BVExpr<'c>
{ Slice(Box::new(a), low, high) }

pub fn bvadd<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> BVExpr<'c>
{ Add(Box::new(a), Box::new(b)) }

pub fn bvsub<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> BVExpr<'c>
{ Sub(Box::new(a), Box::new(b)) }

pub fn bvmul<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> BVExpr<'c>
{ Mul(Box::new(a), Box::new(b)) }

pub fn bvrem<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> BVExpr<'c>
{ Rem(Box::new(a), Box::new(b)) }

pub fn bvite<'c>(cond: Bool<'c>, t: BVExpr<'c>, f: BVExpr<'c>) -> BVExpr<'c>
{ ITE(Box::new(cond), Box::new(t), Box::new(f)) }

pub fn bvsym<'c>(c: &'c Context<'c>, name: &'c str, width: usize) -> BVExpr<'c>
{ Symb(Box::new(c.fresh_bvsym(name, width))) }


#[derive(Debug)]
pub enum Bool<'c> {
    And(Box<Bool<'c>>, Box<Bool<'c>>),
    Or(Box<Bool<'c>>, Box<Bool<'c>>),
    Neg(Box<Bool<'c>>),
    BVEq(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
    BVLt(Box<BVExpr<'c>>, Box<BVExpr<'c>>),
}

use Bool::*;
pub fn band<'c>(a: Bool<'c>, b: Bool<'c>) -> Bool<'c>
{ And(Box::new(a), Box::new(b)) }

pub fn bor<'c>(a: Bool<'c>, b: Bool<'c>) -> Bool<'c>
{ Or(Box::new(a), Box::new(b)) }

pub fn bneg<'c>(a: Bool<'c>, b: Bool<'c>) -> Bool<'c>
{ Neg(Box::new(a)) }

pub fn bveq<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> Bool<'c>
{ BVEq(Box::new(a), Box::new(b)) }

pub fn bvlt<'c>(a: BVExpr<'c>, b: BVExpr<'c>) -> Bool<'c>
{ BVLt(Box::new(a), Box::new(b)) }
