//! This gets rid of free variables in a type by replacing them by fresh bound
//! ones. We use this when building clauses that contain types passed to
//! `program_clauses`; these may contain variables, and just copying those
//! variables verbatim leads to problems. Instead, we return a slightly more
//! general program clause, with new variables in those places. This can only
//! happen with `dyn Trait` currently; that's the only case where we use the
//! types passed to `program_clauses` in the clauses we generate.

use chalk_engine::fallible::Fallible;
use chalk_ir::{
    fold::{Fold, Folder},
    interner::Interner,
    Binders, BoundVar, DebruijnIndex, Lifetime, LifetimeData, ParameterKind, Ty, TyData,
};
use std::collections::HashMap;

pub struct Generalize<'i, I: Interner> {
    binders: Vec<ParameterKind<()>>,
    mapping: HashMap<BoundVar, usize>,
    interner: &'i I,
}

impl<I: Interner> Generalize<'_, I> {
    pub fn apply<T: Fold<I, I>>(interner: &I, value: &T) -> Binders<T::Result> {
        let mut generalize = Generalize {
            binders: Vec::new(),
            mapping: HashMap::new(),
            interner,
        };
        let value = value
            .fold_with(&mut generalize, DebruijnIndex::INNERMOST)
            .unwrap();
        Binders::new(generalize.binders, value)
    }
}

impl<'i, I: Interner> Folder<'i, I> for Generalize<'i, I> {
    fn as_dyn(&mut self) -> &mut dyn Folder<'i, I> {
        self
    }

    fn fold_free_var_ty(
        &mut self,
        bound_var: BoundVar,
        outer_binder: DebruijnIndex,
    ) -> Fallible<Ty<I>> {
        let binder_vec = &mut self.binders;
        let new_index = self.mapping.entry(bound_var).or_insert_with(|| {
            let i = binder_vec.len();
            binder_vec.push(ParameterKind::Ty(()));
            i
        });
        let new_var = BoundVar::new(outer_binder, *new_index);
        Ok(TyData::BoundVar(new_var).intern(self.interner()))
    }

    fn fold_free_var_lifetime(
        &mut self,
        bound_var: BoundVar,
        outer_binder: DebruijnIndex,
    ) -> Fallible<Lifetime<I>> {
        let binder_vec = &mut self.binders;
        let new_index = self.mapping.entry(bound_var).or_insert_with(|| {
            let i = binder_vec.len();
            binder_vec.push(ParameterKind::Ty(()));
            i
        });
        let new_var = BoundVar::new(outer_binder, *new_index);
        Ok(LifetimeData::BoundVar(new_var).intern(self.interner()))
    }

    fn interner(&self) -> &'i I {
        self.interner
    }

    fn target_interner(&self) -> &'i I {
        self.interner
    }
}
