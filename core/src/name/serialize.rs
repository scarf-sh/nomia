use super::*;
use alloc::alloc::AllocError;
use alloc::alloc::Allocator;
use alloc::collections::TryReserveError;
use core::str::Utf8Error;
use cstr_core::CString;
use cstr_core::NulError;

pub enum ParseError {
    NulInInput(NulError),
    BnfcParseError,
    // TODO Add location info to these, or just wait until we have a rust bnfc backend
    VecAllocationError(TryReserveError),
    BoxAllocationError(AllocError),
    StringNotUtf8(Utf8Error),
}

impl From<NulError> for ParseError {
    fn from(e: NulError) -> Self {
        Self::NulInInput(e)
    }
}

impl From<TryReserveError> for ParseError {
    fn from(e: TryReserveError) -> Self {
        Self::VecAllocationError(e)
    }
}

impl From<AllocError> for ParseError {
    fn from(e: AllocError) -> Self {
        Self::BoxAllocationError(e)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(e: Utf8Error) -> Self {
        Self::StringNotUtf8(e)
    }
}

pub trait NameSerializeExt<A: Allocator>: Sized {
    fn parse_in(input: &str, alloc: &A) -> Result<Self, ParseError>;
}

pub trait NameSerializeExtGlobal: Sized {
    fn parse(input: &str) -> Result<Self, ParseError>;
}

impl<A: Allocator + Clone> NameSerializeExt<A> for OwnedName<A> {
    fn parse_in(input: &str, alloc: &A) -> Result<Self, ParseError> {
        // TODO once https://github.com/rust-lang/rust/pull/79500 is fixed use alloc
        let c_input = CString::new(input)?;
        // unsafe: Need to guarantee input is a valid null terminated string, validated by the cstring impl
        let c_parsed = unsafe { bnfc::psName(c_input.as_ptr()) };
        if c_parsed.is_null() {
            Err(ParseError::BnfcParseError)
        } else {
            // Unsafe: Need to ensure pointer is non-null and comes from bnfc
            let res = unsafe { c_parsed.to_rust(alloc) };
            unsafe { bnfc::freeName(c_parsed) };
            res
        }
    }
}

impl NameSerializeExtGlobal for OwnedName {
    fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_in(input, &Global::default())
    }
}

mod bnfc {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    use super::super::*;
    use super::*;
    use alloc::alloc::Allocator;
    use alloc::string::String as RustString;
    use alloc::vec::Vec;
    use cstr_core::CStr;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl Name_ {
        // self must be a valid pointer from bnfc
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<OwnedName<A>, ParseError> {
            let c_listdecl = (*self).u.letin_.listdeclaration_;
            let c_terminal_substitution = (*self).u.letin_.substitution_;

            Ok(OwnedName {
                let_declarations: c_listdecl.to_rust(alloc)?,
                terminal_substitution: c_terminal_substitution.to_rust(alloc)?,
            })
        }
    }

    impl ListDeclaration_ {
        unsafe fn size(self: *const Self) -> usize {
            let mut res = 0;
            let mut cur = self;
            loop {
                if cur.is_null() {
                    return res;
                } else {
                    res += 1;
                    cur = (*cur).listdeclaration_;
                }
            }
        }
        // self must be a valid pointer from bnfc
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<<OwnedNameParameters<A> as NameParameters>::Declarations, ParseError> {
            let sz = self.size();
            let mut res = Vec::new_in(alloc.clone());
            res.try_reserve_exact(sz)?;
            let mut cur = self;
            for _ in 0..sz {
                let curref = *cur;
                res.push(Box::try_new_in(
                    curref.declaration_.to_rust(alloc)?,
                    alloc.clone(),
                )?);
                cur = curref.listdeclaration_;
            }
            Ok(res)
        }
    }

