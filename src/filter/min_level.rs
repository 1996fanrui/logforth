// Copyright 2024 tison <wander4096@gmail.com>
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

use log::LevelFilter;
use log::Metadata;

use crate::filter::Filter;
use crate::filter::FilterResult;

/// A filter that checks if the log level is at most the specified level.
///
/// From least to most verbose, the levels are:
///
/// - `Error`
/// - `Warn`
/// - `Info`
/// - `Debug`
/// - `Trace`
///
/// If MaxLevel is set to `Info`, it will allow `Error`, `Warn`, and `Info` logs.
///
/// If MaxLevel is set to `Off`, it will reject all logs.
#[derive(Debug, Clone)]
pub struct MaxLevel(pub LevelFilter);

impl MaxLevel {
    pub(crate) fn filter(&self, metadata: &Metadata) -> FilterResult {
        let level = metadata.level();
        if level <= self.0 {
            FilterResult::Neutral
        } else {
            FilterResult::Reject
        }
    }
}
impl From<MaxLevel> for Filter {
    fn from(filter: MaxLevel) -> Self {
        Filter::MaxLevel(filter)
    }
}

impl From<LevelFilter> for Filter {
    fn from(filter: LevelFilter) -> Self {
        Filter::MaxLevel(MaxLevel(filter))
    }
}
