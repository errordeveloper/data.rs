#![crate_id = "thrustmacro"]
#![crate_type = "dylib"]
#![feature(macro_rules, macro_registrar, managed_boxes, quote)]

//! Thrust Macros! Yay!

extern crate syntax;

use syntax::ast;
use syntax::codemap;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{
    SyntaxExtension, ExtCtxt, MacResult, MacExpr, DummyResult,
    NormalTT, BasicMacroExpander,
};
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
    let expander = box BasicMacroExpander { expander: native, span: None };
    register(token::intern("thrust"), NormalTT(expander, None))
}

#[allow(experimental)]
fn native(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
  MacExpr::new(quote_expr!(cx, {}))
}
