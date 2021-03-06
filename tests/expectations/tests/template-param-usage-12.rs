/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseUsesT<T> {
    pub t: *mut T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for BaseUsesT<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CrtpUsesU<U> {
    pub _base: BaseUsesT<CrtpUsesU<U>>,
    pub usage: U,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl <U> Default for CrtpUsesU<U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
