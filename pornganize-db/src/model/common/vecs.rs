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

impl<M, T> From<Vec<T>> for MessageVec<M> where
    M: From<T> + Message,
{
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(M::from).collect())
    }
}

impl<M, T> From<MessageVec<M>> for Vec<T> where
    T: From<M>,
    M: Message,
{
    fn from(messages: MessageVec<M>) -> Self {
        messages.value().into_iter().map(T::from).collect()
    }
}

pub struct ProtobufEnumVec<M: ProtobufEnum>(pub Vec<M>);

impl<M: ProtobufEnum> ProtobufEnumVec<M> {
    #[allow(clippy::ptr_arg)]
    pub fn from_vec(v: Vec<M>) -> Self {
        Self(v)
    }

    pub fn value(self) -> Vec<M> {
        self.0
    }
}

impl<M: ProtobufEnum> Deref for ProtobufEnumVec<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<M, T> From<ProtobufEnumVec<M>> for Vec<T> where
    T: From<M>,
    M: ProtobufEnum,
{
    fn from(values: ProtobufEnumVec<M>) -> Self {
        values.value().into_iter().map(T::from).collect()
    }
}

pub fn convert_vec<M, I>(from: I) -> Vec<M> where
    I: IntoIterator,
    M: From<I::Item>,
{
    from.into_iter().map(M::from).collect()
}
