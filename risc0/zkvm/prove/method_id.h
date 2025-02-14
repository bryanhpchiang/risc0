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

#pragma once

#include "risc0/zkvm/verify/method_id.h"

#include <memory>

namespace risc0 {

MethodID makeMethodID(const std::string& elfFile);
void writeMethodID(const std::string& filename, const MethodID& id);

namespace rust {

class MethodID {
public:
  MethodID(const std::string& elf_path);
  void write(const std::string& filename) const;

private:
  ::risc0::MethodID id;
};

std::unique_ptr<MethodID> new_method_id(const std::string& elf_path);

} // namespace rust

} // namespace risc0
