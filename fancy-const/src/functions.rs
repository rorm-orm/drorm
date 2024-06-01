//! `const fn` in traits
//!
//! Traits can have 3 kinds of associated objects: `fn`, `const` and `type`.
//! `fn` can't be const yet, so we have to abuse one of the other two to build a workaround.
//! `const` could be used, but their type and therefore their possible implementations have
//! to be defined by the trait author. (See [example](#Alternative) below)
//! This module uses the third option (associated types).
//!
//! It provides two central traits:
//! - [`Contains`] which defines values i.e. arguments and returns
//! - [`ConstFn`] which defines functions
//!
//! # Basic Idea
//!
//! The basic idea is to have some type `add` (lower case because it should represent a function)
//! which implements `ConstFn<(i32, i32), i32>`.
//! The trait `ConstFn<(i32, i32), i32>` contains one associated constant
//! which takes a `(i32, i32)` as argument and is of type `i32`:
//! ```compile_fail
//! pub trait IdealConstFn<Args, Ret> {
//!     const RETURN<const ARGS: Args>: Ret;
//! }
//!
//! pub struct add;
//! impl IdealConstFn<(i32, i32), i32> for add {
//!     const RETURN<const ARGS: (i32, i32))>: i32 = {
//!         let (x, y) = ARGS;
//!
//!         // The actual function body
//!         x + 1
//!     };
//! }
//!
//! const THREE: i32 = <add as IdealConstFn<(i32, i32), i32>>::RETURN::<(1, 2)>;
//! ```
//!
//! # Actual implementation
//!
//! However, this ideal has two problems:
//! 1. associated consts can't be generic
//! 2. const generics can't be tuples
//!
//! Solving 1. is not much of an issue, just introduce another generic as indirection:
//! ```compile_fail
//! pub trait NotSoIdealConstFn<Args, Ret> {
//!     type Call<const ARGS: Args>: Return<Ret>;
//! }
//! pub trait Return<Ret> {
//!     const RETURN: Ret;
//! }
//!
//! pub struct add;
//! impl NotSoIdealConstFn<(i32, i32), i32> for add {
//!     type Call<const ARGS: (i32, i32))> = addBody<ARGS>;
//! }
//! pub struct addBody<const: ARGS: (i32, i32)>;
//! impl<const ARGS: (i32, i32)> Return<i32> for addBody<ARGS> {
//!     const RETURN: i32 = {
//!         let (x, y) = ARGS;
//!
//!         // The actual function body
//!         x + 1
//!     };
//! }
//! ```
//!
//! Solving 2. is doable but makes calling `ConstFn`s really unergonomic.
//! Since we can't have const generics, we have to fall back to normal generics.
//! However, this means, we have to have one type for each value we might want to pass to a function:
//! ```
//! pub trait Contains<T> {
//!     const ITEM: T;
//! }
//! pub struct OneAndOne;
//! impl Contains<(i32, i32)> for OneAndOne {
//!     const ITEM: (i32, i32) = (1, 1);
//! }
//! ```
//!
//! Actually `Contains<T>` and `Returns<Ret>` look the same, so we can just use `Contains` which brings us to the actual implementation:
//! ```
//! pub trait Contains<T> {
//!     const ITEM: T;
//! }
//! pub trait ConstFn<Arg, Ret> {
//!     // Named it `Body` instead of `Call`
//!     // which makes more sense from the implementors perspective.
//!     type Body<T: Contains<Arg>>: Contains<Ret>;
//! }
//!
//! pub struct add;
//! impl ConstFn<(i32, i32), i32> for add {
//!     type Body<T: Contains<(i32, i32)>> = addBody<T>;
//! }
//! pub struct addBody<T>([T; 0]);
//! impl<T: Contains<(i32, i32)>> Contains<i32> for addBody<T> {
//!     const ITEM: i32 = {
//!         let (x, y) = T::ITEM;
//!
//!         // actual body
//!         x + y
//!     };
//! }
//!
//! pub struct OneAndTwo;
//! impl Contains<(i32, i32)> for OneAndTwo {
//!     const ITEM: (i32, i32) = (1, 2);
//! }
//! const THREE: i32 = <<add as ConstFn<(i32, i32), i32>>::Body::<OneAndTwo> as Contains<i32>>::ITEM;
//! ```
//!
//! # Alternative
//! As described in the introduction paragraph, you could use associated `const`s.
//!
//! Their huge advantage is readability:
//!     There is no type magic, just basic rust.
//!
//! However, they also have a huge disadvantage:
//!     The trait author has to decide on the fixed set of possible methods an implementor may choose from.
//!
//! ```rust
//! trait SomeTrait {
//!     const SOME_METHOD: SomeMethod;
//! }
//! enum SomeMethod {
//!     ImplemenationA,
//!     ImplemenationB,
//!     // ...
//! }
//! impl SomeMethod {
//!     const fn call(self) {
//!         match self {
//!             Self::ImplemenationA => {},
//!             Self::ImplemenationB => {},
//!             // ...
//!         }
//!     }
//! }
//! ```

