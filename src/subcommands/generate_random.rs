// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Generates a sequence of random bytes.

use crate::error::Result;
use parsec_client::BasicClient;
use structopt::StructOpt;

/// Generates a sequence of random bytes.
#[derive(Debug, StructOpt)]
pub struct GenerateRandom {
    #[structopt(short = "n", long = "nbytes")]
    nbytes: usize,
}

impl GenerateRandom {
    /// Generates a sequence of random bytes.
    pub fn run(&self, basic_client: BasicClient) -> Result<()> {
        info!("Generating {} random bytes...", self.nbytes);

        let result = basic_client.psa_generate_random(self.nbytes)?;

        success!("Random bytes:");
        for byte in result {
            print!("{:02X} ", byte);
        }
        println!();
        Ok(())
    }
}
