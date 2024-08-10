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

use std::fmt;
use time::OffsetDateTime;

/// A clock providing access to the current time.
pub trait Clock: fmt::Debug + Send {
    fn now(&self) -> OffsetDateTime;
}

#[derive(Debug)]
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

/// The time could be reset.
#[derive(Debug)]
pub struct ManualClock {
    fixed_time: OffsetDateTime,
}

impl Clock for ManualClock {
    fn now(&self) -> OffsetDateTime {
        self.fixed_time
    }
}

impl ManualClock {
    pub fn new(fixed_time: OffsetDateTime) -> ManualClock {
        ManualClock { fixed_time }
    }

    pub fn set_now(&mut self, new_time: OffsetDateTime) {
        self.fixed_time = new_time;
    }
}

#[derive(Debug)]
pub enum StateClock {
    DefaultClock(DefaultClock),
    ManualClock(ManualClock),
}

impl StateClock {
    pub fn now(&self) -> OffsetDateTime {
        match self {
            StateClock::DefaultClock(clock) => clock.now(),
            StateClock::ManualClock(clock) => clock.now(),
        }
    }

    pub fn set_now(&mut self, new_time: OffsetDateTime) {
        if let StateClock::ManualClock(clock) = self {
            clock.set_now(new_time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_manual_clock_adjusting() {
        let mut clock = ManualClock {
            fixed_time: datetime!(2023-01-01 12:00:00 UTC),
        };
        assert_eq!(clock.now(), datetime!(2023-01-01 12:00:00 UTC));

        clock.set_now(datetime!(2024-01-01 12:00:00 UTC));
        assert_eq!(clock.now(), datetime!(2024-01-01 12:00:00 UTC));
    }
}
