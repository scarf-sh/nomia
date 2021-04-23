use std::fmt;
use std::iter::IntoIterator;

//TODO Move Debug bounds to impls, see https://github.com/rust-lang/rust/issues/26925#issuecomment-818606398 for why we can't just derive()
//     Also make Clone conditionally when we do that
pub trait NameParameters {
    type DeclarationRef: AsRef<Declaration<Self>> + fmt::Debug;
    type Declarations: IntoIterator<Item = Self::DeclarationRef> + fmt::Debug;

    type NestedName: AsRef<Name<Self>> + fmt::Debug;

    type SubstitutionRef: AsRef<Substitution<Self>> + fmt::Debug;

    type SubstitutionSpecs: IntoIterator<Item = SubstitutionSpec<Self>> + fmt::Debug;

    type ParameterName: AsRef<str> + fmt::Debug;
    type ParameterValue: AsRef<str> + fmt::Debug;
    // TODO Should this be a map-like interface instead of a list of kv pairs?
    type Parameters: IntoIterator<Item = Parameter<Self>> + fmt::Debug;

    type Identifier: AsRef<str> + fmt::Debug;
}

#[derive(Debug)]
pub struct Name<P: NameParameters + ?Sized> {
    first_declarations: P::Declarations,
    last_declaration: P::DeclarationRef,
}

#[derive(Debug)]
pub struct Declaration<P: NameParameters + ?Sized> {
    pub var: Option<Identifier<P>>,
    pub val: Substitution<P>,
}

#[derive(Debug)]
pub struct Substitution<P: NameParameters + ?Sized> {
    pub name: NameRef<P>,
    pub inputs: P::SubstitutionSpecs,
}

#[derive(Debug)]
pub enum NameRef<P: NameParameters + ?Sized> {
    Atomic {
        namespace_id: Option<NamespaceId<P>>,
        name_id: ParameterizedId<P>,
    },
    Variable(Identifier<P>),
    Resolved(Identifier<P>),
    Nested(P::NestedName),
}

#[derive(Debug)]
pub struct SubstitutionSpec<P: NameParameters + ?Sized> {
    pub input_id: Option<Identifier<P>>,
    pub input_val: OutputRef<P>,
}

#[derive(Debug)]
pub struct OutputRef<P: NameParameters + ?Sized> {
    pub name: P::SubstitutionRef,
    pub output_id: Option<Identifier<P>>,
}

#[derive(Debug)]
pub enum NamespaceId<P: NameParameters + ?Sized> {
    Builtin(ParameterizedId<P>),
    Named(OutputRef<P>),
}

#[derive(Debug)]
pub struct ParameterizedId<P: NameParameters + ?Sized> {
    id: Identifier<P>,
    params: P::Parameters,
}

#[derive(Debug)]
pub struct Parameter<P: NameParameters + ?Sized> {
    pub name: P::ParameterName,
    pub val: P::ParameterValue,
}

#[derive(Debug)]
pub struct Identifier<P: NameParameters + ?Sized>(pub P::Identifier);

#[derive(Debug)]
pub enum OwnedNameParameters {}

impl NameParameters for OwnedNameParameters {
    type DeclarationRef = Box<Declaration<Self>>;
    type Declarations = Vec<Self::DeclarationRef>;

    type NestedName = Box<Name<Self>>;

    type SubstitutionRef = Box<Substitution<Self>>;

    type SubstitutionSpecs = Vec<SubstitutionSpec<Self>>;

    type ParameterName = String;
    type ParameterValue = String;
    type Parameters = Vec<Parameter<Self>>;

    type Identifier = String;
}

pub type OwnedName = Name<OwnedNameParameters>;
