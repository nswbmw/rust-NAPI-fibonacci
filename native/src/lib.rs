#![feature(link_args)]
#[macro_use(napi_module)]
extern crate node_api;

napi_module!("fibonacci", register);

use node_api::{NapiEnv, NapiValue, FromNapiValues, IntoNapiValue};
use node_api::{create_function, set_named_property};

#[no_mangle]
pub extern "C" fn register(env: NapiEnv,
                           exports: NapiValue,
                           _module: NapiValue,
                           _priv: *mut std::os::raw::c_void) {
    let fib = create_function(env, "fib", |_: NapiEnv, _: NapiValue, arg: f64| {
      fibonacci(arg as i32)
    }).expect("error creating function");
    set_named_property(env, exports, "fib", fib).expect("error attaching function");
}

fn fibonacci(n: i32) -> i32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}
