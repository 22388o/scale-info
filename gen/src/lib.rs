// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use scale_info::{
	form::{CompactForm},
	RegistryReadOnly,
	TypeDef,
};

// trait GenerateTypes

impl RegistryReadOnly {
	fn generate_typescript_definitions(&self) -> String {

	}
}

impl TypeDef {
	fn generate_type(&self, registry: &RegistryReadOnly) -> String {

	}
}