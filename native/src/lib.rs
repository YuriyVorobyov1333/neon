use neon::prelude::*;
use neon::register_module;
// use num_cpus;

// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node"))
// }

// fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
//     Ok(cx.number(num_cpus::get() as f64))
// }

// register_module!(mut m, { m.export_function("threadCount", thread_count) });

fn convert_vec_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    let vec: Vec<String> = Vec::with_capacity(100);

    // Create the JS array
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    vec.iter().enumerate().for_each(|e| {
        let (i, obj) = e;
        let js_string = cx.string(obj);
        let _ = js_array.set(&mut cx, i as u32, js_string);
    });

    Ok(js_array)
}

fn convert_js_array_to_vec(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Take the first argument, which must be an array
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    // Convert a JsArray to a Rust Vec
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    // Return the length of the Vec to JS
    Ok(cx.number(vec.len() as f64))
}

fn return_empty_js_array(mut cx: FunctionContext) -> JsResult<JsArray> {
    Ok(cx.empty_array())
}

fn return_js_array_with_number(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let n = cx.number(9000.0);
    array.set(&mut cx, 0, n)?;
    Ok(array)
}

fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
    let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
    let s = cx.string("hello node");
    array.set(&mut cx, 0, s)?;
    Ok(array)
}

fn make_an_array(mut cx: FunctionContext) -> JsResult<JsArray> {
  // Create some values:
  let n = cx.number(9000);
  let s = cx.string("hello");
  let b = cx.boolean(true);
  // Create a new array:
  let array: Handle<JsArray> = cx.empty_array();
  // Push the values into the array:
  array.set(&mut cx, 0, n)?;
  array.set(&mut cx, 1, s)?;
  array.set(&mut cx, 2, b)?;
  // Return the array:
  Ok(array)
}

register_module!(mut m, {
    m.export_function("convertVecToArray", convert_vec_to_array)?;
    m.export_function("convertJsArrayToVec", convert_js_array_to_vec)?;
    m.export_function("returnJsArray", return_empty_js_array)?;
    m.export_function("returnJsArrayWithNumber", return_js_array_with_number)?;
    m.export_function("returnJsArrayWithString", return_js_array_with_string)?;
    m.export_function("makeAnArray", make_an_array)?;
    Ok(())
});