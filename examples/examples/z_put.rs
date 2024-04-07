//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use clap::Parser;
use zenoh::config::Config;
use zenoh::core::AsyncResolve;
use zenoh::key_expr::KeyExpr;
use zenoh_examples::CommonArgs;

#[tokio::main]
async fn main() {
    // initiate logging
    env_logger::init();

    let (config, key_expr, value) = parse_args();

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Putting Data ('{key_expr}': '{value}')...");
    session.put(&key_expr, value).res().await.unwrap();
}

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    #[arg(short, long, default_value = "demo/example/zenoh-rs-put")]
    /// The key expression to write to.
    key: KeyExpr<'static>,
    #[arg(short, long, default_value = "Put from Rust!")]
    /// The value to write.
    value: String,
    #[command(flatten)]
    common: CommonArgs,
}

fn parse_args() -> (Config, KeyExpr<'static>, String) {
    let args = Args::parse();
    (args.common.into(), args.key, args.value)
}
