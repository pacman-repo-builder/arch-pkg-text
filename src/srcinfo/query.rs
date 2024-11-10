use crate::srcinfo::field::FieldName;

/// Query a section from a `.SRCINFO` file.
pub trait QuerySection<'a, Base, Derivative>: QuerySectionMut<'a, Base, Derivative> {
    /// Get the section under `pkgbase`.
    fn base(&self) -> Self::BaseSection;

    /// Get all the sections under `pkgname`.
    fn derivatives(&self) -> Self::DerivativeContainer;

    /// Get a section whose `pkgname` matches `name`.
    fn derivative(&self, name: Derivative) -> Option<Self::DerivativeSection>;
}

/// Query a field of the `pkgbase` section of a `.SRCINFO` file.
pub trait QueryBaseField<'a, Base>: QueryField<'a> + QueryBaseFieldMut<'a, Base> {
    /// Get the value of the `pkgbase` field of the `.SRCINFO` file.
    fn name(&self) -> Base;
}

/// Query a field of a `pkgname` section of a `.SRCINFO` file.
pub trait QueryDerivativeField<'a, Derivative>:
    QueryField<'a> + QueryDerivativeFieldMut<'a, Derivative>
{
    /// Get the value of the `pkgname` field of the section.
    fn name(&self) -> Derivative;
}

/// Query a field of either a `pkgbase` or `pkgname` section of a `.SRCINFO` file.
pub trait QueryField<'a>: QueryFieldMut<'a> {
    fn query_raw_text(&self, field_name: FieldName) -> Option<Self::QueryRawTextReturn>;
}

/// Query a section from a `.SRCINFO` file.
pub trait QuerySectionMut<'a, Base, Derivative> {
    type BaseSection: QueryBaseFieldMut<'a, Base>;
    type DerivativeSection: QueryDerivativeFieldMut<'a, Derivative>;
    type DerivativeContainer: IntoIterator<Item = Self::DerivativeSection>;

    /// Get the section under `pkgbase`.
    fn base_mut(&mut self) -> Self::BaseSection;

    /// Get all the sections under `pkgname`.
    fn derivatives_mut(&mut self) -> Self::DerivativeContainer;

    /// Get a section whose `pkgname` matches `name`.
    fn derivative_mut(&mut self, name: Derivative) -> Option<Self::DerivativeSection>;
}

/// Query a field of the `pkgbase` section of a `.SRCINFO` file.
pub trait QueryBaseFieldMut<'a, Base>: QueryFieldMut<'a> {
    /// Get the value of the `pkgbase` field of the `.SRCINFO` file.
    fn name_mut(&mut self) -> Base;
}

/// Query a field of a `pkgname` section of a `.SRCINFO` file.
pub trait QueryDerivativeFieldMut<'a, Derivative>: QueryFieldMut<'a> {
    /// Get the value of the `pkgname` field of the section.
    fn name_mut(&mut self) -> Derivative;
}

pub trait QueryFieldMut<'a> {
    type QueryRawTextReturn: IntoIterator<Item = QueryRawTextItem<'a>>;
    fn query_raw_text_mut(&mut self, field_name: FieldName) -> Option<Self::QueryRawTextReturn>;
}

/// [Iterator item](Iterator::Item) of [`QueryFieldMut::QueryRawTextReturn`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryRawTextItem<'a> {
    /// Architecture of the field.
    pub architecture: Option<&'a str>,
    /// Value of the field.
    pub value: &'a str,
}
