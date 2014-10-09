#![allow(non_snake_case)]
#![feature(macro_rules)]

extern crate libc;

use std::default::Default;
use std::fmt;
use std::mem;
use std::ptr;

#[link(name="v8")]
extern {
    fn _ZN2v87Context4ExitEv(this: Context);
    fn _ZN2v87Context5EnterEv(this: Context);
    fn _ZN2v87Context6GlobalEv(this: Context) -> Object;
    fn _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
            isolate: Isolate, extensions: *mut ExtensionConfiguration,
            global_template: ObjectTemplate, global_object: Value) -> Context;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEE14GetReturnValueEv(
            this: FunctionCallbackInfo) -> ReturnValue;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEEixEi(this: FunctionCallbackInfo,
                                                     index: i32) -> Value;
    fn _ZNK2v820FunctionCallbackInfoINS_5ValueEE10GetIsolateEv(
            this: FunctionCallbackInfo) -> Isolate;
    fn _ZN2v816FunctionTemplate11GetFunctionEv(this: FunctionTemplate)
                                               -> Function;
    fn _ZN2v816FunctionTemplate3NewEPNS_7IsolateEPFvRKNS_20FunctionCallbackInfoINS_5ValueEEEENS_6HandleIS4_EENSA_INS_9SignatureEEEi(
            isolate: Isolate, callback: Option<FunctionCallback>,
            data: Value, signature: Signature, length: i32) -> FunctionTemplate;
    fn _ZN2v811HandleScopeC1EPNS_7IsolateE(this: &mut HandleScope,
                                           isolate: Isolate);
    fn _ZN2v811HandleScopeD1Ev(this: &mut HandleScope);
    fn _ZN2v87Isolate3NewERKNS0_12CreateParamsE(params: &CreateParams)
                                                -> Isolate;
    fn _ZN2v87Isolate7DisposeEv(this: Isolate);
    fn _ZN2v87Isolate5EnterEv(this: Isolate);
    fn _ZN2v87Isolate4ExitEv(this: Isolate);
    fn _ZN2v86Locker10InitializeEPNS_7IsolateE(this: &mut Locker,
                                               isolate: Isolate);
    fn _ZN2v86Locker8IsActiveEv() -> bool;
    fn _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate: Isolate) -> bool;
    fn _ZN2v86LockerD1Ev(this: &mut Locker);
    fn _ZN2v86Number3NewEPNS_7IsolateEd(isolate: Isolate, value: f64) -> Number;
    fn _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(this: Object,
                                                 key: Value) -> Value;
    fn _ZN2v86Object3NewEPNS_7IsolateE(isolate: Isolate) -> Object;
    fn _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(this: Object, key: Value,
                                                    value: Value) -> bool;
    fn _ZN2v811ReturnValueINS_5ValueEE3SetIS1_EEvNS_6HandleIT_EE(
            this: ReturnValue, value: Value);
    fn _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
            source: String, origin: *mut ScriptOrigin) -> Script;
    fn _ZN2v86Script3RunEv(this: Script) -> Value;
    fn _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
            isolate: Isolate, data: *const u8,
            typ: NewStringType, length: i32) -> String;
    fn _ZN2v88Unlocker10InitializeEPNS_7IsolateE(this: &mut Unlocker,
                                                 isolate: Isolate);
    fn _ZN2v88UnlockerD1Ev(this: &mut Unlocker);
    fn _ZN2v82V810InitializeEv() -> bool;
    fn _ZN2v82V87DisposeEv() -> bool;
    fn _ZNK2v85Value10IsDataViewEv(this: Value) -> bool;
    fn _ZNK2v85Value10IsExternalEv(this: Value) -> bool;
    fn _ZNK2v85Value10IsFunctionEv(this: Value) -> bool;
    fn _ZNK2v85Value11IsInt8ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsInt16ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsInt32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsTypedArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value12IsUint8ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsArrayBufferEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsNativeErrorEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsUint16ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value13IsUint32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsFloat32ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsFloat64ArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsNumberObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsStringObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value14IsSymbolObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value15IsBooleanObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsArgumentsObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsArrayBufferViewEv(this: Value) -> bool;
    fn _ZNK2v85Value17IsGeneratorObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value19IsGeneratorFunctionEv(this: Value) -> bool;
    fn _ZNK2v85Value19IsUint8ClampedArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value5IsMapEv(this: Value) -> bool;
    fn _ZNK2v85Value5IsSetEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsDateEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsNameEv(this: Value) -> bool;
    fn _ZNK2v85Value6IsTrueEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsArrayEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsFalseEv(this: Value) -> bool;
    fn _ZNK2v85Value7IsInt32Ev(this: Value) -> bool;
    fn _ZNK2v85Value8IsNumberEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsObjectEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsRegExpEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsStringEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsSymbolEv(this: Value) -> bool;
    fn _ZNK2v85Value8IsUint32Ev(this: Value) -> bool;
    fn _ZNK2v85Value9IsBooleanEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsPromiseEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsWeakMapEv(this: Value) -> bool;
    fn _ZNK2v85Value9IsWeakSetEv(this: Value) -> bool;
    fn _ZNK2v85Value11QuickIsNullEv(this: Value) -> bool;
    fn _ZNK2v85Value16QuickIsUndefinedEv(this: Value) -> bool;
    fn _ZNK2v85Value11NumberValueEv(this: Value) -> f64;
}

