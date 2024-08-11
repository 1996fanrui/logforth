// Copyright 2024 CratesLand Developers
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

use std::fmt::Arguments;

use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::TimeZone;
use colored::Color;
use colored::ColoredString;
use colored::Colorize;
use log::Level;

use crate::layout::KvDisplay;
use crate::layout::Layout;

/// A layout that formats log record as text.
///
/// Output format:
///
/// ```text
/// 2024-08-11 19:39:52,583 ERROR simple_stdio: examples/simple_stdio.rs:32 Hello error!
/// 2024-08-11 19:39:52,584  WARN simple_stdio: examples/simple_stdio.rs:33 Hello warn!
/// 2024-08-11 19:39:52,585  INFO simple_stdio: examples/simple_stdio.rs:34 Hello info!
/// 2024-08-11 19:39:52,586 DEBUG simple_stdio: examples/simple_stdio.rs:35 Hello debug!
/// 2024-08-11 19:39:52,587 TRACE simple_stdio: examples/simple_stdio.rs:36 Hello trace!
/// ```
///
/// By default, log levels are colored. You can turn on the `no-color` feature flag to disable this
/// feature.
///
/// You can also customize the color of each log level by setting the `colors` field with a
/// [`LevelColor`] instance.
#[derive(Default, Debug, Clone)]
pub struct TextLayout {
    pub colors: LevelColor,
    pub time_zone: Option<FixedOffset>,
}

/// Customize the color of each log level.
#[derive(Debug, Clone)]
pub struct LevelColor {
    pub error: Color,
    pub warn: Color,
    pub info: Color,
    pub debug: Color,
    pub trace: Color,
}

impl Default for LevelColor {
    fn default() -> Self {
        Self {
            error: Color::Red,
            warn: Color::Yellow,
            info: Color::Green,
            debug: Color::Blue,
            trace: Color::Magenta,
        }
    }
}

const DEFAULT_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S,%3f";

impl TextLayout {
    pub(crate) fn format<F>(&self, record: &log::Record, f: &F) -> anyhow::Result<()>
    where
        F: Fn(Arguments) -> anyhow::Result<()>,
    {
        let color = match record.level() {
            Level::Error => self.colors.error,
            Level::Warn => self.colors.warn,
            Level::Info => self.colors.info,
            Level::Debug => self.colors.debug,
            Level::Trace => self.colors.trace,
        };

        let now = Local::now();
        let time = self.format_data_time(now);

        let level = ColoredString::from(record.level().to_string()).color(color);
        let module = record.module_path().unwrap_or_default();
        let file = record.file().unwrap_or_default();
        let line = record.line().unwrap_or_default();
        let message = record.args();
        let kvs = KvDisplay::new(record.key_values());

        f(format_args!(
            "{time} {level:>5} {module}: {file}:{line} {message}{kvs}"
        ))
    }

    fn format_data_time(&self, now: DateTime<Local>) -> String {
        self.time_zone
            .map_or(now, |tz| now.with_timezone(&Local::from_offset(&tz)))
            .format(&DEFAULT_TIME_FORMAT)
            .to_string()
    }
}

impl From<TextLayout> for Layout {
    fn from(layout: TextLayout) -> Self {
        Layout::Text(layout)
    }
}

#[cfg(test)]
mod tests {
    use chrono::offset::TimeZone;
    use chrono::Datelike;
    use chrono::NaiveDate;
    use chrono::NaiveTime;

    use super::*;

    #[test]
    fn test_format_data_time_with_custom_time_zone() {
        let date_time = mock_date_time(2024, 8, 11, 20, 45, 35, 345);

        let custom_offset = FixedOffset::east_opt(8 * 3600); // UTC+8
                                                             // let custom_offset =None; // UTC+8
        let layout = TextLayout {
            colors: LevelColor::default(),
            time_zone: custom_offset,
        };

        let formatted_time = layout.format_data_time(date_time);

        let expected_time = "2024-08-11 15:36:35,957";

        // 断言格式化的时间是否符合预期
        assert_eq!(formatted_time, expected_time);
    }

    // #[test]
    // fn test_format_data_time_with_no_time_zone() {
    //     // 使用与 test_format_data_time_with_custom_time_zone 相同的方法创建模拟时间
    //
    //     // 创建一个没有时区偏移的 TextLayout
    //     let layout = TextLayout { time_zone: None };
    //
    //     // 调用 format_data_time 方法
    //     let formatted_time = layout.format_data_time(mock_local_datetime);
    //
    //     // 预期的格式化时间字符串，假设本地时间就是 UTC
    //     let expected_time = "2024-08-11 15:36:35,957";
    //
    //     // 断言格式化的时间是否符合预期
    //     assert_eq!(formatted_time, expected_time);
    // }

    fn mock_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        milli: u32,
    ) -> DateTime<Local> {
        let mock_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let mock_time = NaiveTime::from_hms_milli_opt(hour, min, sec, milli).unwrap();
        let mock_local_datetime = Local
            .from_local_datetime(&mock_date.and_time(mock_time))
            .single()
            .unwrap();
        mock_local_datetime
    }
}
