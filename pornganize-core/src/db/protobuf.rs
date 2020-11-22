use std::iter::FromIterator as _;
use ::protobuf::{
    RepeatedField,
    SingularPtrField,
    Message,
    ProtobufEnum,
};

pub fn ptr_to_option<T, M>(ptr: SingularPtrField<M>) -> Option<T>  where
    T: From<M>
{
    match ptr.into_option() {
        Some(v) => Some(T::from(v)),
        None => None,
    }
}

pub fn option_to_ptr<T, M>(option: Option<T>) -> SingularPtrField<M> where
    M: From<T>
{
    SingularPtrField::from_option(
        match option {
            Some(v) => Some(M::from(v)),
            None => None,
        }
    )
}

pub fn to_repeated_field<M, I>(from: I) -> RepeatedField<M> where
    I: IntoIterator,
    M: From<I::Item>,
{
    RepeatedField::from_iter(from.into_iter().map(M::from))
}

pub fn from_repeated_field<M, T: From<M>>(field: RepeatedField<M>) -> Vec<T> {
    field.into_iter().map(T::from).collect()
}

