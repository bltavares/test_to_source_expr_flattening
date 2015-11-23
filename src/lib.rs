#![feature(rustc_private)]
#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;

use rustc::plugin::Registry;
use syntax::ast;
use syntax::codemap::{DUMMY_SP, Span};
use syntax::ext::base::{ExtCtxt, MacEager,  MacResult};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::ToTokens;
use syntax::ptr::P;


fn make_expr(cx: &mut ExtCtxt) -> P<ast::Expr> {
   let a = quote_expr!(cx, 1i32+2);
   let b = quote_expr!(cx, 3+4);
   let expr = cx.expr_binary(DUMMY_SP, ast::BiMul, a, b);
   // expr = (1+2)*(3+4);
   return expr;
}

fn expand_test1(cx: &mut ExtCtxt, _sp: Span, _tts: &[ast::TokenTree])
        -> Box<MacResult + 'static> {
    let expr = make_expr(cx);
    // (1+2)*(3+4) = 21
    // this the correct result
    return MacEager::expr(expr);
}

fn expand_test2(cx: &mut ExtCtxt, _sp: Span, _tts: &[ast::TokenTree])
        -> Box<MacResult + 'static> {
   let expr = make_expr(cx);
   // 1+2*3+4 = 1+(2*3)+4 = 11
   // this is the incorrect result
   return MacEager::expr(quote_expr!(cx, $expr));
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("test1", expand_test1);
    reg.register_macro("test2", expand_test2);
}
