use serde::{Serialize,Deserialize};
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
mod wasi_http;
use wasi_http::HttpCtx;
use serde_json::{Error, Value};

pub static  BINARY_WASM : &'static [u8]  = include_bytes!("../workflow.wasm");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input{
    allowed_hosts: Option<Vec<String>>,
}

pub fn main(args: Value) -> Result<Value, Error>{
    println!("{:?}",args);

    let input = serde_json::from_value::<Input>(args)?;
    println!("{:?}",input);

    
    let engine = Engine::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let linking = Module::from_binary(&engine, BINARY_WASM).unwrap();
   
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = Store::new(&engine, wasi);
    let max_concurrent_requests = Some(42);

    let http = HttpCtx::new(input.allowed_hosts, max_concurrent_requests).unwrap();
    http.add_to_linker(&mut linker).unwrap();
    

    linker.module(&mut store, "", &linking).unwrap();
    linker
        .get_default(&mut store, "").unwrap()
        .typed::<(), (), _>(&store).unwrap()
        .call(&mut store, ()).unwrap();

   
    

Ok(serde_json::json!("{ee}"))
}
