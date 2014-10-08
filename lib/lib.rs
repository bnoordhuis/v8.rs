#![allow(non_snake_case)]
#![feature(macro_rules)]

extern crate libc;

use std::default::Default;
use std::mem;
use std::ptr;

#[link(name="v8")]
extern {
    fn _ZN2v87Context5EnterEv(this: *mut Context) -> ();
    fn _ZN2v87Context4ExitEv(this: *mut Context) -> ();
    fn _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
            isolate: *mut Isolate, extensions: *mut ExtensionConfiguration,
            global_template: *mut ObjectTemplate, global_object: *mut Value)
            -> *mut Context;
    fn _ZN2v811HandleScopeC1EPNS_7IsolateE(this: *mut HandleScope,
                                           isolate: *mut Isolate) -> ();
    fn _ZN2v811HandleScopeD1Ev(this: *mut HandleScope) -> ();
    fn _ZN2v87Isolate3NewERKNS0_12CreateParamsE(
            params: *const CreateParams) -> *mut Isolate;
    fn _ZN2v87Isolate7DisposeEv(this: *mut Isolate) -> ();
    fn _ZN2v87Isolate5EnterEv(this: *mut Isolate) -> ();
    fn _ZN2v87Isolate4ExitEv(this: *mut Isolate) -> ();
    fn _ZN2v86Locker10InitializeEPNS_7IsolateE(this: *mut Locker,
                                               isolate: *mut Isolate) -> ();
    fn _ZN2v86Locker8IsActiveEv() -> bool;
    fn _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate: *mut Isolate) -> bool;
    fn _ZN2v86LockerD1Ev(this: *mut Locker) -> ();
    fn _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(this: *mut Object,
                                                 key: *mut Value) -> *mut Value;
    fn _ZN2v86Object3NewEPNS_7IsolateE(isolate: *mut Isolate) -> *mut Object;
    fn _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(this: *mut Object,
                                                    key: *mut Value,
                                                    value: *mut Value) -> bool;
    fn _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
            source: *mut String, origin: *mut ScriptOrigin) -> *mut Script;
    fn _ZN2v86Script3RunEv(this: *mut Script) -> *mut Value;
    fn _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
            isolate: *mut Isolate, data: *const u8, typ: NewStringType,
            length: i32) -> *mut String;
    fn _ZN2v88Unlocker10InitializeEPNS_7IsolateE(this: *mut Unlocker,
                                                 isolate: *mut Isolate) -> ();
    fn _ZN2v88UnlockerD1Ev(this: *mut Unlocker) -> ();
    fn _ZN2v82V810InitializeEv() -> bool;
    fn _ZN2v82V87DisposeEv() -> bool;
    fn _ZNK2v85Value10IsDataViewEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value10IsExternalEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value10IsFunctionEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value11IsInt8ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value12IsInt16ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value12IsInt32ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value12IsTypedArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value12IsUint8ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value13IsArrayBufferEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value13IsNativeErrorEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value13IsUint16ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value13IsUint32ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value14IsFloat32ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value14IsFloat64ArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value14IsNumberObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value14IsStringObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value14IsSymbolObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value15IsBooleanObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value17IsArgumentsObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value17IsArrayBufferViewEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value17IsGeneratorObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value19IsGeneratorFunctionEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value19IsUint8ClampedArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value5IsMapEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value5IsSetEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value6IsDateEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value6IsNameEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value6IsTrueEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value7IsArrayEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value7IsFalseEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value7IsInt32Ev(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsNumberEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsObjectEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsRegExpEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsStringEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsSymbolEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value8IsUint32Ev(this: *mut Value) -> bool;
    fn _ZNK2v85Value9IsBooleanEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value9IsPromiseEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value9IsWeakMapEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value9IsWeakSetEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value11QuickIsNullEv(this: *mut Value) -> bool;
    fn _ZNK2v85Value16QuickIsUndefinedEv(this: *mut Value) -> bool;
}

pub trait ValueT {
    fn inner(&self) -> *mut Value;
}

