use std::fmt;
use std::iter::IntoIterator;

pub trait NameParameters {
    type DeclarationRef: AsRef<Declaration<Self>>;
    type Declarations: IntoIterator<Item = Self::DeclarationRef>;

    type NestedName: AsRef<Name<Self>>;

    type SubstitutionRef: AsRef<Substitution<Self>>;

    type SubstitutionSpecs: IntoIterator<Item = SubstitutionSpec<Self>>;

    type ParameterName: AsRef<str>;
    type ParameterValue: AsRef<str>;
    // TODO Should this be a map-like interface instead of a list of kv pairs?
    type Parameters: IntoIterator<Item = Parameter<Self>>;

    type Identifier: AsRef<str>;
}

pub struct Name<P: NameParameters + ?Sized> {
    pub first_declarations: P::Declarations,
    pub last_declaration: P::DeclarationRef,
}

impl<P: NameParameters + ?Sized> fmt::Debug for Name<P>
where
    P::Declarations: fmt::Debug,
    P::DeclarationRef: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Name")
            .field("first_declarations", &self.first_declarations)
            .field("last_declaration", &self.last_declaration)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Name<P>
where
    P::Declarations: Clone,
    P::DeclarationRef: Clone,
{
    fn clone(&self) -> Self {
        Self {
            first_declarations: self.first_declarations.clone(),
            last_declaration: self.last_declaration.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for Name<P>
where
    P::Declarations: Copy,
    P::DeclarationRef: Copy,
{
}

pub struct Declaration<P: NameParameters + ?Sized> {
    pub var: Option<Identifier<P>>,
    pub val: Substitution<P>,
}

impl<P: NameParameters + ?Sized> fmt::Debug for Declaration<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
    P::Parameters: fmt::Debug,
    P::NestedName: fmt::Debug,
    P::SubstitutionSpecs: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Declaration")
            .field("var", &self.var)
            .field("val", &self.val)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Declaration<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
    P::Parameters: Clone,
    P::NestedName: Clone,
    P::SubstitutionSpecs: Clone,
{
    fn clone(&self) -> Self {
        Self {
            var: self.var.clone(),
            val: self.val.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for Declaration<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
    P::Parameters: Copy,
    P::NestedName: Copy,
    P::SubstitutionSpecs: Copy,
{
}

pub struct Substitution<P: NameParameters + ?Sized> {
    pub name: NameRef<P>,
    pub inputs: P::SubstitutionSpecs,
}

impl<P: NameParameters + ?Sized> fmt::Debug for Substitution<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
    P::Parameters: fmt::Debug,
    P::NestedName: fmt::Debug,
    P::SubstitutionSpecs: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Substitution")
            .field("name", &self.name)
            .field("inputs", &self.inputs)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Substitution<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
    P::Parameters: Clone,
    P::NestedName: Clone,
    P::SubstitutionSpecs: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            inputs: self.inputs.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for Substitution<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
    P::Parameters: Copy,
    P::NestedName: Copy,
    P::SubstitutionSpecs: Copy,
{
}

pub enum NameRef<P: NameParameters + ?Sized> {
    Atomic {
        namespace_id: Option<NamespaceId<P>>,
        name_id: ParameterizedId<P>,
    },
    Variable(Identifier<P>),
    Resolved(Identifier<P>),
    Nested(P::NestedName),
}

impl<P: NameParameters + ?Sized> fmt::Debug for NameRef<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
    P::Parameters: fmt::Debug,
    P::NestedName: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NameRef::Atomic {
                namespace_id,
                name_id,
            } => fmt
                .debug_struct("Atomic")
                .field("namespace_id", &namespace_id)
                .field("name_id", &name_id)
                .finish(),
            NameRef::Variable(var) => fmt.debug_tuple("Variable").field(&var).finish(),
            NameRef::Resolved(var) => fmt.debug_tuple("Resolved").field(&var).finish(),
            NameRef::Nested(nm) => fmt.debug_tuple("Nested").field(&nm).finish(),
        }
    }
}

impl<P: NameParameters + ?Sized> Clone for NameRef<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
    P::Parameters: Clone,
    P::NestedName: Clone,
{
    fn clone(&self) -> Self {
        match self {
            NameRef::Atomic {
                namespace_id,
                name_id,
            } => NameRef::Atomic {
                namespace_id: namespace_id.clone(),
                name_id: name_id.clone(),
            },
            NameRef::Variable(var) => NameRef::Variable(var.clone()),
            NameRef::Resolved(var) => NameRef::Resolved(var.clone()),
            NameRef::Nested(nm) => NameRef::Nested(nm.clone()),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for NameRef<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
    P::Parameters: Copy,
    P::NestedName: Copy,
{
}

pub struct SubstitutionSpec<P: NameParameters + ?Sized> {
    pub input_id: Option<Identifier<P>>,
    pub input_val: OutputRef<P>,
}

impl<P: NameParameters + ?Sized> fmt::Debug for SubstitutionSpec<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SubstitutionSpec")
            .field("input_id", &self.input_id)
            .field("input_val", &self.input_val)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for SubstitutionSpec<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
{
    fn clone(&self) -> Self {
        Self {
            input_id: self.input_id.clone(),
            input_val: self.input_val.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for SubstitutionSpec<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
{
}

pub struct OutputRef<P: NameParameters + ?Sized> {
    pub name: P::SubstitutionRef,
    pub output_id: Option<Identifier<P>>,
}

impl<P: NameParameters + ?Sized> fmt::Debug for OutputRef<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("OutputRef")
            .field("name", &self.name)
            .field("output_id", &self.output_id)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for OutputRef<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            output_id: self.output_id.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for OutputRef<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
{
}

pub enum NamespaceId<P: NameParameters + ?Sized> {
    Builtin(ParameterizedId<P>),
    Named(OutputRef<P>),
}

impl<P: NameParameters + ?Sized> fmt::Debug for NamespaceId<P>
where
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
    P::Parameters: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamespaceId::Builtin(pid) => fmt.debug_tuple("Builtin").field(&pid).finish(),
            NamespaceId::Named(output) => fmt.debug_tuple("Named").field(&output).finish(),
        }
    }
}

impl<P: NameParameters + ?Sized> Clone for NamespaceId<P>
where
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
    P::Parameters: Clone,
{
    fn clone(&self) -> Self {
        match self {
            NamespaceId::Builtin(pid) => NamespaceId::Builtin(pid.clone()),
            NamespaceId::Named(output) => NamespaceId::Named(output.clone()),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for NamespaceId<P>
where
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
    P::Parameters: Copy,
{
}

pub struct ParameterizedId<P: NameParameters + ?Sized> {
    id: Identifier<P>,
    params: P::Parameters,
}

impl<P: NameParameters + ?Sized> fmt::Debug for ParameterizedId<P>
where
    P::Identifier: fmt::Debug,
    P::Parameters: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ParameterizedId")
            .field("id", &self.id)
            .field("params", &self.params)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for ParameterizedId<P>
where
    P::Identifier: Clone,
    P::Parameters: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            params: self.params.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for ParameterizedId<P>
where
    P::Identifier: Copy,
    P::Parameters: Copy,
{
}

pub struct Parameter<P: NameParameters + ?Sized> {
    pub name: P::ParameterName,
    pub val: P::ParameterValue,
}

impl<P: NameParameters + ?Sized> fmt::Debug for Parameter<P>
where
    P::ParameterName: fmt::Debug,
    P::ParameterValue: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Parameter")
            .field("name", &self.name)
            .field("val", &self.val)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Parameter<P>
where
    P::ParameterName: Clone,
    P::ParameterValue: Clone,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            val: self.val.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for Parameter<P>
where
    P::ParameterName: Copy,
    P::ParameterValue: Copy,
{
}

pub struct Identifier<P: NameParameters + ?Sized>(pub P::Identifier);

impl<P: NameParameters + ?Sized> fmt::Debug for Identifier<P>
where
    P::Identifier: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Identifier").field(&self.0).finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Identifier<P>
where
    P::Identifier: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P: NameParameters + ?Sized> Copy for Identifier<P> where P::Identifier: Copy {}

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
