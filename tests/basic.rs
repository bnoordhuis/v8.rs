extern crate v8;

#[test]
fn smoketest0() {
    assert!(v8::V8::Initialize());
    {
        let isolate = v8::Isolate::New(None).unwrap();
        v8::with_handle_scope(&isolate, || {
            v8::with_isolate_scope(&isolate, || {
                let context = v8::Context::New(&isolate).unwrap();
                v8::with_context_scope(&context, || {
                    let source =
                            v8::String::NewFromUtf8(&isolate, "42",
                                                    v8::kNormalString).unwrap();
                    assert!(source.IsString());
                    assert!(!source.IsStringObject());
                    let script = v8::Script::Compile(source, None).unwrap();
                    let result = script.Run().unwrap();
                    assert!(result.IsNumber());
                });
            });
        });
    }
    assert!(v8::V8::Dispose());
}
