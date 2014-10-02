extern crate v8;

#[test]
fn smoketest() {
    assert!(v8::V8::Initialize());
    v8::with_isolate(|isolate| {
        v8::with_isolate_scope(isolate, || {
            v8::with_handle_scope(isolate, || {
                let context = v8::Context::New(isolate);
                v8::with_context_scope(context, || {
                    let raw_source = "42*42".to_c_str().as_ptr();
                    let source =
                            v8::String::NewFromUtf8(isolate, raw_source, 0, -1);
                    let script = v8::Script::Compile(source, None);
                    let result = script.Run();
                });
            });
        });
    });
    assert!(v8::V8::Dispose());
}
