use std::{thread, time};
use async_std::fs::File;
use async_std::io::BufReader;
use async_std::task;
use neon::prelude::*;
use async_std::prelude::*;
use rayon::prelude::*;

const LEN: usize = 7815744000;
fn read_file_async(path: String)  {

    task::block_on(async {
        let mut file = File::open(&path).await.unwrap();
        let mut br = BufReader::new(file);
        let mut buf =  vec![0u8; LEN];
        loop {
            let n =  br.read(&mut buf).await.unwrap();
            if n == 0usize {
                println!("{:?}", thread::current().id());
                break  ;
            }
        }
    });
}

fn read_files(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0).unwrap();
    let files_handler_js = js_arr_handle.to_vec(&mut cx).unwrap() ;

    files_handler_js
        .iter()
        .map(|&i|{
            let file_name  =  i.to_string(&mut cx).unwrap().value(&mut cx);
            file_name
        })
        .collect::<Vec<String>>()
        .par_iter()
        .for_each(|file_name| {
            println!("ready for read file {}",file_name);
            let start = time::Instant::now();
            read_file_async(file_name.clone());
            println!("file {} time elapsed {} ms",file_name, start.elapsed().as_millis());
        });

    Ok(cx.number(0f64))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("readFiles", read_files)?;
    Ok(())
}
