use std::io::{Error, Read};
use std::thread;
use neon::prelude::*;
use tokio::fs::File;
use tokio::fs::file::OpenFuture;
use tokio::prelude::{AsyncRead, Future};
use tokio::prelude::future::{AndThen, MapErr};


fn read_file(path: String) {

    let task=
        tokio::fs::read(path.clone())
        .map(move |mut contents| {


            println!("path {} {}", String::from_utf8(contents).unwrap(), path);
        })
        .map_err(|err| println!("IO error: {:?}", err));

    tokio::run(task);
}


fn read_files(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0).unwrap();
    let files_handler_js = js_arr_handle.to_vec(&mut cx).unwrap() ;

    for file_value_js in files_handler_js {
        let file_name  =  file_value_js.to_string(&mut cx).unwrap().value(&mut cx);
        thread::spawn(|| {
            println!("thread");
            read_file(file_name);
        });
    }
    Ok(cx.number(0f64))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("readFiles", read_files)?;
    Ok(())
}
