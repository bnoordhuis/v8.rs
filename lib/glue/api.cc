#pragma GCC diagnostic ignored "-Wunused-parameter"
#include "v8.h"
#pragma GCC diagnostic warning "-Wunused-parameter"

namespace {

extern "C" void Context_Enter(v8::Local<v8::Context> context) {
  return context->Enter();
}

extern "C" void Context_Exit(v8::Local<v8::Context> context) {
  return context->Exit();
}

extern "C" v8::Local<v8::Context> Context_New(v8::Isolate* isolate) {
  return v8::Context::New(isolate);
}

extern "C" void HandleScope_With(v8::Isolate* isolate, void (*callback)(void*),
                                 void* arg) {
  v8::HandleScope handle_scope(isolate);
  callback(arg);
}

extern "C" void Isolate_Dispose(v8::Isolate* isolate) {
  return isolate->Dispose();
}

extern "C" void Isolate_Enter(v8::Isolate* isolate) {
  return isolate->Enter();
}

extern "C" void Isolate_Exit(v8::Isolate* isolate) {
  return isolate->Exit();
}

extern "C" v8::Isolate* Isolate_New() {
  return v8::Isolate::New();
}

extern "C" bool V8_Dispose() {
  return v8::V8::Dispose();
}

extern "C" bool V8_Initialize() {
  return v8::V8::Initialize();
}

extern "C" v8::Local<v8::Script> Script_Compile(v8::Local<v8::String> source,
                                                v8::ScriptOrigin* origin) {
  return v8::Script::Compile(source, origin);
}

extern "C" v8::Local<v8::Value> Script_Run(v8::Local<v8::Script> script) {
  return script->Run();
}

extern "C" v8::Local<v8::String> String_NewFromUtf8(
    v8::Isolate* isolate,
    const char* data,
    v8::String::NewStringType type,
    int length) {
  return v8::String::NewFromUtf8(isolate, data, type, length);
}

extern "C" void TryCatch_With(void (*callback)(v8::TryCatch*, void*),
                              void* arg) {
  v8::TryCatch try_catch;
  callback(&try_catch, arg);
}

#define V(clazz, method, type) \
  extern "C" type clazz##_##method(v8::clazz* that) { return that->method(); }
V(TryCatch, CanContinue, bool)
V(TryCatch, Exception, v8::Local<v8::Value>)
V(TryCatch, HasCaught, bool)
V(TryCatch, HasTerminated, bool)
V(TryCatch, Message, v8::Local<v8::Message>)
V(TryCatch, ReThrow, v8::Local<v8::Value>)
V(TryCatch, Reset, void)
V(TryCatch, StackTrace, v8::Local<v8::Value>)
V(Value, BooleanValue, bool);
V(Value, Int32Value, int32_t);
V(Value, IntegerValue, int64_t);
V(Value, IsArgumentsObject, bool);
V(Value, IsArray, bool);
V(Value, IsArrayBuffer, bool);
V(Value, IsArrayBufferView, bool);
V(Value, IsBoolean, bool);
V(Value, IsBooleanObject, bool);
V(Value, IsDataView, bool);
V(Value, IsDate, bool);
V(Value, IsExternal, bool);
V(Value, IsFalse, bool);
V(Value, IsFloat32Array, bool);
V(Value, IsFloat64Array, bool);
V(Value, IsFunction, bool);
V(Value, IsGeneratorFunction, bool);
V(Value, IsGeneratorObject, bool);
V(Value, IsInt16Array, bool);
V(Value, IsInt32, bool);
V(Value, IsInt32Array, bool);
V(Value, IsInt8Array, bool);
V(Value, IsMap, bool);
V(Value, IsName, bool);
V(Value, IsNativeError, bool);
V(Value, IsNull, bool);
V(Value, IsNumber, bool);
V(Value, IsNumberObject, bool);
V(Value, IsObject, bool);
V(Value, IsPromise, bool);
V(Value, IsRegExp, bool);
V(Value, IsSet, bool);
V(Value, IsString, bool);
V(Value, IsStringObject, bool);
V(Value, IsSymbol, bool);
V(Value, IsSymbolObject, bool);
V(Value, IsTrue, bool);
V(Value, IsTypedArray, bool);
V(Value, IsUint16Array, bool);
V(Value, IsUint32, bool);
V(Value, IsUint32Array, bool);
V(Value, IsUint8Array, bool);
V(Value, IsUint8ClampedArray, bool);
V(Value, IsUndefined, bool);
V(Value, IsWeakMap, bool);
V(Value, IsWeakSet, bool);
V(Value, NumberValue, double);
V(Value, ToArrayIndex, v8::Local<v8::Uint32>);
V(Value, ToBoolean, v8::Local<v8::Boolean>);
V(Value, ToDetailString, v8::Local<v8::String>);
V(Value, ToInt32, v8::Local<v8::Int32>);
V(Value, ToInteger, v8::Local<v8::Integer>);
V(Value, ToNumber, v8::Local<v8::Number>);
V(Value, ToObject, v8::Local<v8::Object>);
V(Value, ToString, v8::Local<v8::String>);
V(Value, ToUint32, v8::Local<v8::Uint32>);
V(Value, Uint32Value, uint32_t);
#undef V

}  // namespace anonymous