pub trait ValueT {
    fn as_val(&self) -> Value;
}

macro_rules! data_methods(
    ($ty:ident) => (
        impl $ty {
            #[inline(always)]
            fn raw_ptr(&self) -> *mut $ty {
                match *self { $ty(that) => that }
            }

            #[allow(dead_code)]
            fn to_option(&self) -> Option<$ty> {
                match *self {
                    $ty(that) if that.is_null() => None,
                    that => Some(that),
                }
            }
        }

        impl Default for $ty {
            fn default() -> $ty {
                $ty(ptr::null_mut())
            }
        }

        impl PartialEq for $ty {
            fn eq(&self, that: &$ty) -> bool {
                self.raw_ptr() == that.raw_ptr()
            }
        }

        impl fmt::Show for $ty {
            // TODO(bnoordhuis) Maybe specialize for SMIs and strings.
            // Maybe ToString() objects?
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "{}({:p})", stringify!($ty),
                       match *self { $ty(val) => val })
            }
        }
    );
)

macro_rules! value_methods(
    ($ty:ident) => (
        data_methods!($ty)

        impl $ty {
            #[inline(always)]
            pub fn IsArgumentsObject(&self) -> bool {
                unsafe { _ZNK2v85Value17IsArgumentsObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArray(&self) -> bool {
                unsafe { _ZNK2v85Value7IsArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArrayBuffer(&self) -> bool {
                unsafe { _ZNK2v85Value13IsArrayBufferEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsArrayBufferView(&self) -> bool {
                unsafe { _ZNK2v85Value17IsArrayBufferViewEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsBoolean(&self) -> bool {
                unsafe { _ZNK2v85Value9IsBooleanEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsBooleanObject(&self) -> bool {
                unsafe { _ZNK2v85Value15IsBooleanObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsDataView(&self) -> bool {
                unsafe { _ZNK2v85Value10IsDataViewEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsDate(&self) -> bool {
                unsafe { _ZNK2v85Value6IsDateEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsExternal(&self) -> bool {
                unsafe { _ZNK2v85Value10IsExternalEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFalse(&self) -> bool {
                unsafe { _ZNK2v85Value7IsFalseEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFloat32Array(&self) -> bool {
                unsafe { _ZNK2v85Value14IsFloat32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFloat64Array(&self) -> bool {
                unsafe { _ZNK2v85Value14IsFloat64ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsFunction(&self) -> bool {
                unsafe { _ZNK2v85Value10IsFunctionEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsGeneratorFunction(&self) -> bool {
                unsafe { _ZNK2v85Value19IsGeneratorFunctionEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsGeneratorObject(&self) -> bool {
                unsafe { _ZNK2v85Value17IsGeneratorObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt16Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsInt16ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt32(&self) -> bool {
                unsafe { _ZNK2v85Value7IsInt32Ev(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt32Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsInt32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsInt8Array(&self) -> bool {
                unsafe { _ZNK2v85Value11IsInt8ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsMap(&self) -> bool {
                unsafe { _ZNK2v85Value5IsMapEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsName(&self) -> bool {
                unsafe { _ZNK2v85Value6IsNameEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNativeError(&self) -> bool {
                unsafe { _ZNK2v85Value13IsNativeErrorEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNull(&self) -> bool {
                // FIXME(bnoordhuis) Use inline heap object tag check.
                unsafe { _ZNK2v85Value11QuickIsNullEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNumber(&self) -> bool {
                unsafe { _ZNK2v85Value8IsNumberEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsNumberObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsNumberObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsObject(&self) -> bool {
                unsafe { _ZNK2v85Value8IsObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsPromise(&self) -> bool {
                unsafe { _ZNK2v85Value9IsPromiseEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsRegExp(&self) -> bool {
                unsafe { _ZNK2v85Value8IsRegExpEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSet(&self) -> bool {
                unsafe { _ZNK2v85Value5IsSetEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsString(&self) -> bool {
                unsafe { _ZNK2v85Value8IsStringEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsStringObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsStringObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSymbol(&self) -> bool {
                unsafe { _ZNK2v85Value8IsSymbolEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsSymbolObject(&self) -> bool {
                unsafe { _ZNK2v85Value14IsSymbolObjectEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsTrue(&self) -> bool {
                unsafe { _ZNK2v85Value6IsTrueEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsTypedArray(&self) -> bool {
                unsafe { _ZNK2v85Value12IsTypedArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint16Array(&self) -> bool {
                unsafe { _ZNK2v85Value13IsUint16ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint32(&self) -> bool {
                unsafe { _ZNK2v85Value8IsUint32Ev(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint32Array(&self) -> bool {
                unsafe { _ZNK2v85Value13IsUint32ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint8Array(&self) -> bool {
                unsafe { _ZNK2v85Value12IsUint8ArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUint8ClampedArray(&self) -> bool {
                unsafe { _ZNK2v85Value19IsUint8ClampedArrayEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsUndefined(&self) -> bool {
                // FIXME(bnoordhuis) Use inline heap object tag check.
                unsafe { _ZNK2v85Value16QuickIsUndefinedEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsWeakMap(&self) -> bool {
                unsafe { _ZNK2v85Value9IsWeakMapEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn IsWeakSet(&self) -> bool {
                unsafe { _ZNK2v85Value9IsWeakSetEv(self.as_val()) }
            }
            #[inline(always)]
            pub fn NumberValue(&self) -> f64 {
                unsafe { _ZNK2v85Value11NumberValueEv(self.as_val()) }
            }
            #[inline(always)]
            fn as_val(&self) -> Value {
                Value(unsafe { mem::transmute(*self) })
            }
        }

        impl ValueT for $ty {
            #[inline(always)]
            fn as_val(&self) -> Value {
                Value(unsafe { mem::transmute(*self) })
            }
        }
    );
)

#[repr(C)]
pub struct ExtensionConfiguration;

#[repr(C)]
pub struct CreateParams {
    entry_hook: *const u8,
    code_event_handler: *const u8,
    constraints: ResourceConstraints,
    enable_serializer: bool,
}

impl Default for CreateParams {
    fn default() -> CreateParams {
        CreateParams {
            entry_hook: 0 as *const u8,
            code_event_handler: 0 as *const u8,
            constraints: Default::default(),
            enable_serializer: false,
        }
    }
}

#[repr(C)]
pub struct Context(*mut Context);

data_methods!(Context)

impl Context {
    pub fn Enter(&self) {
        unsafe { _ZN2v87Context5EnterEv(*self) }
    }

    pub fn Exit(&self) {
        unsafe { _ZN2v87Context4ExitEv(*self) }
    }

    pub fn Global(&self) -> Option<Object> {
        unsafe { _ZN2v87Context6GlobalEv(*self) }.to_option()
    }

    pub fn New(isolate: Isolate) -> Option<Context> {
        unsafe {
            _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
                    isolate, ptr::null_mut(), Default::default(),
                    Default::default())
        }.to_option()
    }
}

// Can't use a RAII type for Context::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_context_scope<T>(context: Context, closure: || -> T) -> T {
    context.Enter();
    let rval = closure();
    context.Exit();
    rval
}

#[repr(C)]
pub struct Function(*mut Function);

value_methods!(Function)

#[repr(C)]
pub type FunctionCallback = extern fn(FunctionCallbackInfo);

#[repr(C)]
pub struct FunctionCallbackInfo(*mut FunctionCallbackInfo);

impl FunctionCallbackInfo {
    pub fn At(&self, index: i32) -> Value {
        unsafe { _ZNK2v820FunctionCallbackInfoINS_5ValueEEixEi(*self, index) }
    }

    pub fn GetIsolate(&self) -> Isolate {
        unsafe { _ZNK2v820FunctionCallbackInfoINS_5ValueEE10GetIsolateEv(*self) }
    }

    pub fn GetReturnValue(&self) -> ReturnValue {
        unsafe {
            _ZNK2v820FunctionCallbackInfoINS_5ValueEE14GetReturnValueEv(*self)
        }
    }
}

#[repr(C)]
pub struct FunctionTemplate(*mut FunctionTemplate);

data_methods!(FunctionTemplate)

impl FunctionTemplate {
    pub fn GetFunction(&self) -> Option<Function> {
        unsafe { _ZN2v816FunctionTemplate11GetFunctionEv(*self) }.to_option()
    }

    pub fn New(isolate: Isolate, callback: Option<FunctionCallback>,
               data: Option<Value>, signature: Option<Signature>,
               length: i32) -> Option<FunctionTemplate> {
        let data = data.unwrap_or(Default::default());
        let signature = signature.unwrap_or(Default::default());
        unsafe {
            _ZN2v816FunctionTemplate3NewEPNS_7IsolateEPFvRKNS_20FunctionCallbackInfoINS_5ValueEEEENS_6HandleIS4_EENSA_INS_9SignatureEEEi(
                    isolate, callback, data, signature, length)
        }.to_option()
    }
}

#[repr(C)]
struct HandleScope([*mut u8, ..3]);

pub fn with_handle_scope<T>(isolate: Isolate, closure: || -> T) -> T {
    let null = ptr::null_mut();
    let mut this: HandleScope = HandleScope([null, null, null]);
    unsafe { _ZN2v811HandleScopeC1EPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v811HandleScopeD1Ev(&mut this) };
    rval
}

#[repr(C)]
pub struct Isolate(*mut Isolate);

impl Isolate {
    pub fn Dispose(&mut self) {
        unsafe { _ZN2v87Isolate7DisposeEv(*self); }
        *self = Isolate(ptr::null_mut());
    }

    pub fn Enter(&self) {
        unsafe { _ZN2v87Isolate5EnterEv(*self) }
    }

    pub fn Exit(&self) {
        unsafe { _ZN2v87Isolate4ExitEv(*self) }
    }

    pub fn New(_: Option<CreateParams>) -> Option<Isolate> {
        let params = Default::default();
        match unsafe { _ZN2v87Isolate3NewERKNS0_12CreateParamsE(&params) } {
            Isolate(that) if that.is_null() => None,
            this => Some(this),
        }
    }
}

// Can't use a RAII type for Isolate::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_isolate_scope<T>(isolate: Isolate, closure: || -> T) -> T {
    isolate.Enter();
    let rval = closure();
    isolate.Exit();
    rval
}

#[repr(C)]
pub struct Locker([*mut u8, ..3]);

impl Locker {
    pub fn IsActive() -> bool {
        unsafe { _ZN2v86Locker8IsActiveEv() }
    }

    pub fn IsLocked(isolate: Isolate) -> bool {
        unsafe { _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate) }
    }
}

pub fn with_locker<T>(isolate: Isolate, closure: || -> T) -> T {
    let null = ptr::null_mut();
    let mut this = Locker([null, null, null]);
    unsafe { _ZN2v86Locker10InitializeEPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v86LockerD1Ev(&mut this) };
    rval
}

#[repr(C)]
pub struct Number(*mut Number);

value_methods!(Number)

impl Number {
    pub fn New(isolate: Isolate, value: f64) -> Option<Number> {
        unsafe { _ZN2v86Number3NewEPNS_7IsolateEd(isolate, value) }.to_option()
    }
}

#[repr(C)]
pub struct Object(*mut Object);

value_methods!(Object)

impl Object {
    pub fn Get<K: ValueT>(&self, key: K) -> Option<Value> {
        unsafe {
            _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(*self, key.as_val())
        }.to_option()
    }

    pub fn New(isolate: Isolate) -> Option<Object> {
        unsafe { _ZN2v86Object3NewEPNS_7IsolateE(isolate) }.to_option()
    }

    pub fn Set<K: ValueT, V: ValueT>(&self, key: K, value: V) -> bool {
        unsafe {
            _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(
                    *self, key.as_val(), value.as_val())
        }
    }
}

#[repr(C)]
pub struct ObjectTemplate(*mut ObjectTemplate);

impl Default for ObjectTemplate {
    fn default() -> ObjectTemplate {
        ObjectTemplate(ptr::null_mut())
    }
}

#[repr(C)]
pub struct ResourceConstraints {
    max_semi_space_size: i32,
    max_old_space_size: i32,
    max_executable_size: i32,
    stack_limit: *mut u32,
    max_available_threads: i32,
    code_range_size: libc::size_t,
}

impl Default for ResourceConstraints {
    fn default() -> ResourceConstraints {
        ResourceConstraints {
            max_semi_space_size: 0,
            max_old_space_size: 0,
            max_executable_size: 0,
            stack_limit: ptr::null_mut(),
            max_available_threads: 0,
            code_range_size: 0,
        }
    }
}

#[repr(C)]
pub struct ReturnValue(*mut ReturnValue);

impl ReturnValue {
    pub fn Set<T: ValueT>(&self, value: T) {
        unsafe {
            _ZN2v811ReturnValueINS_5ValueEE3SetIS1_EEvNS_6HandleIT_EE(
                    *self, value.as_val())
        }
    }
}

#[repr(C)]
pub struct Script(*mut Script);

data_methods!(Script)

impl Script {
    pub fn Compile(source: String,
                   origin: Option<ScriptOrigin>) -> Option<Script> {
        let _ = origin;
        unsafe {
            _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
                    source, ptr::null_mut())
        }.to_option()
    }

    pub fn Run(&self) -> Option<Value> {
        unsafe { _ZN2v86Script3RunEv(*self) }.to_option()
    }
}

#[repr(C)]
pub struct ScriptOrigin;

#[repr(C)]
pub struct Signature(*mut Signature);

data_methods!(Signature)

#[repr(C)]
pub struct String(*mut String);

value_methods!(String)

impl String {
    pub fn NewFromUtf8(isolate: Isolate, data: &str,
                       typ: NewStringType) -> Option<String> {
        unsafe {
            _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
                    isolate, data.as_ptr(), typ, data.len() as i32)
        }.to_option()
    }
}

#[repr(C)]
#[deriving(Show)]
pub enum NewStringType {
    kNormalString,
    kInternalizedString,
    kUndetectableString,
}

#[repr(C)]
struct Unlocker(*mut u8);

pub fn with_unlocker<T>(isolate: Isolate, closure: || -> T) -> T {
    let mut this = Unlocker(ptr::null_mut());
    unsafe { _ZN2v88Unlocker10InitializeEPNS_7IsolateE(&mut this, isolate) };
    let rval = closure();
    unsafe { _ZN2v88UnlockerD1Ev(&mut this) };
    rval
}

#[repr(C)]
pub struct V8(*mut V8);

impl V8 {
    pub fn Initialize() -> bool {
        unsafe { _ZN2v82V810InitializeEv() }
    }

    pub fn Dispose() -> bool {
        unsafe { _ZN2v82V87DisposeEv() }
    }
}

#[repr(C)]
pub struct Value(*mut Value);

value_methods!(Value)
