use anyhow::Result;
use futures::FutureExt;
use log::info;
use MEV_Bot_Solana::arbitrage::strategies::run_arbitrage_strategy;
use MEV_Bot_Solana::transactions::create_transaction::{create_swap_transaction, ChainType, TransactionType};
use std::collections::HashMap;
use tokio::task::JoinSet;
use solana_client::rpc_client::RpcClient;
use MEV_Bot_Solana::common::constants::Env;
use MEV_Bot_Solana::markets::pools::load_all_pools;
use MEV_Bot_Solana::common::utils::{from_str, get_tokens_infos, setup_logger};
use MEV_Bot_Solana::arbitrage::types::{SwapPathResult, SwapRouteSimulation, TokenInArb, TokenInfos};

use rust_socketio::{Payload, asynchronous::{Client, ClientBuilder},};


// use MEV_Bot_Solana::common::pools::{load_all_pools, Pool};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    // log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    setup_logger().unwrap();

    info!("Starting MEV_Bot_Solana");
    info!("⚠️⚠️ New fresh pools fetched on METEORA and RAYDIUM are excluded because a lot of time there have very low liquidity, potentially can be used on subscribe log strategy");
    info!("⚠️⚠️ Liquidity is fetch to API and can be outdated on Radyium Pool");

    let env = Env::new();

    let rpc_client: RpcClient = RpcClient::new(env.rpc_url);

    let mut set: JoinSet<()> = JoinSet::new();
    
    // info!("🏊 Launch pools fetching infos...");
    // //Params is for re-fetching pools on API or not
    // let dexs = load_all_pools(false).await;
    // info!("🏊 {} Dexs are loaded", dexs.len());
    
    // // The first token is the base token (here SOL)
    // let tokens_to_arb: Vec<TokenInArb> = vec![
    //     TokenInArb{address: String::from("So11111111111111111111111111111111111111112"), symbol: String::from("SOL")}, // Base token here
    //     TokenInArb{address: String::from("25hAyBQfoDhfWx9ay6rarbgvWGwDdNqcHsXS3jQ3mTDJ"), symbol: String::from("MANEKI")},
    //     TokenInArb{address: String::from("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"), symbol: String::from("JUP")},
    //     TokenInArb{address: String::from("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"), symbol: String::from("WIF")},
    // ];
    // The first token is the base token (here SOL)
    // let tokens_to_arb: Vec<TokenInArb> = vec![
    //     TokenInArb{address: String::from("So11111111111111111111111111111111111111112"), symbol: String::from("SOL")}, // Base token here
    //     TokenInArb{address: String::from("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), symbol: String::from("USDC")},
    //     TokenInArb{address: String::from("FePbYijSZfdHvswUhfBqztJ7kzUs5AEBMDi71xQhTtWC"), symbol: String::from("kiki")},
    //     TokenInArb{address: String::from("8vCAUbxejdtaxn6jnX5uaQTyTZLmXALg9u1bvFCAjtx7"), symbol: String::from("ZACK")},
    // ];
    let tokens_to_arb: Vec<TokenInArb> = vec![
        TokenInArb{address: String::from("So11111111111111111111111111111111111111112"), symbol: String::from("SOL")}, // Base token here
        // TokenInArb{address: String::from("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"), symbol: String::from("USDC")},
        TokenInArb{address: String::from("9LAjk5F4rFetELE4CygcBbZ5hYc2QhRrbJjfm5Q26jWM"), symbol: String::from("DAVIDO")},
        // TokenInArb{address: String::from("GoxLaNFQiqnV97p7aRGP4ghvLZ4GwJN9NUNPpozvJZCV"), symbol: String::from("OSAK")},
    ];

    // let tokens_infos: HashMap<String, TokenInfos> = get_tokens_infos(tokens_to_arb.clone()).await;

    info!("🪙🪙 Tokens Infos: {:?}", tokens_to_arb);
    info!("📈 Launch arbitrage process...");
    // let (markets_arb, all_paths) = calculate_arb(dexs, tokens_to_arb).await;
    
    info!("Open Socket IO channel...");
    let env = Env::new();
    
    let callback = |payload: Payload, socket: Client| {
        async move {
            match payload {
                Payload::String(data) => println!("Received: {}", data),
                Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
                Payload::Text(data) => println!("Received Text: {:?}", data),
            }
        }
        .boxed()
    };
    
    let mut socket = ClientBuilder::new("http://localhost:3000")
        .namespace("/")
        .on("connection", callback)
        .on("error", |err, _| {
            async move { eprintln!("Error: {:#?}", err) }.boxed()
        })
        .on("orca_quote", callback)
        .on("orca_quote_res", callback)
        .connect()
        .await
        .expect("Connection failed");


    
    // set.spawn(run_arbitrage_strategy(socket, dexs, tokens_to_arb, tokens_infos));

    // let spr = SwapPathResult{ 
    //     path_id: 1,
    //     hops: 1,
    //     tokens_path: "".to_string(),
    //     route_simulations: vec![
    //         SwapRouteSimulation{
    //             id_route: 53,
    //             pool_address: "HvYSJ3CxLviabBAbrkXAKh7xk7DJZtn4gvLeEuFdGeXi".to_string(),
    //             dex_label: MEV_Bot_Solana::markets::types::DexLabel::METEORA,
    //             token_in: "So11111111111111111111111111111111111111112".to_string(),
    //             token_out: "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm".to_string(),
    //             token_0to1: true,
    //             amount_in: 1000000,
    //             estimated_amount_out: "100".to_string(),
    //             estimated_min_amount_out: "100".to_string()
    //           },
    //         SwapRouteSimulation{ 
    //             id_route: 0,
    //             pool_address: "48XCxXxuVjEefX8K1qAMD2yELYUkXihYQegtSKXf4JXG".to_string(),
    //             dex_label: MEV_Bot_Solana::markets::types::DexLabel::RAYDIUM,
    //             token_0to1: true,
    //             token_in: "So11111111111111111111111111111111111111112".to_string(),
    //             token_out: "9LAjk5F4rFetELE4CygcBbZ5hYc2QhRrbJjfm5Q26jWM".to_string(),
    //             amount_in: 1000000, // 0.001 SOL
    //             estimated_amount_out:"710927".to_string(),
    //             estimated_min_amount_out: "703888".to_string()
    //         },
    //         SwapRouteSimulation{ 
    //             id_route: 0,
    //             pool_address: "4E6q7eJE6vBNdquqzYYi5gvzd5MNpwiQKhjbRTRQGuQd".to_string(),
    //             dex_label: MEV_Bot_Solana::markets::types::DexLabel::ORCA_WHIRLPOOLS,
    //             token_0to1: true,
    //             token_in: "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm".to_string(),
    //             token_out: "So11111111111111111111111111111111111111112".to_string(),
    //             amount_in: 1000000, // 0.001 SOL
    //             estimated_amount_out:"100".to_string(),
    //             estimated_min_amount_out: "100".to_string()
    //         }
    //     ],
    //     token_in: "So11111111111111111111111111111111111111112".to_string(),
    //     token_in_symbol: "SOL".to_string(),
    //     token_out: "So11111111111111111111111111111111111111112".to_string(),
    //     token_out_symbol: "SOL".to_string(),
    //     amount_in: 1000000,
    //     estimated_amount_out: "10000000".to_string(),
    //     estimated_min_amount_out: "10000000".to_string(),
    //     result: -46478200.0
    // };

    // let _ = create_transaction(ChainType::Mainnet, TransactionType::CreateSwap, spr, from_str("6nGymM5X1djYERKZtoZ3Yz3thChMVF6jVRDzhhcmxuee").unwrap()).await;
    
    while let Some(res) = set.join_next().await {
        info!("{:?}", res);
    }

    println!("End");
    Ok(())
}