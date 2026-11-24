use ast::{Argument, Expression, ExpressionId};
use objtree::TypeId;
use prelude::Intrinsic;

use crate::{
    Analyzer,
    error::{SemaError, SemaErrorKind},
    walk::{self, DeclaredType, ProcContext, Visitor},
};

#[derive(Default)]
struct ImplicitIstype {
    rewrites: Vec<(ExpressionId, TypeId)>,
    errors: Vec<SemaError>,
}

impl Visitor for ImplicitIstype {
    fn expression(&mut self, proc: &ProcContext, id: ExpressionId, expression: &Expression) {
        let Expression::Call { callee, args } = expression else {
            return;
        };

        let is_istype = proc
            .called_proc(*callee)
            .is_some_and(|called| called.intrinsic == Some(Intrinsic::Istype));
        if !is_istype {
            return;
        }

        let [
            Argument {
                key: None,
                value: Some(value),
            },
        ] = args.as_slice()
        else {
            return;
        };

        match proc.declared_type(*value) {
            DeclaredType::Declared(ty) => self.rewrites.push((id, ty)),
            DeclaredType::Unknown(name) => self.errors.push(SemaError::new(
                SemaErrorKind::UnknownVarType(name),
                proc.location_of(id),
            )),
        }
    }
}

impl Analyzer<'_> {
    pub(crate) fn resolve_implicit_istype(&mut self) {
        let mut pass = ImplicitIstype::default();
        walk::walk_procs(self.ast, &self.tree, &mut pass);
        self.errors.extend(pass.errors);

        let ast = &mut *self.ast;
        for (call, ty) in pass.rewrites {
            let Some(path) = self.tree.get(ty).map(|decl| decl.path.clone()) else {
                continue;
            };

            let Some(id) = ExpressionId::new(ast.expressions.len()) else {
                break;
            };

            let line = ast.expression_lines.get(call);
            ast.expressions.push(Expression::Path(path));
            ast.expression_lines.push(id, line);
            if let Some(Expression::Call { args, .. }) = ast.expressions.get_mut(call.index())
                && args.len() == 1
            {
                args.push(Argument {
                    key: None,
                    value: Some(id),
                });
            }
        }
    }
}
