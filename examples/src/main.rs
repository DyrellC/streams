#![allow(non_snake_case)]
#![allow(dead_code)]
//#![no_std]

use std::env;

use rand::Rng;

use iota_streams::{
    app::{
        identity::iota::Client as DIDClient,
        id::make_account,
        transport::tangle::client::Client
    },
    app_channels::api::tangle::{
        ChannelType,
        Transport,
    },
    core::{
        prelude::{
            Rc,
            String,
        },
        Result,
    },
};

use core::cell::RefCell;
use iota_streams::app::identity::account::Account;

mod branching;

fn run_recovery_test<T: Transport>(transport: Rc<RefCell<T>>, seed: &str) {
    println!("\tRunning Recovery Test, seed: {}", seed);
    match branching::recovery::example(transport, ChannelType::SingleBranch, seed) {
        Err(err) => println!("Error in recovery test: {:?}", err),
        Ok(_) => println!("\tRecovery test completed!!"),
    }
    println!("#######################################");
}

async fn run_single_branch_test<T: Transport>(transport: Rc<RefCell<T>>, seed: &str, account: &Account) {
    println!("\tRunning Single Branch Test, seed: {}", seed);
    match branching::single_branch::example(account, transport, ChannelType::SingleBranch, seed).await {
        Err(err) => println!("Error in Single Branch test: {:?}", err),
        Ok(_) => println!("\tSingle Branch Test completed!!"),
    }
    println!("#######################################");
}

fn run_multi_branch_test<T: Transport>(transport: Rc<RefCell<T>>, seed: &str) {
    println!("\tRunning Multi Branch Test, seed: {}", seed);
    match branching::multi_branch::example(transport, ChannelType::MultiBranch, seed) {
        Err(err) => println!("Error in Multi Branch test: {:?}", err),
        Ok(_) => println!("\tMulti Branch Test completed!!"),
    }
    println!("#######################################");
}

fn run_main<T: Transport>(transport: T) -> Result<()> {
    let seed1: &str = "SEEDSINGLE";
    let seed2: &str = "SEEDMULTI9";
    let seed3: &str = "SEEDRECOVERY";

    let transport = Rc::new(RefCell::new(transport));
    //run_single_branch_test(transport.clone(), seed1);
    run_multi_branch_test(transport.clone(), seed2);
    run_recovery_test(transport, seed3);

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
    //run_single_branch_test(transport.clone(), "PURESEEDA");
    run_multi_branch_test(transport.clone(), "PURESEEDB");
    run_recovery_test(transport, "PURESEEDC");
    println!("Done running pure tests without accessing Tangle");
    println!("#######################################");
}

#[allow(dead_code)]
async fn main_client() {
    // Load or .env file, log message if we failed
    if dotenv::dotenv().is_err() {
        println!(".env file not found; copy and rename example.env to \".env\"");
    };

    // Parse env vars with a fallback
    let node_url = env::var("URL").unwrap_or_else(|_| "https://chrysalis-nodes.iota.org".to_string());

    println!("#######################################");
    println!("Making a DID Account for use in testing at node {}", &node_url);
    println!("#######################################");

    let did_client = DIDClient::builder().node("http://68.183.204.5:14265").unwrap().build().await.unwrap();
    let account = make_account(did_client).await;

    let client = Client::new_from_url(&node_url);

    let transport = Rc::new(RefCell::new(client));

    let alph9 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
    let seed1: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let seed2: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();
    let seed3: &str = &(0..10)
        .map(|_| alph9.chars().nth(rand::thread_rng().gen_range(0, 27)).unwrap())
        .collect::<String>();

    println!("#######################################");
    println!("Running tests accessing Tangle via node {}", &node_url);
    println!("#######################################");
    println!("\n");

    run_single_branch_test(transport.clone(), seed1, &account).await;
    //run_multi_branch_test(transport.clone(), seed2);
    //run_recovery_test(transport, seed3);
    println!("Done running tests accessing Tangle via node {}", &node_url);
    println!("#######################################");
}

#[tokio::main]
async fn main() {
    // main_pure();
    main_client().await;
}
