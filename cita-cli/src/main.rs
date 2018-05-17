extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate clap;

extern crate cita_tool;

use dotenv::dotenv;

use cita_tool::{Client, JsonRpcParams, ParamsValue};

fn main() {
    dotenv().ok();

    let jsonrpc_url = dotenv!("JSONRPC_URL");

    let matches = clap::App::new("CITA CLI")
        .arg(
            clap::Arg::with_name("url")
                .long("url")
                .default_value(jsonrpc_url)
                .takes_value(true)
                .help("JSONRPC server URL"),
        )
        .get_matches();
    let url = matches.value_of("url").unwrap();

    let mut client = Client::new().unwrap().add_url(url);
    let params = JsonRpcParams::new().insert(
        "method",
        ParamsValue::String(String::from("cita_blockNumber")),
    );

    let responses = client.send_request(params).unwrap();
    for response in responses {
        println!("{}", response);
    }
}
