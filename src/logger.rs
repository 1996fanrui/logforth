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

use log::Log;
use log::Metadata;
use log::Record;

use crate::appender::AppenderImpl;

pub struct Logger {
    pub appenders: Vec<AppenderImpl>,
}

impl Logger {
    /// Dispatch this log record to all appenders.
    fn do_log(&self, record: &Record) {
        for appender in &self.appenders {
            appender.log(record);
        }
    }

    /// Whether the filters prevent this log record from logging.
    fn check_filtered(&self, _: &Metadata) -> bool {
        false
    }

    /// Whether a log with the given metadata would eventually end up logging something.
    fn check_enabled(&self, m: &Metadata) -> bool {
        !self.check_filtered(m) && self.appenders.iter().any(|a| a.enabled(m))
    }
}

impl Log for Logger {
    fn enabled(&self, m: &Metadata) -> bool {
        self.check_enabled(m)
    }

    fn log(&self, record: &Record) {
        if self.check_filtered(record.metadata()) {
            return;
        }

        self.do_log(record);
    }

    fn flush(&self) {
        for appender in &self.appenders {
            appender.flush();
        }
    }
}
