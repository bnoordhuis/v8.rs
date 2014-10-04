#![allow(non_snake_case)]
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
#[repr(C)] pub struct TryCatch;
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
    unsafe extern fn trampoline(closure: *const ||) {
        (*closure)()
    }
    unsafe {
        HandleScope_With(isolate, trampoline, &closure)
    }
}

pub fn with_try_catch(closure: |&TryCatch|) {
    unsafe extern fn trampoline(try_catch: &TryCatch,
                                closure: *const |&TryCatch|) {
        (*closure)(try_catch)
    }
    unsafe {
        TryCatch_With(trampoline, &closure)
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
    pub fn NewFromUtf8(isolate: &Isolate, data: *const u8, typ: i32,
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

impl TryCatch {
    pub fn CanContinue(&self) -> bool {
        unsafe { TryCatch_CanContinue(self) }
    }

    pub fn Exception(&self) -> Local<Value> {
        unsafe { TryCatch_Exception(self) }
    }

    pub fn HasCaught(&self) -> bool {
        unsafe { TryCatch_HasCaught(self) }
    }

    pub fn HasTerminated(&self) -> bool {
        unsafe { TryCatch_HasTerminated(self) }
    }

    pub fn ReThrow(&self) -> Local<Value> {
        unsafe { TryCatch_ReThrow(self) }
    }

    pub fn Reset(&self) {
        unsafe { TryCatch_Reset(self) }
    }

    pub fn StackTrace(&self) -> Local<Value> {
        unsafe { TryCatch_StackTrace(self) }
    }
}

impl V8 {
    pub fn Dispose() -> bool {
        return unsafe { V8_Dispose() }
    }

    pub fn Initialize() -> bool {
        return unsafe { V8_Initialize() }
    }
}

impl Value {
    pub fn BooleanValue(&self) -> bool {
        unsafe { Value_BooleanValue(self) }
    }

    pub fn Int32Value(&self) -> i32 {
        unsafe { Value_Int32Value(self) }
    }

    pub fn IntegerValue(&self) -> i64 {
        unsafe { Value_IntegerValue(self) }
    }

    pub fn IsArgumentsObject(&self) -> bool {
        unsafe { Value_IsArgumentsObject(self) }
    }

    pub fn IsArray(&self) -> bool {
        unsafe { Value_IsArray(self) }
    }

    pub fn IsArrayBuffer(&self) -> bool {
        unsafe { Value_IsArrayBuffer(self) }
    }

    pub fn IsArrayBufferView(&self) -> bool {
        unsafe { Value_IsArrayBufferView(self) }
    }

    pub fn IsBoolean(&self) -> bool {
        unsafe { Value_IsBoolean(self) }
    }

    pub fn IsBooleanObject(&self) -> bool {
        unsafe { Value_IsBooleanObject(self) }
    }

    pub fn IsDataView(&self) -> bool {
        unsafe { Value_IsDataView(self) }
    }

    pub fn IsDate(&self) -> bool {
        unsafe { Value_IsDate(self) }
    }

    pub fn IsExternal(&self) -> bool {
        unsafe { Value_IsExternal(self) }
    }

    pub fn IsFalse(&self) -> bool {
        unsafe { Value_IsFalse(self) }
    }

    pub fn IsFloat32Array(&self) -> bool {
        unsafe { Value_IsFloat32Array(self) }
    }

    pub fn IsFloat64Array(&self) -> bool {
        unsafe { Value_IsFloat64Array(self) }
    }

    pub fn IsFunction(&self) -> bool {
        unsafe { Value_IsFunction(self) }
    }

    pub fn IsGeneratorFunction(&self) -> bool {
        unsafe { Value_IsGeneratorFunction(self) }
    }

    pub fn IsGeneratorObject(&self) -> bool {
        unsafe { Value_IsGeneratorObject(self) }
    }

    pub fn IsInt16Array(&self) -> bool {
        unsafe { Value_IsInt16Array(self) }
    }

    pub fn IsInt32(&self) -> bool {
        unsafe { Value_IsInt32(self) }
    }

    pub fn IsInt32Array(&self) -> bool {
        unsafe { Value_IsInt32Array(self) }
    }

    pub fn IsInt8Array(&self) -> bool {
        unsafe { Value_IsInt8Array(self) }
    }

    pub fn IsMap(&self) -> bool {
        unsafe { Value_IsMap(self) }
    }

    pub fn IsName(&self) -> bool {
        unsafe { Value_IsName(self) }
    }

    pub fn IsNativeError(&self) -> bool {
        unsafe { Value_IsNativeError(self) }
    }

    pub fn IsNull(&self) -> bool {
        unsafe { Value_IsNull(self) }
    }

    pub fn IsNumber(&self) -> bool {
        unsafe { Value_IsNumber(self) }
    }

    pub fn IsNumberObject(&self) -> bool {
        unsafe { Value_IsNumberObject(self) }
    }

    pub fn IsObject(&self) -> bool {
        unsafe { Value_IsObject(self) }
    }

    pub fn IsPromise(&self) -> bool {
        unsafe { Value_IsPromise(self) }
    }

    pub fn IsRegExp(&self) -> bool {
        unsafe { Value_IsRegExp(self) }
    }

    pub fn IsSet(&self) -> bool {
        unsafe { Value_IsSet(self) }
    }

    pub fn IsString(&self) -> bool {
        unsafe { Value_IsString(self) }
    }

    pub fn IsStringObject(&self) -> bool {
        unsafe { Value_IsStringObject(self) }
    }

    pub fn IsSymbol(&self) -> bool {
        unsafe { Value_IsSymbol(self) }
    }

    pub fn IsSymbolObject(&self) -> bool {
        unsafe { Value_IsSymbolObject(self) }
    }

    pub fn IsTrue(&self) -> bool {
        unsafe { Value_IsTrue(self) }
    }

    pub fn IsTypedArray(&self) -> bool {
        unsafe { Value_IsTypedArray(self) }
    }

    pub fn IsUint16Array(&self) -> bool {
        unsafe { Value_IsUint16Array(self) }
    }

    pub fn IsUint32(&self) -> bool {
        unsafe { Value_IsUint32(self) }
    }

    pub fn IsUint32Array(&self) -> bool {
        unsafe { Value_IsUint32Array(self) }
    }

    pub fn IsUint8Array(&self) -> bool {
        unsafe { Value_IsUint8Array(self) }
    }

    pub fn IsUint8ClampedArray(&self) -> bool {
        unsafe { Value_IsUint8ClampedArray(self) }
    }

    pub fn IsUndefined(&self) -> bool {
        unsafe { Value_IsUndefined(self) }
    }

    pub fn IsWeakMap(&self) -> bool {
        unsafe { Value_IsWeakMap(self) }
    }

    pub fn IsWeakSet(&self) -> bool {
        unsafe { Value_IsWeakSet(self) }
    }

    pub fn NumberValue(&self) -> f64 {
        unsafe { Value_NumberValue(self) }
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
    fn String_NewFromUtf8(isolate: &Isolate, data: *const u8, typ: i32,
                          length: i32) -> Local<String>;
    fn TryCatch_CanContinue(try_catch: &TryCatch) -> bool;
    fn TryCatch_Exception(try_catch: &TryCatch) -> Local<Value>;
    fn TryCatch_HasCaught(try_catch: &TryCatch) -> bool;
    fn TryCatch_HasTerminated(try_catch: &TryCatch) -> bool;
    fn TryCatch_ReThrow(try_catch: &TryCatch) -> Local<Value>;
    fn TryCatch_Reset(try_catch: &TryCatch);
    fn TryCatch_StackTrace(try_catch: &TryCatch) -> Local<Value>;
    fn TryCatch_With(callback: unsafe extern fn(&TryCatch, *const |&TryCatch|),
                     closure: *const |&TryCatch|);
    fn V8_Dispose() -> bool;
    fn V8_Initialize() -> bool;
    fn Value_BooleanValue(that: &Value) -> bool;
    fn Value_Int32Value(that: &Value) -> i32;
    fn Value_IntegerValue(that: &Value) -> i64;
    fn Value_IsArgumentsObject(that: &Value) -> bool;
    fn Value_IsArray(that: &Value) -> bool;
    fn Value_IsArrayBuffer(that: &Value) -> bool;
    fn Value_IsArrayBufferView(that: &Value) -> bool;
    fn Value_IsBoolean(that: &Value) -> bool;
    fn Value_IsBooleanObject(that: &Value) -> bool;
    fn Value_IsDataView(that: &Value) -> bool;
    fn Value_IsDate(that: &Value) -> bool;
    fn Value_IsExternal(that: &Value) -> bool;
    fn Value_IsFalse(that: &Value) -> bool;
    fn Value_IsFloat32Array(that: &Value) -> bool;
    fn Value_IsFloat64Array(that: &Value) -> bool;
    fn Value_IsFunction(that: &Value) -> bool;
    fn Value_IsGeneratorFunction(that: &Value) -> bool;
    fn Value_IsGeneratorObject(that: &Value) -> bool;
    fn Value_IsInt16Array(that: &Value) -> bool;
    fn Value_IsInt32(that: &Value) -> bool;
    fn Value_IsInt32Array(that: &Value) -> bool;
    fn Value_IsInt8Array(that: &Value) -> bool;
    fn Value_IsMap(that: &Value) -> bool;
    fn Value_IsName(that: &Value) -> bool;
    fn Value_IsNativeError(that: &Value) -> bool;
    fn Value_IsNull(that: &Value) -> bool;
    fn Value_IsNumber(that: &Value) -> bool;
    fn Value_IsNumberObject(that: &Value) -> bool;
    fn Value_IsObject(that: &Value) -> bool;
    fn Value_IsPromise(that: &Value) -> bool;
    fn Value_IsRegExp(that: &Value) -> bool;
    fn Value_IsSet(that: &Value) -> bool;
    fn Value_IsString(that: &Value) -> bool;
    fn Value_IsStringObject(that: &Value) -> bool;
    fn Value_IsSymbol(that: &Value) -> bool;
    fn Value_IsSymbolObject(that: &Value) -> bool;
    fn Value_IsTrue(that: &Value) -> bool;
    fn Value_IsTypedArray(that: &Value) -> bool;
    fn Value_IsUint16Array(that: &Value) -> bool;
    fn Value_IsUint32(that: &Value) -> bool;
    fn Value_IsUint32Array(that: &Value) -> bool;
    fn Value_IsUint8Array(that: &Value) -> bool;
    fn Value_IsUint8ClampedArray(that: &Value) -> bool;
    fn Value_IsUndefined(that: &Value) -> bool;
    fn Value_IsWeakMap(that: &Value) -> bool;
    fn Value_IsWeakSet(that: &Value) -> bool;
    fn Value_NumberValue(that: &Value) -> f64;
}
