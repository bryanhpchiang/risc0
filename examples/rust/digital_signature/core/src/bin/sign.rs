// Copyright 2022 Risc0, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
#![no_std]

use risc0_zkvm_guest::{env, sha};

use digital_signature_core::{SignMessageCommit, SigningRequest};
extern crate nalgebra as na;
use na::{U2, U3, Dynamic, ArrayStorage, SMatrix};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let request: SigningRequest = env::read();
    let m = SMatrix::<u32, 3, 4>::new(11, 12, 13, 14,
        21, 22, 23, 24,
        31, 32, 33, 34);
    
    env::commit(&SignMessageCommit {
        identity: *sha::digest(&request.passphrase.pass),
        msg: request.msg,
    });
}
