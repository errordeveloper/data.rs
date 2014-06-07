#![crate_id = "thrustmacro"]
#![crate_type = "dylib"]
#![feature(macro_rules, macro_registrar, managed_boxes, quote)]
#![allow(unused_variable, unused_imports)]

//! Thrust Macros! Yay!

extern crate thrust;
extern crate syntax;
extern crate debug;

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
use syntax::parse::parser::Parser;

pub struct ThriftParser<'a> {
    cx: &'a ExtCtxt<'a>,
    span: codemap::Span,
    parser: Parser<'a>,
    tree: &'a [ast::TokenTree]
}

impl<'a> ThriftParser<'a> {

    pub fn new(cx: &'a ExtCtxt, span: codemap::Span, tree: &'a [ast::TokenTree]) -> ThriftParser<'a> {
        ThriftParser {
            cx: cx,
            span: span,
            parser: parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), Vec::from_slice(tree)),
            tree: tree
        }
    }

    pub fn to_expr(&self) -> @ast::Expr {
        let v = self.tree.iter()
            .filter_map(|tt| self.tt_to_expr(tt).map(|e| self.cx.stmt_expr(e)))
            .collect();
        let block = self.cx.block(self.span, v, None);
        self.cx.expr_block(block)
    }

    fn tt_to_expr(&self, tt: &ast::TokenTree) -> Option<@ast::Expr> {
        match *tt {
            ast::TTTok(span, ref tok) => self.token_to_expr(tok),
            _ => { None }
        }
    }

    fn token_to_expr(&self, tok: &token::Token) -> Option<@ast::Expr> {

        match *tok {
            token::IDENT(iden, _) => {
                println!("{}",iden);
            },
            _ => {}
        }

        None
    }

}

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
    let expander = box BasicMacroExpander { expander: native, span: None };
    register(token::intern("thrust"), NormalTT(expander, None));
}

#[allow(experimental)]
fn native(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let parser = ThriftParser::new(cx, sp, tts);

    let code = parser.to_expr();

    MacExpr::new(quote_expr!(parser.cx, {
        ::thrust::Thrust::new()
    }))
}
