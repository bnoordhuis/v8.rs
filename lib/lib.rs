#![allow(non_snake_case)]
#![feature(link_args)]
extern crate libc;

use std::default::Default;
use std::ptr;

#[repr(C)]
pub struct Local<T> {
    val_: *const T,
}

#[repr(C)] pub struct Context;
#[repr(C)] pub struct Isolate;
#[repr(C)] pub struct Script;
#[repr(C)] pub struct ScriptOrigin;
#[repr(C)] pub struct String;
#[repr(C)] pub struct Value;
#[repr(C)] pub struct V8;

pub fn with_context_scope(context: Local<Context>, closure: ||) {
    (*context).Enter();
    closure();
    (*context).Exit();
}

pub fn with_isolate(closure: |&Isolate|) {
    let isolate = Isolate::New();
    closure(isolate);
    drop(isolate);
}

pub fn with_isolate_scope(isolate: &Isolate, closure: ||) {
    isolate.Enter();
    closure();
    isolate.Exit();
}

pub fn with_handle_scope(isolate: &Isolate, closure: ||) {
    unsafe extern fn invoke(closure: *const ||) { 
        (*closure)()
    }
    unsafe {
        HandleScope_With(isolate, invoke, &closure)
    }
}

impl<T> Local<T> {
    pub fn IsEmpty(&self) -> bool {
        self.val_ == ptr::null()
    }
}

impl<T> Default for Local<T> {
    fn default() -> Local<T> {
        Local { val_: ptr::null() }
    }
}

impl<T> Deref<T> for Local<T> {
    fn deref<'a>(&self) -> &T {
        unsafe { &*self.val_ }
    }
}

impl Context {
    pub fn Enter(&self) {
        unsafe { Context_Enter(self) }
    }

    pub fn Exit(&self) {
        unsafe { Context_Exit(self) }
    }

    pub fn New(isolate: &Isolate) -> Local<Context> {
        unsafe { Context_New(isolate) }
    }
}

// FIXME(bnoordhuis) Should probably implement Drop and, er, drop Dispose().
impl Isolate {
    pub fn Dispose(&mut self) {
        unsafe { Isolate_Dispose(self) }
    }

    pub fn Enter(&self) {
        unsafe { Isolate_Enter(self) }
    }

    pub fn Exit(&self) {
        unsafe { Isolate_Exit(self) }
    }

    pub fn New<'a>() -> &'a mut Isolate {
        unsafe { Isolate_New() }
    }
}

impl Drop for Isolate {
    fn drop(&mut self) {
        self.Dispose()
    }
}

impl String {
    pub fn NewFromUtf8(isolate: &Isolate, data: *const i8, typ: i32,
                       length: i32) -> Local<String> {
        unsafe { String_NewFromUtf8(isolate, data, typ, length) }
    }
}

fn to_local<T>(val: *const T) -> Local<T> {
    Local { val_: val }
}

impl Script {
    pub fn Compile(source: Local<String>,
                   _: Option<ScriptOrigin>) -> Local<Script> {
        unsafe { Script_Compile(source, ptr::null()) }
    }

    pub fn Run(&self) -> Local<Value> {
        unsafe { Script_Run(to_local(self)) }
    }
}

impl ScriptOrigin {
}

impl V8 {
    pub fn Dispose() -> bool {
        return unsafe { V8_Dispose() }
    }

    pub fn Initialize() -> bool {
        return unsafe { V8_Initialize() }
    }
}

#[link(name="glue")]
#[link(name="v8")]
#[link(name="stdc++")]  // __gxx_personality_v0@@CXXABI_1.3
extern {
    fn HandleScope_With(isolate: &Isolate,
                        callback: unsafe extern fn(*const ||),
                        closure: *const ||);
    fn Context_Enter(context: &Context);
    fn Context_Exit(context: &Context);
    fn Context_New(isolate: &Isolate) -> Local<Context>;
    fn Isolate_Dispose(isolate: &mut Isolate);
    fn Isolate_Enter(isolate: &Isolate);
    fn Isolate_Exit(isolate: &Isolate);
    fn Isolate_New() -> &mut Isolate;
    fn Script_Compile(source: Local<String>,
                      origin: *const ScriptOrigin) -> Local<Script>;
    fn Script_Run(script: Local<Script>) -> Local<Value>;
    fn String_NewFromUtf8(isolate: &Isolate, data: *const i8, typ: i32,
                          length: i32) -> Local<String>;
    fn V8_Dispose() -> bool;
    fn V8_Initialize() -> bool;
}
