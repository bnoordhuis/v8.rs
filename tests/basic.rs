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

fn with_isolate_and_context(closure: |&v8::Isolate, &v8::Context|) {
    assert!(v8::V8::Initialize());
    {
        let isolate = v8::Isolate::New(None).unwrap();
        v8::with_locker(&isolate, || {
            v8::with_handle_scope(&isolate, || {
                v8::with_isolate_scope(&isolate, || {
                    let context = v8::Context::New(&isolate).unwrap();
                    v8::with_context_scope(&context, || {
                        closure(&isolate, &context)
                    });
                });
            });
        });
        drop(isolate);
    }
    // Don't call v8::V8::Dispose(), it permanently tears down V8.
}