/// Attaches a constant value of type `T`.
///
/// See [module docs](self) for more information about how and why.
pub trait Contains<T> {
    /// The value attached to / represented by `Self`
    const ITEM: T;
}

/// A `const fn(...Arg) -> Ret` which can be used in traits.
///
/// See [`const_fn!`](crate::const_fn) for an easy way to implement this.
///
/// See [module docs](self) for more information about how and why.
pub trait ConstFn<Arg, Ret> {
    /// A type which is generic over `T` and uses `T::ITEM` in its `Contains` implementation
    /// to compute the "return" value.
    type Body<T: Contains<Arg>>: Contains<Ret>;
}

/// Converts a normal function into a [`ConstFn`].
///
/// Only accepts a very simple `fn` syntax!
///
/// ```
/// # use fancy_const::const_fn;
/// const_fn! {
///     fn add(x: i32, y: i32) -> i32 {
///         x + y
///     }
/// }
/// ```
#[macro_export]
macro_rules! const_fn {
    ($(#[$attr:meta])* $vis:vis fn $fun_name:ident($( $arg_name:tt : $arg_type:ty ),+ $(,)?) -> $ret_type:ty $body:block) => {
        /// `ConstFn` version of
        #[doc = concat!("[`", stringify!($fun_name), "`](fn@", stringify!($fun_name), ")")]
        #[allow(non_camel_case_types)]
        $vis struct $fun_name { phantom: ::core::marker::PhantomData<()> }

        $(#[$attr])*
        $vis const fn $fun_name($( $arg_name : $arg_type ),*) -> $ret_type $body
        const _: () = {
            impl $crate::ConstFn<($($arg_type,)+), $ret_type> for $fun_name {
                type Body<T: $crate::Contains<($($arg_type,)+)>> = Body<T>;
            }
            $vis struct Body<T: $crate::Contains<($($arg_type,)+)>>(::std::marker::PhantomData<T>);
            impl<T: $crate::Contains<($($arg_type,)+)>> $crate::Contains<$ret_type> for Body<T> {
                const ITEM: $ret_type = {
                    let ($($arg_name,)+) = T::ITEM;
                    $fun_name($($arg_name,)*)
                };
            }
        };
    };
    ($(#[$attr:meta])* $vis:vis fn $fun_name:ident<const $gen_name:ident: $gen_type:ty> ($( $arg_name:tt : $arg_type:ty ),* $(,)?) -> $ret_type:ty $body:block) => {
        /// `ConstFn` version of
        #[doc = concat!("[`", stringify!($fun_name), "`](fn@", stringify!($fun_name), ")")]
        #[allow(non_camel_case_types)]
        $vis struct $fun_name<const $gen_name: $gen_type> { phantom: ::core::marker::PhantomData<()> }

        $(#[$attr])*
        $vis const fn $fun_name<const $gen_name: $gen_type>($( $arg_name : $arg_type ),*) -> $ret_type $body
        const _: () = {
            impl<const $gen_name: $gen_type> $crate::ConstFn<($($arg_type,)*), $ret_type> for $fun_name<$gen_name> {
                type Body<Arg: $crate::Contains<($($arg_type,)*)>> = Body<Arg, $gen_name>;
            }
            $vis struct Body<Arg: $crate::Contains<($($arg_type,)*)>, const $gen_name: $gen_type>(::std::marker::PhantomData<Arg>);
            impl<Arg: $crate::Contains<($($arg_type,)*)>, const $gen_name: $gen_type> $crate::Contains<$ret_type> for Body<Arg, $gen_name> {
                const ITEM: $ret_type = {
                    let ($($arg_name,)*) = Arg::ITEM;
                    $fun_name::<$gen_name>($($arg_name,)*)
                };
            }
        };
    };
    ($(#[$attr:meta])* $vis:vis fn $fun_name:ident<$generic:ident $(: $bound:path)?> ($( $arg_name:tt : $arg_type:ty ),* $(,)?) -> $ret_type:ty $body:block) => {
        /// `ConstFn` version of
        #[doc = concat!("[`", stringify!($fun_name), "`](fn@", stringify!($fun_name), ")")]
        #[allow(non_camel_case_types)]
        $vis struct $fun_name<$generic $(:$bound)?> { phantom: ::core::marker::PhantomData<$generic> }

        $(#[$attr])*
        $vis const fn $fun_name<$generic $(:$bound)?>($( $arg_name : $arg_type ),*) -> $ret_type $body
        const _: () = {
            impl<$generic $(:$bound)?> $crate::ConstFn<($($arg_type,)*), $ret_type> for $fun_name<$generic> {
                type Body<Arg: $crate::Contains<($($arg_type,)*)>> = Body<Arg, $generic>;
            }
            $vis struct Body<Arg: $crate::Contains<($($arg_type,)*)>, $generic $(:$bound)?>(::std::marker::PhantomData<Arg>, ::std::marker::PhantomData<$generic>);
            impl<Arg: $crate::Contains<($($arg_type,)*)>, $generic $(:$bound)?> $crate::Contains<$ret_type> for Body<Arg, $generic> {
                const ITEM: $ret_type = {
                    let ($($arg_name,)*) = Arg::ITEM;
                    $fun_name::<$generic>($($arg_name,)*)
                };
            }
        };
    };
}

mod wrappers {
    macro_rules! impl_wrappers {
        ($( $wrapper:ident: $typ:ty ),+$(,)?) => {$(
            /// Type which provides a simple implementation for
            #[doc = concat!("[`Contains<", stringify!($typ), ">.`]")]
            pub struct $wrapper<const ITEM: $typ>;
            impl<const ITEM: $typ> $crate::Contains<$typ> for $wrapper<ITEM> {
                const ITEM: $typ = ITEM;
            }
        )+};
    }
    impl_wrappers![
        I8: i8,
        I16: i16,
        I32: i32,
        I64: i64,
        Isize: isize,
        U8: u8,
        U16: u16,
        U32: u32,
        U64: u64,
        Usize: usize
    ];
}
pub use wrappers::*;

mod tuple {
    /// Implements `Contains<T>` for `C` where `T` is any tuple and `C` is a tuple
    /// whose elements implement `Contains<_>` for their corresponding element in `T`
    macro_rules! impl_tuples {
        ($( ($($C:ident : $T:ident),+) ),+$(,)?) => {$(
            impl<$($T, $C: $crate::Contains<$T>),+> $crate::Contains<($($T,)+)> for ($($C,)+) {
                const ITEM: ($($T,)+) = ($($C::ITEM,)+);
            }
        )+};
    }
    impl_tuples! [
        (C1: T1),
        (C1: T1, C2: T2),
        (C1: T1, C2: T2, C3: T3),
        (C1: T1, C2: T2, C3: T3, C4: T4),
        (C1: T1, C2: T2, C3: T3, C4: T4, C5: T5),
    ];
}
