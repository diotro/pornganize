use std::iter::FromIterator as _;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use protobuf::{
    RepeatedField,
    SingularPtrField,
    Message,
    ProtobufEnum,
};

pub struct MessageVec<M: Message>(pub Vec<M>);

impl<M: Message> MessageVec<M> {
    pub fn from_vec(v: Vec<M>) -> Self {
        Self(v)
    }

    pub fn value(self) -> Vec<M> {
        self.0
    }
}

impl<M: Message> Deref for MessageVec<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, M, T> From<&'a Vec<T>> for MessageVec<M> where
    M: From<&'a T> + Message,
{
    fn from(v: &'a Vec<T>) -> Self {
        Self(v.iter().map(M::from).collect())
    }
}

impl<'a, M, T> From<&'a MessageVec<M>> for Vec<T> where
    T: From<&'a M>,
    M: Message,
{
    fn from(messages: &'a MessageVec<M>) -> Self {
        messages.iter().map(T::from).collect()
    }
}

pub struct ProtobufEnumVec<'a, M: ProtobufEnum>(pub &'a Vec<M>);

impl<'a, M: ProtobufEnum> ProtobufEnumVec<'a, M> {
    #[allow(clippy::ptr_arg)]
    pub fn from_vec(v: &'a Vec<M>) -> Self {
        Self(v)
    }
}

impl<'a, M: ProtobufEnum> Deref for ProtobufEnumVec<'a, M> {
    type Target = &'a Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<'a, 'b, M, T> From<&'b ProtobufEnumVec<'a, M>> for Vec<T> where
    'b: 'a + 'b,
    T: From<&'a M>,
    M: ProtobufEnum,
{
    fn from(values: &'b ProtobufEnumVec<'a, M>) -> Self {
        values.iter().map(T::from).collect()
    }
}

impl<'a, M, T> From<ProtobufEnumVec<'a, M>> for Vec<T> where
    T: From<&'a M>,
    M: ProtobufEnum,
{
    fn from(values: ProtobufEnumVec<'a, M>) -> Self {
        values.iter().map(T::from).collect()
    }
}

pub fn to_vec<'a, M: From<&'a T>, T>(from: &'a [T]) -> Vec<M> {
    from.iter().map(M::from).collect()
}

pub fn from_vec<'a, M, T: From<&'a M>>(field: &'a [M]) -> Vec<T> {
    field.iter().map(T::from).collect()
}

pub fn to_repeated_field<'a, M: From<&'a T>, T>(from: &'a [T]) -> RepeatedField<M> {
    RepeatedField::from_iter(from.iter().map(M::from))
}

pub fn from_repeated_field<'a, M, T: From<&'a M>>(field: &'a RepeatedField<M>) -> Vec<T> {
    field.into_iter().map(T::from).collect()
}
