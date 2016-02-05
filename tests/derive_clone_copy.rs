#![feature(plugin)]
#![plugin(parse_generics_poc)]
#[macro_use] extern crate custom_derive;

macro_rules! CloneCopy {
    (
        () $(pub)* enum $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            then CloneCopy! { @with_generics (enum $name), },
            $($tail)*
        }
    };

    (
        () $(pub)* struct $name:ident $($tail:tt)*
    ) => {
        parse_generics! {
            then CloneCopy! { @with_generics (struct $name), },
            $($tail)*
        }
    };

    (
        @with_generics
        $prefix:tt, $generics:tt,
        ($($body:tt)*)
        $($tail:tt)*
    ) => {
        parse_where! {
            then CloneCopy! { @expand $prefix, $generics, },
            $($tail)* ($($body)*)
        }
    };

    (
        @with_generics
        $prefix:tt, $generics:tt,
        $($tail:tt)*
    ) => {
        parse_where! {
            then CloneCopy! { @expand $prefix, $generics, },
            $($tail)*
        }
    };

    (
        @expand ($_kind:tt $name:ident),
        {
            constr: [$($constr:tt)*],
            ltimes: [$($ltimes:tt)*],
            params: []
            $($_more_generics:tt)*
        },
        {
            preds: []
            $($_more_preds:tt)*
        },
        $($_tail:tt)*
    ) => {
        CloneCopy! {
            @as_item
            impl<$($constr)*> Clone for $name<$($ltimes)*> {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    };

    (
        @expand ($_kind:tt $name:ident),
        {
            constr: [$($constr:tt)*],
            ltimes: [$($ltimes:tt)*],
            params: [$($params:ident,)*]
            $($_more_generics:tt)*
        },
        {
            preds: [$($preds:tt)*]
            $($_more_preds:tt)*
        },
        $($_tail:tt)*
    ) => {
        CloneCopy! {
            @as_item
            impl<$($constr)*> Clone for $name<$($ltimes)* $($params)*>
            where $($params: Copy,)* $($preds)* {
                fn clone(&self) -> Self {
                    *self
                }
            }
        }
    };

    (@as_item $i:item) => { $i };
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    struct Type0(u32);
}

#[test]
fn test_type_0() {
    let v = Type0(42);
    let (a, b) = (v, v);
    assert_eq!(a, b);
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    struct Type1<T> { value: T }
}

#[test]
fn test_type_1() {
    let v = Type1 { value: 42 };
    let (a, b) = (v, v);
    assert_eq!(a, b);
}

custom_derive! {
    #[derive(Copy, CloneCopy, Eq, PartialEq, Debug)]
    enum Type2<T> where T: Ord { A(T) }
}

#[test]
fn test_type_2() {
    let v = Type2::A(42);
    let (a, b) = (v, v);
    assert_eq!(a, b);
}