    impl Substitution_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<super::super::Substitution<OwnedNameParameters<A>>, ParseError> {
            let c_nameref = (*self).u.substitute_.nameref_;
            let c_inputs = (*self).u.substitute_.inputs_;
            Ok(super::super::Substitution {
                name: c_nameref.to_rust(alloc)?,
                inputs: c_inputs.to_rust(alloc)?,
            })
        }
    }

    impl Declaration_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<super::super::Declaration<OwnedNameParameters<A>>, ParseError> {
            let c_binding = (*self).u.declare_.binding_;
            let c_substitution = (*self).u.declare_.substitution_;
            Ok(super::super::Declaration {
                var: c_binding.to_rust(alloc)?,
                val: c_substitution.to_rust(alloc)?,
            })
        }
    }

    impl NameRef_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<super::super::NameRef<OwnedNameParameters<A>>, ParseError> {
            match (*self).kind {
                NameRef__is_AtomicRef => {
                    let c_namespace_id =
                        (*(*self).u.atomicref_.atomicname_).u.atomic_.qualification_;
                    let c_name_id = (*(*self).u.atomicref_.atomicname_)
                        .u
                        .atomic_
                        .parameterizedid_;
                    Ok(super::super::NameRef::Atomic {
                        namespace_id: c_namespace_id.to_rust(alloc)?,
                        name_id: c_name_id.to_rust(alloc)?,
                    })
                }
                NameRef__is_Variable => {
                    let ident = (*self).u.variable_.identifier_;
                    Ok(super::super::NameRef::Variable(super::super::Identifier(
                        RustString::from(CStr::from_ptr(ident).to_str()?),
                    )))
                }
                NameRef__is_Resolved => {
                    let ident = (*self).u.resolved_.identifier_;
                    Ok(super::super::NameRef::Resolved(super::super::Identifier(
                        RustString::from(CStr::from_ptr(ident).to_str()?),
                    )))
                }
                NameRef__is_NestedNameRef => {
                    let nested = (*self).u.nestednameref_.name_;
                    Ok(super::super::NameRef::Nested(Box::try_new_in(
                        nested.to_rust(alloc)?,
                        alloc.clone(),
                    )?))
                }
                _ => unreachable!(),
            }
        }
    }

    impl Inputs_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<Vec<super::super::SubstitutionSpec<OwnedNameParameters<A>>, A>, ParseError>
        {
            match (*self).kind {
                Inputs__is_Nullary => Ok(Vec::new_in(alloc.clone())),
                Inputs__is_Multiary => {
                    let c_graphindex = (*self).u.multiary_.graphindex_;
                    let c_substitutionspecs = (*self).u.multiary_.listsubstitutionspec_;

                    if (*c_graphindex).kind != GraphIndex__is_DefaultIndex {
                        unimplemented!()
                    }

                    Ok(c_substitutionspecs.to_rust(alloc)?)
                }
                _ => unreachable!(),
            }
        }
    }

    impl Binding_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            _alloc: &A, // Will use alloc once String has from_in
        ) -> Result<Option<super::super::Identifier<OwnedNameParameters<A>>>, ParseError> {
            match (*self).kind {
                Binding__is_Unbound => Ok(None),
                Binding__is_Bound => {
                    let c_identifier = (*self).u.bound_.identifier_;
                    Ok(Some(super::super::Identifier(RustString::from(
                        CStr::from_ptr(c_identifier).to_str()?,
                    ))))
                }
                _ => unreachable!(),
            }
        }
    }

    impl ParameterizedId_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<super::super::ParameterizedId<OwnedNameParameters<A>>, ParseError> {
            let c_identifier = (*self).u.paramid_.identifier_;
            let c_params = (*self).u.paramid_.params_;
            Ok(super::super::ParameterizedId {
                id: super::super::Identifier(RustString::from(
                    CStr::from_ptr(c_identifier).to_str()?,
                )),
                params: match (*c_params).kind {
                    Params__is_UnParameterized => Vec::new_in(alloc.clone()),
                    Params__is_Parameterized => {
                        let c_params_ = (*c_params).u.parameterized_.listparam_;
                        c_params_.to_rust(alloc)?
                    }
                    _ => unreachable!(),
                },
            })
        }
    }

    impl Qualification_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<Option<super::super::NamespaceId<OwnedNameParameters<A>>>, ParseError> {
            unimplemented!()
        }
    }

    impl ListSubstitutionSpec_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<Vec<super::super::SubstitutionSpec<OwnedNameParameters<A>>, A>, ParseError>
        {
            unimplemented!()
        }
    }

    impl ListParam_ {
        pub unsafe fn to_rust<A: Allocator + Clone>(
            self: *const Self,
            alloc: &A,
        ) -> Result<Vec<super::super::Parameter<OwnedNameParameters<A>>, A>, ParseError> {
            unimplemented!()
        }
    }
}
