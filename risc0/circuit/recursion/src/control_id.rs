// Copyright 2024 RISC Zero, Inc.
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

/// Merkle root of the RECURSION_CONTROL_IDS
pub const ALLOWED_IDS_ROOT: &str =
    "12dcee702de31c25ddac59432cf2630c0947fb006170b810ca71c73eb63a6b0d";

pub const RECURSION_CONTROL_IDS: [(&str, &str); 15] = [
    (
        "identity.zkr",
        "f5385d749b7313743520f7267a1d0472ed75c703ab9e23657419df15f7516a0a",
    ),
    (
        "join.zkr",
        "1224f74f3e87920d57ff8b53ed2174278d96691d0a361171a01fd36c8305bd01",
    ),
    (
        "lift_14.zkr",
        "7a00ce560d276c374349f265b26c2c70ecbab61f8e21753c179cc560a3c5ff3a",
    ),
    (
        "lift_15.zkr",
        "d5724822ce4e8e35cc647238c965741ba759e662fa58b4162eb31e183d01a074",
    ),
    (
        "lift_16.zkr",
        "a47bf3031c9fe80c3b73c3197ce9d75b1bc9be6454df845df4dec940e51c0903",
    ),
    (
        "lift_17.zkr",
        "48fc131bae53d026e9a2e237fee7145ac19e9906c2243c724ce1c9576e238377",
    ),
    (
        "lift_18.zkr",
        "ca5c2d48a5095f05a5b20318b9b2c41cc55465086d8e4b381b1a9171d0dc9e26",
    ),
    (
        "lift_19.zkr",
        "0595b92df69b83300fc07f2f7f389c6696bb8e4a0ed8932edb18083a69ccd81b",
    ),
    (
        "lift_20.zkr",
        "b10e770ac0dabc4e8a4ee105419255280c7bac44af5a4f5fd7899e2d13df8a4a",
    ),
    (
        "lift_21.zkr",
        "0be6f52e41ed284a7e644d296ec74f5c059bc146f0bef067592f6a3133da046f",
    ),
    (
        "lift_22.zkr",
        "bb41e502a2be79145c43d43eaa69136f287b144a33210d307595796e82be3e22",
    ),
    (
        "lift_23.zkr",
        "174e3c46c0bf154cf1c737427ab2da30694a24302a2ac147aacaac5b73434839",
    ),
    (
        "lift_24.zkr",
        "964cb40d31995f0ac716af230570e65968709b741433d741dbc6606f01e66c14",
    ),
    (
        "resolve.zkr",
        "2a67fc52e72cbe6be9c52e58d55c52292a1d0c3ab3e3cf760259813be3772f11",
    ),
    (
        "test_recursion_circuit.zkr",
        "ca7ade1f42976e5e103ad45c97e42963515f5b4b33076418e0a9390a576edd4e",
    ),
];

pub const BN254_CONTROL_ID: &str =
    "14b5616aff6c7fc70765bfc92fad96826d76375e5731a65e762e89bb8e328400";
