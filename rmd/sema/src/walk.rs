use ast::{
    AST, AccessKind, Builtin, Declaration, Expression, ExpressionId, ForLoop, LoopBinding, Statement, SwitchValue,
    VarSpec,
};
use base::{
    location::{Location, SourceLine},
    path::TreePath,
    types::Identifier,
};
use objtree::{ObjectTree, ProcDecl, TypeId};
use rustc_hash::FxHashMap;

pub(crate) trait Visitor {
    fn expression(&mut self, proc: &ProcContext, id: ExpressionId, expression: &Expression);
}

pub(crate) enum DeclaredType {
    Declared(TypeId),
    Unknown(Option<Identifier>),
}

pub(crate) struct ProcContext<'a> {
    pub(crate) ast: &'a AST,
    tree: &'a ObjectTree,
    owner: TypeId,
    location: Location,
    scopes: Vec<FxHashMap<Identifier, Option<TreePath>>>,
}

impl ProcContext<'_> {
    pub(crate) fn location_of(&self, expression: ExpressionId) -> Location {
        self.ast
            .expression_lines
            .get(expression)
            .map_or(self.location, SourceLine::location)
    }

    pub(crate) fn called_proc(&self, callee: ExpressionId) -> Option<&ProcDecl> {
        let Some(Expression::Identifier(name)) = self.ast.get_expr(callee) else {
            return None;
        };

        self.tree.proc_inherited(self.owner, name)
    }

    pub(crate) fn declared_type(&self, expression: ExpressionId) -> DeclaredType {
        match self.ast.get_expr(expression) {
            Some(Expression::Identifier(name)) => {
                let local = self.scopes.iter().rev().find_map(|scope| scope.get(name));
                match local {
                    Some(declared) => self.declared_as(name, declared.as_ref()),
                    None => match self.tree.var_inherited(self.owner, name) {
                        Some(var) => self.declared_as(name, var.declared_type.as_ref()),
                        None => DeclaredType::Unknown(Some(name.clone())),
                    },
                }
            },
            Some(Expression::Builtin(Builtin::Src)) if self.owner != TypeId::ROOT => DeclaredType::Declared(self.owner),
            Some(Expression::Builtin(Builtin::Usr)) => self
                .tree
                .roots()
                .mob
                .map_or(DeclaredType::Unknown(None), DeclaredType::Declared),
            Some(Expression::Field { object, name, access }) if *access != AccessKind::Scope => {
                let Some(owner) = self.type_of(*object) else {
                    return DeclaredType::Unknown(Some(name.clone()));
                };

                match self.tree.var_inherited(owner, name) {
                    Some(var) => self.declared_as(name, var.declared_type.as_ref()),
                    None => DeclaredType::Unknown(Some(name.clone())),
                }
            },
            Some(Expression::Grouped(inner)) => self.declared_type(*inner),
            _ => DeclaredType::Unknown(None),
        }
    }

    fn type_of(&self, expression: ExpressionId) -> Option<TypeId> {
        if matches!(
            self.ast.get_expr(expression),
            Some(Expression::Builtin(Builtin::Global))
        ) {
            return Some(TypeId::ROOT);
        }

        match self.declared_type(expression) {
            DeclaredType::Declared(ty) => Some(ty),
            DeclaredType::Unknown(_) => None,
        }
    }

    fn declared_as(&self, name: &Identifier, declared: Option<&TreePath>) -> DeclaredType {
        match declared.and_then(|path| self.resolve_declared(path)) {
            Some(ty) => DeclaredType::Declared(ty),
            None => DeclaredType::Unknown(Some(name.clone())),
        }
    }

    /// `var/list/mob/L` declares a list; the rest of the path only hints at its contents
    fn resolve_declared(&self, path: &TreePath) -> Option<TypeId> {
        if let Some(ty) = self.tree.id_of(path) {
            return Some(ty);
        }

        let root = self
            .tree
            .id_of(&TreePath::new(vec![path.segments.first()?.clone()], true))?;
        let roots = self.tree.roots();
        let is_list = roots.list == Some(root) || roots.alist == Some(root);

        is_list.then_some(root)
    }
}

pub(crate) fn walk_procs(ast: &AST, tree: &ObjectTree, visitor: &mut impl Visitor) {
    let mut walker = Walker {
        proc: ProcContext {
            ast,
            tree,
            owner: TypeId::ROOT,
            location: Location::default(),
            scopes: Vec::new(),
        },
        visitor,
        pending: Vec::new(),
    };
    for declaration in &ast.declarations {
        walker.declaration(declaration, &TreePath::default());
    }
}

struct Walker<'a, 'v, V> {
    proc: ProcContext<'a>,
    visitor: &'v mut V,
    pending: Vec<ExpressionId>,
}

