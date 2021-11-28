use crate::err::{Error, ErrorKind};
use crate::policy::condition::Condition;
use rusty_v8 as v8;
use serde_json::Value;
use std::convert::TryFrom;
use std::sync::Once;

const V8_INIT: Once = Once::new();

macro_rules! wrap_script {
    ($code: expr) => {{
        format!(
            r#"
!!(function () {{
{}
}})();
"#,
            $code
        )
    }};
}

struct Processor<'s, 'i> {
    context: v8::Local<'s, v8::Context>,
    context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
}

impl<'s, 'i> Processor<'s, 'i>
where
    's: 'i,
{
    fn new(isolate_scope: &'i mut v8::HandleScope<'s, ()>) -> Self {
        V8_INIT.call_once(|| {
            let p = v8::new_default_platform().unwrap();
            v8::V8::initialize_platform(p);
            v8::V8::initialize();
        });

        let global = v8::ObjectTemplate::new(isolate_scope);
        let context = v8::Context::new_from_template(isolate_scope, global);
        let context_scope = v8::ContextScope::new(isolate_scope, context);

        Self {
            context,
            context_scope,
        }
    }

    fn value_to_v8_object(&mut self, value: &Value) -> v8::Local<'s, v8::Value> {
        match value {
            Value::Null => v8::null(&mut self.context_scope).into(),
            Value::Bool(value) => v8::Boolean::new(&mut self.context_scope, *value).into(),
            Value::Number(num) => {
                v8::Number::new(&mut self.context_scope, num.as_f64().unwrap()).into()
            }
            Value::String(str) => v8::String::new(&mut self.context_scope, str.as_str())
                .unwrap()
                .into(),
            Value::Array(vec) => {
                let arr =
                    v8::Array::new(&mut self.context_scope, i32::try_from(vec.len()).unwrap());
                for (i, value) in vec.iter().enumerate() {
                    let value = self.value_to_v8_object(value);
                    arr.set_index(&mut self.context_scope, u32::try_from(i).unwrap(), value);
                }

                arr.into()
            }
            Value::Object(map) => {
                let obj = v8::Object::new(&mut self.context_scope);
                for (idx, value) in map.iter() {
                    let value = self.value_to_v8_object(value);
                    let key = v8::String::new(&mut self.context_scope, idx.as_str()).unwrap();
                    obj.set(&mut self.context_scope, key.into(), value);
                }

                obj.into()
            }
        }
    }

    fn execute_script(&mut self, code: &str, params: &Value) -> Result<bool, Error> {
        let source = v8::String::new(&mut self.context_scope, wrap_script!(code).as_str())
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnwrapNoneValueError,
                    "cannot create script string",
                )
            })?;

        let params = self.value_to_v8_object(params);
        let key = v8::String::new(&mut self.context_scope, "request").unwrap();
        self.context.global(&mut self.context_scope).set(
            &mut self.context_scope,
            key.into(),
            params,
        );

        let scope = &mut v8::HandleScope::new(&mut self.context_scope);
        let try_catch = &mut v8::TryCatch::new(scope);

        let script = v8::Script::compile(try_catch, source, None).ok_or_else(|| {
            Error::new(
                ErrorKind::UnwrapNoneValueError,
                "cannot create script local",
            )
        })?;
        let run_result = script.run(try_catch);

        if let Some(result) = run_result {
            let scope = &mut v8::ContextScope::new(try_catch, self.context);
            Ok(result.to_boolean(scope) == v8::Boolean::new(scope, true))
        } else {
            let exception = try_catch.exception().unwrap();
            let exception_string = exception
                .to_string(try_catch)
                .unwrap()
                .to_rust_string_lossy(try_catch);

            Err(Error::new(ErrorKind::UnknownError, exception_string))
        }
    }
}

#[inline]
pub(super) fn make_script(value: &Value) -> Result<Condition, Error> {
    value
        .as_str()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::UnwrapNoneValueError,
                "Conditions.Script value is not a string",
            )
        })
        .map(|s| Condition::Script(String::from(s)))
}

#[inline]
pub(super) fn evaluate_script(script: &str, params: &Value) -> bool {
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let mut isolate_scope = v8::HandleScope::new(&mut isolate);

    let mut processor = Processor::new(&mut isolate_scope);
    processor.execute_script(script, params).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use crate::policy::condition::initialize_v8_platform;
    use crate::policy::condition::script::evaluate_script;

    #[test]
    fn should_correctly_evaluate_script() {
        initialize_v8_platform().expect("");
        let result = evaluate_script(
            r#"
let source = request.source;
return source === 'CorrectSource';
        "#,
            &serde_json::json!({
                "source": "CorrectSource"
            }),
        );

        assert_eq!(true, result);
    }

    #[test]
    fn should_convert_returned_value_into_a_boolean() {
        initialize_v8_platform().expect("");

        let result = evaluate_script(r#"return 1;"#, &serde_json::json!({}));
        assert_eq!(true, result);

        let result = evaluate_script(r#"return 0;"#, &serde_json::json!({}));
        assert_eq!(false, result);

        let result = evaluate_script(r#"return undefined;"#, &serde_json::json!({}));
        assert_eq!(false, result);
    }

    #[test]
    fn should_return_false_in_case_of_error() {
        initialize_v8_platform().expect("");

        let result = evaluate_script(r#"throw new Error();"#, &serde_json::json!({}));
        assert_eq!(false, result);
    }
}
