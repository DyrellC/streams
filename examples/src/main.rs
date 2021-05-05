#![allow(non_snake_case)]
#![allow(dead_code)]
//#![no_std]

use dotenv;
use futures::executor::block_on;
use std::env;

use rand::Rng;

use iota_streams::{
    app::transport::{
        tangle::client::{
            iota_client,
            Client,
            SendOptions,
        },
        TransportOptions,
    },
    app_channels::api::tangle::Transport,
    core::{
        prelude::{
            Rc,
            String,
        },
        Result,
    },
};

use core::cell::RefCell;

mod branching;

fn run_single_branch_test<T: Transport>(transport: Rc<RefCell<T>>, seed: &str) {
    println!("\tRunning Single Branch Test, seed: {}", seed);
    match branching::single_branch::example(transport, false, seed) {
        Err(err) => println!("Error in Single Branch test: {:?}", err),
        Ok(_) => println!("\tSingle Branch Test completed!!"),
    }
    println!("#######################################");
}

fn run_multi_branch_test<T: Transport>(transport: Rc<RefCell<T>>, seed: &str) {
    println!("\tRunning Multi Branch Test, seed: {}", seed);
    match branching::multi_branch::example(transport, true, seed) {
        Err(err) => println!("Error in Multi Branch test: {:?}", err),
        Ok(_) => println!("\tMulti Branch Test completed!!"),
    }
    println!("#######################################");
}

fn run_main<T: Transport>(transport: T) -> Result<()> {
    let seed1: &str = "SEEDSINGLE";
    let seed2: &str = "SEEDMULTI9";

    let transport = Rc::new(RefCell::new(transport));
    run_single_branch_test(transport.clone(), seed1);
    run_multi_branch_test(transport.clone(), seed2);

    Ok(())
}

#[allow(dead_code)]
fn main_pure() {
    let transport = iota_streams::app_channels::api::tangle::BucketTransport::new();

    println!("#######################################");
    println!("Running pure tests without accessing Tangle");
    println!("#######################################");
    println!("\n");

    let transport = Rc::new(RefCell::new(transport));
    run_single_branch_test(transport.clone(), "PURESEEDA");
    run_multi_branch_test(transport.clone(), "PURESEEDB");
    println!("Done running pure tests without accessing Tangle");
    println!("#######################################");
}

#[allow(dead_code)]
fn main_client() {
    // Load or .env file, log message if we failed
    if dotenv::dotenv().is_err() {
        println!(".env file not found; copy and rename example.env to \".env\"");
    };

    // Parse env vars with a fallback
    let node_url = env::var("URL").unwrap_or("http://68.183.204.5:14265".to_string());

    let mut send_opt = SendOptions::default();

    // Fails at unwrap when the url isnt working
    // TODO: Fail gracefully
    let iota_client = block_on(
        iota_client::ClientBuilder::new()
            .with_node(&node_url)
            .unwrap()
            //.with_node_sync_disabled()
            .with_local_pow(false)
            .finish(),
    )
    .unwrap();

    let client = Client::new(send_opt, iota_client);

    let mut transport = Rc::new(RefCell::new(client));
    transport.set_send_options(send_opt);

    let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let seed1: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let seed2: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();

    println!("#######################################");
    println!("Running tests accessing Tangle via node {}", &node_url);
    println!("#######################################");
    println!("\n");

    run_single_branch_test(transport.clone(), seed1);
    run_multi_branch_test(transport.clone(), seed2);
    println!("Done running tests accessing Tangle via node {}", &node_url);
    println!("#######################################");
}

#[tokio::main]
async fn main() {
    // main_pure();
    main_client();
}
