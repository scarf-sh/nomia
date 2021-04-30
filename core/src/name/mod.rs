//! Basic AST for names.

use std::fmt;
use std::iter::IntoIterator;

/// Type parameters for the [Name](Name) AST.
///
/// Implementing this trait allows the user to use alternative types
/// of references or sequences.
///
/// By default, you should just used [OwnedName](OwnedName).
pub trait NameParameters {
    /// A reference to a node declaration, held in the top-level [Name](Name).
    type DeclarationRef: AsRef<Declaration<Self>>;
    /// An iterable sequence of [DeclarationRefs](Self::DeclarationRef).
    type Declarations: IntoIterator<Item = Self::DeclarationRef>;

    /// A reference to another [Name](Name) composed into the [Name](Name) being defined.
    type NestedName: AsRef<Name<Self>>;

    /// A reference to a [Substitution](Substitution) nested under another [Substitution](Substitution).
    ///
    /// For example, in `foo(bar: baz(qux: quux))`, `baz(qux: quux)` is held as a `SubstitutionRef`.
    type SubstitutionRef: AsRef<Substitution<Self>>;

    /// An iterable sequence of [SubstitutionSpecs](SubstitutionSpec), the inputs to a [Substitution](Substitution).
    type SubstitutionSpecs: IntoIterator<Item = SubstitutionSpec<Self>>;

    /// Key names in a parameter list.
    type ParameterName: AsRef<str>;
    // TODO Multiple types
    /// Values in a parameter list.
    type ParameterValue: AsRef<str>;
    // TODO Should this be a map-like interface instead of a list of kv pairs?
    /// An iterable sequence of key-value [Parameters](Parameter).
    type Parameters: IntoIterator<Item = Parameter<Self>>;

    /// An identifier for variables and name/namespace IDs.
    type Identifier: AsRef<str>;
}

/// A Nomia name
///
/// In fully generic form, `Name`s consist of a (possibly empty)
/// sequence of node [Declarations](Declaration) bound in a `let` and
/// a terminal node [Substitution](Substitution).
pub struct Name<P: NameParameters + ?Sized> {
    /// The let-bound nodes.
    pub let_declarations: P::Declarations,
    /// The terminal node.
    pub terminal_substitution: Substitution<P>,
}

impl<P: NameParameters + ?Sized> fmt::Debug for Name<P>
where
    P::Declarations: fmt::Debug,
    P::Identifier: fmt::Debug,
    P::SubstitutionRef: fmt::Debug,
    P::Parameters: fmt::Debug,
    P::NestedName: fmt::Debug,
    P::SubstitutionSpecs: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Name")
            .field("let_declarations", &self.let_declarations)
            .field("terminal_substitution", &self.terminal_substitution)
            .finish()
    }
}

impl<P: NameParameters + ?Sized> Clone for Name<P>
where
    P::Declarations: Clone,
    P::Identifier: Clone,
    P::SubstitutionRef: Clone,
    P::Parameters: Clone,
    P::NestedName: Clone,
    P::SubstitutionSpecs: Clone,
{
    fn clone(&self) -> Self {
        Self {
            let_declarations: self.let_declarations.clone(),
            terminal_substitution: self.terminal_substitution.clone(),
        }
    }
}

impl<P: NameParameters + ?Sized> Copy for Name<P>
where
    P::Declarations: Copy,
    P::Identifier: Copy,
    P::SubstitutionRef: Copy,
    P::Parameters: Copy,
    P::NestedName: Copy,
    P::SubstitutionSpecs: Copy,
{
}

/// A node declaration in a composition graph.
///
/// Each `Declaration` defines a name and any names substituted into
/// it.
///
/// The substituted name can optionally be assigned to a specified
/// variable.
pub struct Declaration<P: NameParameters + ?Sized> {
    /// The variable name to bind to the substituted name,
    /// if any.
    pub var: Option<Identifier<P>>,
    /// The name and its substituted inputs.
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

/// A name together with any inputs substituted into it.
pub struct Substitution<P: NameParameters + ?Sized> {
    /// An identification of the name to substitute into.
    pub name: NameRef<P>,
    /// The inputs to substitute into the given name.
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

/// An identification of a [Name](Name).
pub enum NameRef<P: NameParameters + ?Sized> {
    /// An atomic name, with no substitutions.
    Atomic {
        /// The namespace the name is in.
        ///
        /// If omitted, it must be filled in from the context this
        /// name is resolved in.
        namespace_id: Option<NamespaceId<P>>,
        /// The specific name within the namespace.
        name_id: ParameterizedId<P>,
    },
    /// A variable.
    ///
    /// The variable must be bound somewhere within the scope of
    /// the usage.
    Variable(Identifier<P>),
    /// A resolved name reference.
    ///
    /// The variable name must be bound to an appropriate resolved
    /// name when resolving the name it is referenced within.
    Resolved(Identifier<P>),
    /// A recursively inlined other name.
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

/// A specification of an input to be substituted.
pub struct SubstitutionSpec<P: NameParameters + ?Sized> {
    /// The name of the input to substitute into.
    ///
    /// If omitted, the input must be determinable positionally.
    pub input_id: Option<Identifier<P>>,
    /// The name output to substitute into the given input.
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

/// A reference to an output of some name.
pub struct OutputRef<P: NameParameters + ?Sized> {
    /// The name whose output is being referenced.
    pub name: P::SubstitutionRef,
    /// The specific output being referenced.
    ///
    /// If omitted, the output ID must be inferrable from the
    /// resolution context (either because there is only one or through
    /// a default output ID).
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

/// An identification of a namespace within a root namespace of
/// namespaces.
pub enum NamespaceId<P: NameParameters + ?Sized> {
    /// A namespace identified by an opaque ID and parameters.
    Builtin(ParameterizedId<P>),
    /// A namespace identified as the output of some other name.
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

/// An identifier paired with a parameter list.
pub struct ParameterizedId<P: NameParameters + ?Sized> {
    /// The opaque identifier.
    id: Identifier<P>,
    /// The parameters modifying the identifier.
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

/// A key-value pair parameterizing some identifier.
pub struct Parameter<P: NameParameters + ?Sized> {
    /// The parameter name.
    pub name: P::ParameterName,
    /// The parameter value.
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

/// An opaque identifier.
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

/// A type to carry [NameParameters](NameParameters) using [Box](Box)
/// for references and [Vec](Vec) for sequences.
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

/// A [Name](Name) where all internal references and sequences are owned.
///
/// Built on top of [Box](Box) and [Vec](Vec).
pub type OwnedName = Name<OwnedNameParameters>;