macro_rules! value_methods(
    ($ty:ident) => (
        impl $ty {
            #[inline(always)]
            pub fn IsArgumentsObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value17IsArgumentsObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsArray(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value7IsArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsArrayBuffer(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value13IsArrayBufferEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsArrayBufferView(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value17IsArrayBufferViewEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsBoolean(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value9IsBooleanEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsBooleanObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value15IsBooleanObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsDataView(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value10IsDataViewEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsDate(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value6IsDateEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsExternal(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value10IsExternalEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsFalse(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value7IsFalseEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsFloat32Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value14IsFloat32ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsFloat64Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value14IsFloat64ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsFunction(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value10IsFunctionEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsGeneratorFunction(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value19IsGeneratorFunctionEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsGeneratorObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value17IsGeneratorObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsInt16Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value12IsInt16ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsInt32(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value7IsInt32Ev(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsInt32Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value12IsInt32ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsInt8Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value11IsInt8ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsMap(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value5IsMapEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsName(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value6IsNameEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsNativeError(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value13IsNativeErrorEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsNull(&self) -> bool {
                // FIXME(bnoordhuis) Use inline heap object tag check.
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value11QuickIsNullEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsNumber(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsNumberEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsNumberObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value14IsNumberObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsPromise(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value9IsPromiseEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsRegExp(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsRegExpEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsSet(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value5IsSetEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsString(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsStringEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsStringObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value14IsStringObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsSymbol(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsSymbolEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsSymbolObject(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value14IsSymbolObjectEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsTrue(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value6IsTrueEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsTypedArray(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value12IsTypedArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUndefined(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value16QuickIsUndefinedEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUint16Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value13IsUint16ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUint32(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value8IsUint32Ev(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUint32Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value13IsUint32ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUint8Array(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value12IsUint8ArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsUint8ClampedArray(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value19IsUint8ClampedArrayEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsWeakMap(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value9IsWeakMapEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn IsWeakSet(&self) -> bool {
                match *self {
                    $ty(this) => unsafe {
                        _ZNK2v85Value9IsWeakSetEv(mem::transmute(this))
                    }
                }
            }
            #[inline(always)]
            pub fn inner(&self) -> *mut $ty {
                match *self { $ty(this) => this }
            }
        }

        impl ValueT for $ty {
            #[inline(always)]
            fn inner(&self) -> *mut Value {
                match *self {
                    $ty(this) => unsafe { mem::transmute(this) }
                }
            }
        }
    );
)

#[repr(C)]
pub struct ExtensionConfiguration;

#[repr(C)]
pub struct ObjectTemplate;

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

impl Context {
    pub fn Enter(&self) {
        match *self {
            Context(this) => unsafe { _ZN2v87Context5EnterEv(this) }
        }
    }

    pub fn Exit(&self) {
        match *self {
            Context(this) => unsafe { _ZN2v87Context4ExitEv(this) }
        }
    }

    pub fn New(isolate: &Isolate) -> Option<Context> {
        maybe(Context, match *isolate {
            Isolate(isolate) => unsafe {
                _ZN2v87Context3NewEPNS_7IsolateEPNS_22ExtensionConfigurationENS_6HandleINS_14ObjectTemplateEEENS5_INS_5ValueEEE(
                    isolate, ptr::null_mut(), ptr::null_mut(), ptr::null_mut())
            }
        })
    }
}

// Can't use a RAII type for Context::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_context_scope<T>(context: &Context, closure: || -> T) -> T {
    context.Enter();
    let rval = closure();
    context.Exit();
    rval
}

#[repr(C)]
struct HandleScope([*mut u8, ..3]);

pub fn with_handle_scope<T>(isolate: &Isolate, closure: || -> T) -> T {
    let null = ptr::null_mut();
    let mut this: HandleScope = HandleScope([null, null, null]);
    match *isolate {
        Isolate(isolate) => unsafe {
            _ZN2v811HandleScopeC1EPNS_7IsolateE(&mut this, isolate)
        }
    };
    let rval = closure();
    unsafe { _ZN2v811HandleScopeD1Ev(&mut this) };
    rval
}

#[repr(C)]
pub struct Isolate(*mut Isolate);

impl Isolate {
    pub fn Enter(&self) {
        match *self {
            Isolate(this) => unsafe { _ZN2v87Isolate5EnterEv(this) }
        }
    }

    pub fn Exit(&self) {
        match *self {
            Isolate(this) => unsafe { _ZN2v87Isolate4ExitEv(this) }
        }
    }

    pub fn New(_: Option<CreateParams>) -> Option<Isolate> {
        let params = Default::default();
        maybe(Isolate, unsafe {
            _ZN2v87Isolate3NewERKNS0_12CreateParamsE(&params)
        })
    }
}

impl Drop for Isolate {
    fn drop(&mut self) {
        match *self {
            Isolate(this) => unsafe { _ZN2v87Isolate7DisposeEv(this) }
        }
    }
}

// Can't use a RAII type for Isolate::Scope because of
// https://github.com/rust-lang/rust/issues/17858
pub fn with_isolate_scope<T>(isolate: &Isolate, closure: || -> T) -> T {
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

    pub fn IsLocked(isolate: &Isolate) -> bool {
        match *isolate {
            Isolate(isolate) => unsafe {
                _ZN2v86Locker8IsLockedEPNS_7IsolateE(isolate)
            }
        }
    }
}

pub fn with_locker<T>(isolate: &Isolate, closure: || -> T) -> T {
    let null = ptr::null_mut();
    let mut this = Locker([null, null, null]);
    match *isolate {
        Isolate(isolate) => unsafe {
            _ZN2v86Locker10InitializeEPNS_7IsolateE(&mut this, isolate)
        }
    };
    let rval = closure();
    unsafe { _ZN2v86LockerD1Ev(&mut this) };
    rval
}

#[repr(C)]
pub struct Object(*mut Object);

value_methods!(Object)

impl Object {
    pub fn Get<K: ValueT>(&self, key: K) -> Option<Value> {
        maybe(Value, unsafe {
            _ZN2v86Object3GetENS_6HandleINS_5ValueEEE(self.inner(), key.inner())
        })
    }

    pub fn New(isolate: &Isolate) -> Option<Object> {
        maybe(Object, match *isolate {
            Isolate(isolate) => unsafe {
                _ZN2v86Object3NewEPNS_7IsolateE(isolate)
            }
        })
    }

    pub fn Set<K: ValueT, V: ValueT>(&self, key: K, value: V) -> bool {
        unsafe {
            _ZN2v86Object3SetENS_6HandleINS_5ValueEEES3_(self.inner(),
                                                         key.inner(),
                                                         value.inner())
        }
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
pub struct Script(*mut Script);

impl Script {
    pub fn Compile(source: String,
                   origin: Option<ScriptOrigin>) -> Option<Script> {
        let _ = origin;
        maybe(Script, match source {
            String(source) => unsafe {
                _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
                        source, ptr::null_mut())
            }
        })
    }

    pub fn Run(&self) -> Option<Value> {
        maybe(Value, match *self {
            Script(this) => unsafe { _ZN2v86Script3RunEv(this) }
        })
    }
}

#[repr(C)]
pub struct ScriptOrigin;

#[repr(C)]
pub struct String(*mut String);

value_methods!(String)

impl String {
    pub fn NewFromUtf8(isolate: &Isolate, data: &str,
                       typ: NewStringType) -> Option<String> {
        maybe(String, match *isolate {
            Isolate(isolate) => unsafe {
                _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
                        isolate, data.as_ptr(), typ, -1)
            }
        })
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

pub fn with_unlocker<T>(isolate: &Isolate, closure: || -> T) -> T {
    let mut this = Unlocker(ptr::null_mut());
    match *isolate {
        Isolate(isolate) => unsafe {
            _ZN2v88Unlocker10InitializeEPNS_7IsolateE(&mut this, isolate)
        }
    };
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

fn maybe<T>(make: |*mut T| -> T, val: *mut T) -> Option<T> {
    match val as uint {
        0 => None,
        _ => Some(make(val)),
    }
}
