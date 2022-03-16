use crate::{
    utils::{State},
    Config,
};
use std::{cell::RefCell, rc::Rc};
use swc_common::{
    comments::Comments,
};
use swc_ecmascript::{
    ast::*,
    visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith},
};

pub fn pure<C: Comments>(
    comments: Option<C>, 
    config: Rc<Config>,
    state: Rc<RefCell<State>>,
) -> impl Fold + VisitMut {
    as_folder(Pure {
        comments,
        config,
        state,
    })
}


#[derive(Default)]
struct Pure<C: Comments> {
    comments: Option<C>, 
    config: Rc<Config>,
    state: Rc<RefCell<State>>,
}

impl<C: Comments> VisitMut for Pure<C> {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if !self.config.pure {
            return
        }

        match expr {
            Expr::TaggedTpl(tpl) => {
                if self.state.borrow().is_styled(&tpl.tag) {
                    if let Some(comments) = &self.comments {
                        comments.add_pure_comment(tpl.span.lo);
                    }
                }
            },
            // Expr::Call(call) => {
            //     match call {
            //         CallExpr {
            //             callee: Callee::Expr(callee),
            //             ..
            //         } => {
            //             if self.state.borrow().is_styled(&expr) || 
            //             self.state.borrow().is_styled(&*callee) || 
            //             self.state.borrow().is_pure_helper(callee) {
            //                 if let Some(comments) = &self.comments {
            //                     comments.add_pure_comment(call.span.lo);
            //                 }
            //             }
            //         },
            //         _ => {},
            //     }
            // },
            _ => {},
        }
    }
}
