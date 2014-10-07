#![allow(non_snake_case)]

extern crate libc;

use std::default::Default;
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
    fn _ZN2v86Script7CompileENS_6HandleINS_6StringEEEPNS_12ScriptOriginE(
            source: *mut String, origin: *mut ScriptOrigin) -> *mut Script;
    fn _ZN2v86Script3RunEv(this: *mut Script) -> *mut Value;
    fn _ZN2v86String11NewFromUtf8EPNS_7IsolateEPKcNS0_13NewStringTypeEi(
            isolate: *mut Isolate, data: *const u8, typ: NewStringType,
            length: i32) -> *mut String;
    fn _ZN2v82V810InitializeEv() -> bool;
    fn _ZN2v82V87DisposeEv() -> bool;
    fn _ZNK2v85Value8IsNumberEv(this: *mut Value) -> bool;
}

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

#[repr(C)]
#[deriving(Show)]
pub enum NewStringType {
    kNormalString,
    kInternalizedString,
    kUndetectableString,
}

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

impl Value {
    pub fn IsNumber(&self) -> bool {
        match *self {
            Value(this) => unsafe { _ZNK2v85Value8IsNumberEv(this) }
        }
    }
}

fn maybe<T>(make: |*mut T| -> T, val: *mut T) -> Option<T> {
    match val as uint {
        0 => None,
        _ => Some(make(val)),
    }
}
