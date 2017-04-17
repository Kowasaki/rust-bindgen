/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo<T> {
    pub t_member: T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for Foo<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub member: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quux<V> {
    pub v_member: V,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<V>>,
}
impl <V> Default for Quux<V> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Lobo {
    pub also_member: ::std::os::raw::c_char,
}
pub type AliasWithAnonType = ::std::os::raw::c_char;
