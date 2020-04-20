//! Tests targeting auto traits specifically

use super::*;

#[test]
fn auto_semantics() {
    test! {
        program {
            #[lang(sized)] trait Sized { }
            #[auto] trait Send { }

            struct TypeA { }

            struct Ptr<T> { }
            impl<T> Send for Ptr<T> where T: Send { }

            struct List<T> where T: Sized {
                data: T,
                next: Ptr<List<T>>
            }
        }

        goal {
            forall<T> {
                List<T>: Send
            }
        } yields {
            "No possible solution"
        }
        goal {
            forall<T> {
                if (T: Send) {
                    List<T>: Send
                }
            }
        } yields {
            "Unique"
        }

        goal {
            List<TypeA>: Send
        } yields {
            "Unique"
        }

        goal {
            exists<T> {
                T: Send
            }
        } yields {
            "Ambiguous"
        }
    }
}

#[test]
fn auto_trait_without_impls() {
    test! {
        program {
            #[auto] trait Send { }

            struct TypeA { }

            struct Useless<T> { }

            struct Data<T> {
                data: T
            }
        }

        goal {
            TypeA: Send
        } yields {
            "Unique"
        }

        // No fields so `Useless<T>` is `Send`.
        goal {
            forall<T> {
                Useless<T>: Send
            }
        } yields {
            "Unique"
        }

        goal {
            forall<T> {
                if (T: Send) {
                    Data<T>: Send
                }
            }
        } yields {
            "Unique"
        }
    }
}

#[test]
fn auto_trait_with_impls() {
    test! {
        program {
            #[auto] trait Send { }

            struct TypeA { }
            struct TypeB { }
            struct Vec<T> { }

            impl<T> Send for Vec<T> where T: Send { }
            impl !Send for TypeA { }
        }

        goal {
            TypeA: Send
        } yields {
            "No possible solution"
        }

        goal {
            TypeB: Send
        } yields {
            "Unique"
        }

        goal {
            Vec<TypeA>: Send
        } yields {
            "No possible solution"
        }

        goal {
            Vec<TypeB>: Send
        } yields {
            "Unique"
        }

        goal {
            forall<T> {
                Vec<T>: Send
            }
        } yields {
            "No possible solution"
        }
    }
}

/// This Flounders because auto traits can't be enumerated
#[test]
fn auto_traits_flounder() {
    test! {
        program {
            struct Foo { }
            struct Bar { }

            #[auto]
            trait Send { }
        }

        goal {
            exists<A> { A: Send }
        } yields_first[SolverChoice::slg(3, None)] {
            "Floundered"
        }
    }
}
