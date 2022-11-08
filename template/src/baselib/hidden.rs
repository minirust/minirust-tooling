use std::rc::Rc;
use crate::baselib::Nondet;

pub fn deref_rc<T: Clone>(rc: &Rc<T>) -> T {
    (**rc).clone()
}

pub trait MonadicReturn<T> {
	fn monadic_return(t: T) -> Self;
}

pub fn ret<I, O: MonadicReturn<I>>(i: I) -> O {
	O::monadic_return(i)
}

impl<T> MonadicReturn<T> for T { fn monadic_return(t: T) -> Self { t } }
impl<T> MonadicReturn<T> for Option<T> { fn monadic_return(t: T) -> Self { Some(t) } }
impl<T, E> MonadicReturn<T> for Result<T, E> { fn monadic_return(t: T) -> Self { Ok(t) } }
impl<T> MonadicReturn<T> for Nondet<T> { fn monadic_return(t: T) -> Self { Nondet(t) } }
impl<T, E> MonadicReturn<T> for Nondet<Result<T, E>> { fn monadic_return(t: T) -> Self { Nondet(Ok(t)) } }

#[test]
fn monadic_return_test() {
	let _: i32 = ret(5);
	let _: Option<i32> = ret(5);
	let _: Result<i32, ()> = ret(5);
	let _: Nondet<i32> = ret(5);
	let _: Nondet<Result<i32, ()>> = ret(5);
}

