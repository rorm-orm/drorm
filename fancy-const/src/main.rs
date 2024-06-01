use fancy_const::*;

const_fn! {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
const_fn! {
    pub fn increment(x: i32) -> i32 {
        add(x, 1)
    }
}
const_fn! {
    fn unpack([elem]: [i32; 1]) -> i32 {
        elem
    }
}
const_fn! {
    fn foo<T: Contains<i32>>() -> i32 {
        <T as Contains<i32>>::ITEM
    }
}

fn main() {
    type One = I32<1>;
    type Two = I32<2>;
    println!("{}", <<increment as ConstFn<_, _>>::Body<(One,)>>::ITEM);
    println!("{}", <<add as ConstFn<_, _>>::Body<(One, Two)>>::ITEM);
}