impl<V: Visitor> Walker<'_, '_, V> {
    fn declaration(&mut self, declaration: &Declaration, prefix: &TreePath) {
        match declaration {
            Declaration::Type { path, body, .. } => {
                let full = prefix.concat(path);
                for child in body {
                    self.declaration(child, &full);
                }
            },
            Declaration::Proc {
                path,
                params,
                body: Some(body),
                location,
                ..
            } => {
                let owner = prefix.concat(&TreePath::new(path.declaration_owner().to_vec(), path.absolute));
                let Some(id) = self.proc.tree.id_of(&owner) else {
                    return;
                };

                self.proc.owner = id;
                self.proc.location = *location;
                self.proc.scopes.clear();
                self.proc.scopes.push(FxHashMap::default());
                for param in params {
                    self.expressions(param.default.into_iter().chain(param.in_list));
                    self.declare(&param.spec);
                }

                self.statements(body);
            },
            Declaration::Proc { body: None, .. } | Declaration::Var { .. } | Declaration::Override { .. } => {},
        }
    }

    fn statements(&mut self, body: &[Statement]) {
        self.proc.scopes.push(FxHashMap::default());
        for statement in body {
            self.statement(statement);
        }

        self.proc.scopes.pop();
    }

    fn statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Empty | Statement::Break(_) | Statement::Continue(_) | Statement::Goto(_) => {},
            Statement::Expression(expression) | Statement::Throw(expression) | Statement::Del(expression) => {
                self.expressions([*expression]);
            },
            Statement::Setting { value, .. } => self.expressions([*value]),
            Statement::Return(value) => self.expressions(*value),
            Statement::Output { target, value } => self.expressions([*target, *value]),
            Statement::Input { source, destination } => self.expressions([*source, *destination]),
            Statement::Var {
                spec,
                initializer,
                in_list,
            } => {
                self.expressions(initializer.iter().chain(in_list).copied());
                self.declare(spec);
            },
            Statement::If { branches, else_branch } => {
                for (condition, body) in branches {
                    self.expressions([*condition]);
                    self.statements(body);
                }

                if let Some(body) = else_branch {
                    self.statements(body);
                }
            },
            Statement::While { condition, body } => {
                self.expressions([*condition]);
                self.statements(body);
            },
            Statement::DoWhile { body, condition } => {
                self.statements(body);
                self.expressions([*condition]);
            },
            Statement::For(header) => {
                self.proc.scopes.push(FxHashMap::default());
                self.for_loop(header);
                self.proc.scopes.pop();
            },
            Statement::Switch { value, cases, default } => {
                self.expressions([*value]);
                for case in cases {
                    for value in &case.values {
                        match value {
                            SwitchValue::Value(value) => self.expressions([*value]),
                            SwitchValue::Range(start, end) => self.expressions([*start, *end]),
                        }
                    }

                    self.statements(&case.body);
                }

                if let Some(body) = default {
                    self.statements(body);
                }
            },
            Statement::Spawn { delay, body } => {
                self.expressions(*delay);
                self.statements(body);
            },
            Statement::TryCatch {
                try_body,
                catch_param,
                catch_body,
            } => {
                self.statements(try_body);
                self.proc.scopes.push(FxHashMap::default());
                if let Some(spec) = catch_param {
                    self.declare(spec);
                }

                self.statements(catch_body);
                self.proc.scopes.pop();
            },
            Statement::Label { body, .. } => self.statements(body),
        }
    }

    fn for_loop(&mut self, header: &ForLoop) {
        match header {
            ForLoop::Standard {
                init,
                condition,
                step,
                body,
            } => {
                if let Some(init) = init {
                    self.statement(init);
                }

                self.expressions(*condition);
                if let Some(step) = step {
                    self.statement(step);
                }

                self.statements(body);
            },
            ForLoop::List { key, value, list, body } => {
                self.expressions(*list);
                for binding in key.iter().chain([value]) {
                    self.binding(binding);
                }

                self.statements(body);
            },
            ForLoop::Range {
                variable,
                start,
                end,
                step,
                body,
            } => {
                self.expressions([*start, *end].into_iter().chain(*step));
                self.binding(variable);
                self.statements(body);
            },
        }
    }

    fn binding(&mut self, binding: &LoopBinding) {
        match binding {
            LoopBinding::Declare(spec) => self.declare(spec),
            LoopBinding::Assign { target, .. } => self.expressions([*target]),
        }
    }

    fn declare(&mut self, spec: &VarSpec) {
        self.expressions(spec.dimensions.iter().flatten().copied());
        if let Some(scope) = self.proc.scopes.last_mut() {
            scope.insert(spec.name.clone(), spec.var_type.clone());
        }
    }

    fn expressions(&mut self, roots: impl IntoIterator<Item = ExpressionId>) {
        let ast = self.proc.ast;
        self.pending.extend(roots);
        while let Some(id) = self.pending.pop() {
            let Some(expression) = ast.get_expr(id) else {
                continue;
            };

            self.visitor.expression(&self.proc, id, expression);
            expression.for_each_child(|child| self.pending.push(child));
        }
    }
}
