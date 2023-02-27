use super::{BinaryQuery, Query};
use halo2_proofs::{
    arithmetic::Field,
    circuit::{Region, Value},
    plonk::{Advice, Column, Fixed, Selector},
    poly::Rotation,
};
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct SelectorColumn(pub Selector);

impl SelectorColumn {
    pub fn current<F: Field>(self) -> BinaryQuery<F> {
        BinaryQuery(Query(Box::new(move |meta| meta.query_selector(self.0))))
    }
}

#[derive(Clone, Copy)]
pub struct FixedColumn(pub Column<Fixed>);

impl FixedColumn {
    pub fn rotation<F: Field>(self, i: i32) -> Query<F> {
        Query(Box::new(move |meta| meta.query_fixed(self.0, Rotation(i))))
    }

    pub fn current<F: Field>(self) -> Query<F> {
        self.rotation(0)
    }

    pub fn previous<F: Field>(self) -> Query<F> {
        self.rotation(-1)
    }

    pub fn assign<F: Field, T: Copy + TryInto<F>>(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        value: T,
    ) where
        <T as TryInto<F>>::Error: Debug,
    {
        region
            .assign_fixed(
                || "",
                self.0,
                offset,
                || Value::known(value.try_into().unwrap()),
            )
            .expect("failed assign_fixed");
    }
}

#[derive(Clone, Copy)]
pub struct AdviceColumn(pub Column<Advice>);

impl AdviceColumn {
    pub fn rotation<F: Field>(self, i: i32) -> Query<F> {
        Query(Box::new(move |meta| meta.query_advice(self.0, Rotation(i))))
    }

    pub fn current<F: Field>(self) -> Query<F> {
        self.rotation(0)
    }

    pub fn previous<F: Field>(self) -> Query<F> {
        self.rotation(-1)
    }

    pub fn assign<F: Field, T: Copy + TryInto<F>>(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        value: T,
    ) where
        <T as TryInto<F>>::Error: Debug,
    {
        region
            .assign_advice(
                || "",
                self.0,
                offset,
                || Value::known(value.try_into().unwrap()),
            )
            .expect("failed assign_advice");
    }
}

#[derive(Clone, Copy)]
pub struct IsZeroColumn(pub AdviceColumn);

// probably a better name for this is IsZeroConfig
impl IsZeroColumn {
    pub fn rotation<F: Field>(self, i: i32) -> Query<F> {
        self.0.rotation(i)
    }

    pub fn current<F: Field>(self) -> Query<F> {
        self.0.current()
    }

    pub fn previous<F: Field>(self) -> Query<F> {
        self.0.previous()
    }

    pub fn assign<F: Field, T: Copy + TryInto<F>>(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        value: T,
    ) where
        <T as TryInto<F>>::Error: Debug,
    {
        self.0.assign(
            region,
            offset,
            value.try_into().unwrap().invert().unwrap_or(F::zero()),
        );
    }
}
