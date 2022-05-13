extern crate tokio;
extern crate reqwest;

use std::fs::File;
use std::io;
use std::process;

//use anyhow::Result;

#[cfg(target_os = "windows")]
async fn file_download(url:String){
    //https://zenn.dev/hpp/articles/76483c0d6237be
    let response = reqwest::get(url).await;
    let bytes = match response {
        Ok(res) => res.bytes().await,
        Err(err) => {
            println!("{}",err);
            process::exit(1);
        }
    };

}

#[cfg(target_os = "windows")]
fn ruby_download(){
    let client = reqwest::Client::new();
}

#[tokio::main]
async fn main() {

    
}

