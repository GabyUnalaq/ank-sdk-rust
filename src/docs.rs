// Copyright (c) 2025 Elektrobit Automotive GmbH
//
// This program and the accompanying materials are made available under the
// terms of the Apache License, Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! This module is used to include additional markdown files in
//! the documentation.

mod contributing {
//! # Contributing
//! 
//! Welcome to the Ankaios community. Start here for info on how to contribute and help improve our project.
//! Please observe our [Community Code of Conduct](../conduct/index.html).
//! 
//! ## How to Contribute
//! 
//! This project welcomes contributions and suggestions.
//! You'll also need to create an [Eclipse Foundation account](https://accounts.eclipse.org/) and agree to the [Eclipse Contributor Agreement](https://www.eclipse.org/legal/ECA.php). See more info at <https://www.eclipse.org/projects/handbook/#contributing-contributors>.
//! 
//! If you have a bug to report or a feature to suggest, please use the New Issue button on the Issues page to access templates for these items.
//! 
//! Code contributions are to be submitted via pull requests.
//! For this fork this repository, apply the suggested changes and create a
//! pull request to integrate them.
//! Before creating the request, please ensure the following which we will check
//! besides a technical review:
//! 
//! - **No breaks**: All builds and tests pass (GitHub actions).
//! - **Docs updated**: Make sure any changes and additions are appropriately included into the design and user documentation.
//! - **Requirements**: Make sure that requirements are created for new features and those are [traced in the code and tests](https://eclipse-ankaios.github.io/ankaios/main/development/requirement-tracing/).
//! - **Rust coding guidelines**: Make sure to follow the [Rust coding guidelines for this project](https://eclipse-ankaios.github.io/ankaios/main/development/rust-coding-guidelines/).
//! - **Unit verification strategy**: Unit tests have been created according to the [unit verification strategy](https://eclipse-ankaios.github.io/ankaios/main/development/unit-verification/).
//! 
//! ## Communication
//! 
//! Please join our [developer mailing list](https://accounts.eclipse.org/mailing-list/ankaios-dev) for up to date information or use the Ankaios [discussion forum](https://github.com/eclipse-ankaios/ankaios/discussions).
}

mod conduct {
    #![doc = include_str!("../CODE_OF_CONDUCT.md")]
}
