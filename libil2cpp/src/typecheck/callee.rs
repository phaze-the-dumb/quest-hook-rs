use std::any::Any;
use std::fmt;

use crate::{Builtin, Il2CppType, MethodInfo, Type};

use super::ty::semantics;

/// Trait implemented by types that can be used as C# `this` method parameters
///
/// # Note
/// You should most likely not be implementing this trait yourself, but rather
/// the [`Type`] trait
///
/// # Safety
/// The implementation must be correct
pub unsafe trait ThisParameter {
    /// Type of the actual `this` parameter
    type Actual;
    /// Normalized type of `this`, useful for caching
    type Type: Any;

    /// Checks whether the type can be used as a C# instance parameter for the
    /// given [`MethodInfo`]
    fn matches(method: &MethodInfo) -> bool;

    /// Converts from the actual type to the desired one
    fn from_actual(actual: Self::Actual) -> Self;
    /// Converts from the desired type into the actual one
    fn into_actual(self) -> Self::Actual;
}

/// Trait implemented by types that can be used as C# method parameters
///
/// # Note
/// You should most likely not be implementing this trait yourself, but rather
/// the [`Type`] trait
///
/// # Safety
/// The implementation must be correct
pub unsafe trait Parameter {
    /// Type of the actual parameter
    type Actual;
    /// Normalized type of the parameter, useful for caching
    type Type: Any;

    /// Checks whether the type can be used as a C# parameter with the given
    /// [`Il2CppType`]
    fn matches(ty: &Il2CppType) -> bool;

    /// Converts from the actual type to the desired one
    fn from_actual(actual: Self::Actual) -> Self;
    /// Converts from the desired type into the actual one
    fn into_actual(self) -> Self::Actual;
}

/// Trait implemented by types that can be used as return types for C#
/// methods
///
/// # Note
/// You should most likely not be implementing this trait yourself, but rather
/// the [`Type`] trait
///
/// # Safety
/// The implementation must be correct
pub unsafe trait Return {
    /// Type of the actual return value
    type Actual;
    /// Normalized type of the return value, useful for caching
    type Type: Any;

    /// Checks whether the type can be used as a C# return type of the given
    /// [`Il2CppType`]
    fn matches(ty: &Il2CppType) -> bool;

    /// Converts from the desired type into the actual one
    fn into_actual(self) -> Self::Actual;
    /// Converts from the actual type to the desired one
    fn from_actual(actual: Self::Actual) -> Self;
}

/// Trait implemented by types that can be used as a collection of C# method
/// parameters
///
/// # Note
/// You should most likely not be implementing this trait yourself
///
/// # Safety
/// The implementation must be correct
pub unsafe trait Parameters {
    /// Normalized type of the parameters, useful for caching
    type Type: Any;

    /// Parameter count
    const COUNT: usize;

    /// Checks whether the type can be used as a C# parameter collection for the
    /// given [`MethodInfo`]
    fn matches(method: &MethodInfo) -> bool;
}

unsafe impl<T> ThisParameter for Option<&mut T>
where
    T: Type,
{
    type Actual = Self;
    type Type = T;

    fn matches(method: &MethodInfo) -> bool {
        T::matches_this_parameter(method)
    }

    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}

unsafe impl<T> ThisParameter for &mut T
where
    T: Type,
{
    type Actual = Option<Self>;
    type Type = T;

    fn matches(method: &MethodInfo) -> bool {
        T::matches_this_parameter(method)
    }

    fn from_actual(actual: Self::Actual) -> Self {
        actual.unwrap()
    }
    fn into_actual(self) -> Self::Actual {
        Some(self)
    }
}

unsafe impl ThisParameter for () {
    type Actual = ();
    type Type = ();

    fn matches(method: &MethodInfo) -> bool {
        method.is_static()
    }

    fn from_actual((): ()) {}
    fn into_actual(self) {}
}

unsafe impl<T, S> Parameter for Option<&mut T>
where
    T: Type<Semantics = S>,
    S: semantics::ReferenceParameter,
{
    type Actual = Self;
    type Type = T;

    fn matches(ty: &Il2CppType) -> bool {
        T::matches_reference_parameter(ty)
    }

    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}

unsafe impl<T, S> Parameter for &mut T
where
    T: Type<Semantics = S>,
    S: semantics::ReferenceParameter,
{
    type Actual = Option<Self>;
    type Type = T;

    fn matches(ty: &Il2CppType) -> bool {
        T::matches_reference_parameter(ty)
    }

    fn from_actual(actual: Self::Actual) -> Self {
        actual.unwrap()
    }
    fn into_actual(self) -> Self::Actual {
        Some(self)
    }
}

unsafe impl<T, S> Return for Option<&mut T>
where
    T: Type<Semantics = S>,
    S: semantics::ReferenceReturn,
{
    type Actual = Self;
    type Type = T;

    fn matches(ty: &Il2CppType) -> bool {
        T::matches_reference_return(ty)
    }

    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}

unsafe impl<T, S> Return for &mut T
where
    T: Type<Semantics = S>,
    S: semantics::ReferenceReturn,
{
    type Actual = Option<Self>;
    type Type = T;

    fn matches(ty: &Il2CppType) -> bool {
        T::matches_reference_return(ty)
    }

    fn into_actual(self) -> Self::Actual {
        Some(self)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual.unwrap()
    }
}

unsafe impl Return for () {
    type Actual = ();
    type Type = ();

    fn matches(ty: &Il2CppType) -> bool {
        ty.is_builtin(Builtin::Void)
    }

    fn into_actual(self) {}
    fn from_actual((): ()) {}
}

unsafe impl<T, E> Return for Result<T, E>
where
    T: Return,
    E: fmt::Debug,
{
    type Actual = T::Actual;
    type Type = T::Type;

    fn matches(ty: &Il2CppType) -> bool {
        T::matches(ty)
    }

    fn into_actual(self) -> Self::Actual {
        self.unwrap().into_actual()
    }
    fn from_actual(actual: Self::Actual) -> Self {
        Ok(T::from_actual(actual))
    }
}

unsafe impl Parameters for () {
    type Type = ();

    const COUNT: usize = 0;

    fn matches(method: &MethodInfo) -> bool {
        method.parameters().is_empty()
    }
}

unsafe impl<P> Parameters for P
where
    P: Parameter,
{
    type Type = (P::Type,);

    const COUNT: usize = 1;

    fn matches(method: &MethodInfo) -> bool {
        let params = method.parameters();
        params.len() == 1 && unsafe { P::matches(params.get_unchecked(0).ty()) }
    }
}
