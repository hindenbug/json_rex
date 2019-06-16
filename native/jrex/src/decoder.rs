use atoms;
use rustler::types::map::map_new;
use rustler::{Encoder, Env, NifResult, Term};
use serde_json::{self, Value};

pub fn decode<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let data = args[0].decode()?;

    match serde_json::from_str(data) {
        Ok(json_data) => {
            let term = json_to_term(env, json_data);
            Ok((atoms::ok(), term).encode(env))
        }

        Err(err) => {
            let error = format!("{}", err).encode(env);
            Ok((atoms::error(), error).encode(env))
        }
    }
}

fn json_to_term<'a>(env: Env<'a>, value: Value) -> Term<'a> {
    match value {
        Value::Null => atoms::nil().encode(env),
        Value::String(s) => s.as_str().encode(env),
        Value::Number(n) => n.as_f64().encode(env),
        Value::Bool(b) => b.encode(env),
        Value::Array(arr) => {
            let terms: Vec<Term<'a>> = arr.into_iter().map(|val| json_to_term(env, val)).collect();
            terms.encode(env)
        }
        Value::Object(mut obj) => obj.iter_mut().fold(map_new(env), |map, (key, val)| {
            let key_term = key.encode(env);
            let value_term = json_to_term(env, val.take());
            map.map_put(key_term, value_term).ok().unwrap()
        }),
    }
}
