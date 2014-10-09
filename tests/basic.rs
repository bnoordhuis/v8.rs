extern crate v8;

#[test]
fn up_and_down() {
    with_isolate_and_context(|_, _| {});
    with_isolate_and_context(|_, _| {});
}

#[test]
fn lock_and_unlock() {
    with_isolate_and_context(|isolate, _| {
        assert!(v8::Locker::IsActive());
        assert!(v8::Locker::IsLocked(isolate));
        v8::with_unlocker(isolate, || {
            assert!(!v8::Locker::IsLocked(isolate));
            v8::with_locker(isolate, || {
                assert!(v8::Locker::IsLocked(isolate));
            });
            assert!(!v8::Locker::IsLocked(isolate));
        });
        assert!(v8::Locker::IsLocked(isolate));
    });
}

#[test]
fn fortytwo() {
    with_isolate_and_context(|isolate, _| {
        let source = v8::String::NewFromUtf8(isolate, "42",
                                             v8::kNormalString).unwrap();
        assert!(source.IsString());
        assert!(!source.IsStringObject());
        let script = v8::Script::Compile(source, None).unwrap();
        let result = script.Run().unwrap();
        assert!(!result.IsNull());
        assert!(result.IsNumber());
        assert!(!result.IsUndefined());
    });
}

#[test]
fn basic_number() {
    with_isolate_and_context(|isolate, _| {
        let val = 13.37;
        let num = v8::Number::New(isolate, val).unwrap();
        assert_eq!(val, num.NumberValue());
    });
}

#[test]
fn basic_object() {
    with_isolate_and_context(|isolate, _| {
        let object = v8::Object::New(isolate).unwrap();
        assert!(object.IsObject());
        let key = v8::String::NewFromUtf8(isolate, "the_key",
                                          v8::kNormalString).unwrap();
        let value = v8::String::NewFromUtf8(isolate, "the_value",
                                            v8::kNormalString).unwrap();
        assert!(object.Set(key, value));
        assert!(object.Get(key).unwrap().IsString());
    });
}

#[test]
fn global_object() {
    with_isolate_and_context(|isolate, context| {
        let key = v8::String::NewFromUtf8(isolate, "Object",
                                          v8::kNormalString).unwrap();
        assert!(context.Global().unwrap().Get(key).unwrap().IsObject());
    });
}

#[test]
fn native_api_call() {
    with_isolate_and_context(|isolate, context| {
        extern fn f(info: v8::FunctionCallbackInfo) {
            let isolate = info.GetIsolate();
            assert!(info.At(0).IsNumber());
            assert!(info.At(1).IsNumber());
            let a = info.At(0).NumberValue();
            let b = info.At(1).NumberValue();
            let x = v8::Number::New(isolate, a * b).unwrap();
            info.GetReturnValue().Set(x);
        }
        let t = v8::FunctionTemplate::New(isolate, Some(f),
                                          None, None, 0).unwrap();
        let key = v8::String::NewFromUtf8(isolate, "f",
                                          v8::kNormalString).unwrap();
        let global = context.Global().unwrap();
        global.Set(key, t.GetFunction().unwrap());
        assert!(global.Get(key).unwrap().IsFunction());
        let source = v8::String::NewFromUtf8(isolate, "f(1337, 42)",
                                             v8::kNormalString).unwrap();
        let script = v8::Script::Compile(source, None).unwrap();
        let result = script.Run().unwrap();
        assert!(result.IsNumber());
        assert_eq!(result.NumberValue(), 1337. * 42.);
    });
}

fn with_isolate_and_context(closure: |v8::Isolate, v8::Context|) {
    assert!(v8::V8::Initialize());
    {
        let mut isolate = v8::Isolate::New(None).unwrap();
        v8::with_locker(isolate, || {
            v8::with_handle_scope(isolate, || {
                v8::with_isolate_scope(isolate, || {
                    let context = v8::Context::New(isolate).unwrap();
                    v8::with_context_scope(context, || {
                        closure(isolate, context)
                    });
                });
            });
        });
        isolate.Dispose();
    }
    // Don't call v8::V8::Dispose(), it permanently tears down V8.
}
