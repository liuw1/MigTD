// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

mod build;
mod config;
mod library;
mod servtd_info_hash;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Program {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Image(build::BuildArgs),
    Hash(servtd_info_hash::ServtdInfoHashArgs),
    LibTest(library::LibraryCrates),
    LibBuild(library::LibraryCrates),
}

fn main() {
    match Program::parse().command {
        Commands::Image(args) => {
            let bin = args.build().expect("Fail to build migtd binary");
            println!("Successfully generate MigTD binary: {}", bin.display());
        }
        Commands::Hash(args) => {
            args.generate().expect("Fail to calculate tdinfo hash");
        }
        Commands::LibTest(args) => args.test().expect("Library crates test failed"),
        Commands::LibBuild(args) => args.build().expect("Library crates build failed"),
    };
}
