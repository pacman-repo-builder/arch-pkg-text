use super::{
    QueryBaseField, QueryBaseFieldMut, QueryDerivativeField, QueryDerivativeFieldMut, QueryField,
    QueryFieldMut,
};
use crate::{
    srcinfo::field::FieldName,
    value::{Base, Name},
};

/// Query the fields of a derivative section with inheritance from the `pkgbase` section.
#[derive(Debug, Clone, Copy)]
pub struct JoinedSection<BaseSection, DerivativeExclusiveSection> {
    base: BaseSection,
    derivative_exclusive: DerivativeExclusiveSection,
}

impl<BaseSection, DerivativeExclusiveSection>
    JoinedSection<BaseSection, DerivativeExclusiveSection>
{
    /// Join a `pkgbase` section and a `pkgname` exclusive section into a new querier.
    pub fn new(base: BaseSection, derivative_exclusive: DerivativeExclusiveSection) -> Self {
        Self {
            base,
            derivative_exclusive,
        }
    }

    /// Get the value of `pkgbase`.
    pub fn base_name<'a>(&self) -> Base<'a>
    where
        BaseSection: QueryBaseField<'a>,
    {
        self.base.name()
    }

    /// Get the value of `pkgname` in this section.
    pub fn derivative_name<'a>(&self) -> Name<'a>
    where
        DerivativeExclusiveSection: QueryDerivativeField<'a>,
    {
        self.derivative_exclusive.name()
    }
}

impl<'a, BaseSection, DerivativeExclusiveSection> QueryField<'a>
    for JoinedSection<BaseSection, DerivativeExclusiveSection>
where
    BaseSection: QueryBaseField<'a>,
    DerivativeExclusiveSection: QueryDerivativeField<'a>,
{
    fn query_raw_text(
        &self,
        field_name: FieldName,
    ) -> impl IntoIterator<Item = super::QueryRawTextItem<'a>> {
        self.base
            .query_raw_text(field_name)
            .into_iter()
            .chain(self.derivative_exclusive.query_raw_text(field_name))
    }
}

impl<'a, BaseSection, DerivativeExclusiveSection> QueryFieldMut<'a>
    for JoinedSection<BaseSection, DerivativeExclusiveSection>
where
    BaseSection: QueryBaseField<'a>,
    DerivativeExclusiveSection: QueryDerivativeField<'a>,
{
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl IntoIterator<Item = super::QueryRawTextItem<'a>> {
        self.query_raw_text(field_name)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JoinedMutSection<BaseSection, DerivativeExclusiveSection> {
    base: BaseSection,
    derivative_exclusive: DerivativeExclusiveSection,
}

impl<BaseSection, DerivativeExclusiveSection>
    JoinedMutSection<BaseSection, DerivativeExclusiveSection>
{
    /// Join a `pkgbase` section and a `pkgname` exclusive section into a new querier.
    pub fn new(base: BaseSection, derivative_exclusive: DerivativeExclusiveSection) -> Self {
        JoinedMutSection {
            base,
            derivative_exclusive,
        }
    }

    /// Get the value of `pkgbase`.
    pub fn base_name_mut<'a>(&mut self) -> Base<'a>
    where
        BaseSection: QueryBaseFieldMut<'a>,
    {
        self.base.name_mut()
    }

    /// Get the value of `pkgname` in this section.
    pub fn derivative_name_mut<'a>(&mut self) -> Name<'a>
    where
        DerivativeExclusiveSection: QueryDerivativeFieldMut<'a>,
    {
        self.derivative_exclusive.name_mut()
    }
}

impl<'a, BaseSection, DerivativeExclusiveSection> QueryFieldMut<'a>
    for JoinedMutSection<BaseSection, DerivativeExclusiveSection>
where
    BaseSection: QueryBaseFieldMut<'a>,
    DerivativeExclusiveSection: QueryDerivativeFieldMut<'a>,
{
    fn query_raw_text_mut(
        &mut self,
        field_name: FieldName,
    ) -> impl IntoIterator<Item = super::QueryRawTextItem<'a>> {
        self.base
            .query_raw_text_mut(field_name)
            .into_iter()
            .chain(self.derivative_exclusive.query_raw_text_mut(field_name))
    }
}
