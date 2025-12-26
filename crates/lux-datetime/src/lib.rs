#![allow(clippy::cargo_common_metadata)]

//! DateTime - Roblox-compatible DateTime type
//!
//! Provides the exact Roblox DateTime API:
//! - DateTime.now()
//! - DateTime.fromUnixTimestamp(seconds)
//! - DateTime.fromUnixTimestampMillis(millis)
//! - DateTime.fromUniversalTime(year, month, day, hour, min, sec, ms)
//! - DateTime.fromLocalTime(year, month, day, hour, min, sec, ms)
//! - DateTime.fromIsoDate(string)
//!
//! Instance properties/methods:
//! - .UnixTimestamp, .UnixTimestampMillis
//! - :ToUniversalTime(), :ToLocalTime(), :ToIsoDate()
//! - :FormatUniversalTime(format, locale)
//! - :FormatLocalTime(format, locale)

use chrono::{
    DateTime as ChronoDateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use lux_utils::TableBuilder;
use mlua::prelude::*;
use std::cmp::Ordering;

const TYPEDEFS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/types.d.luau"));

/// Returns type definitions for the DateTime library.
#[must_use]
pub fn typedefs() -> String {
    TYPEDEFS.to_string()
}

/// DateTime struct - wraps chrono UTC datetime
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    inner: ChronoDateTime<Utc>,
}

impl DateTime {
    /// Current time
    #[inline]
    pub fn now() -> Self {
        Self { inner: Utc::now() }
    }

    /// From unix timestamp (seconds)
    pub fn from_unix_timestamp(secs: f64) -> Option<Self> {
        let whole = secs.trunc() as i64;
        let fract = secs.fract();
        let nanos = (fract * 1_000_000_000f64)
            .round()
            .clamp(0.0, u32::MAX as f64) as u32;
        ChronoDateTime::<Utc>::from_timestamp(whole, nanos).map(|inner| Self { inner })
    }

    /// From unix timestamp millis
    pub fn from_unix_timestamp_millis(millis: i64) -> Option<Self> {
        ChronoDateTime::<Utc>::from_timestamp_millis(millis).map(|inner| Self { inner })
    }

    /// From universal time components
    pub fn from_universal_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millis: u32,
    ) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        let time = NaiveTime::from_hms_milli_opt(hour, minute, second, millis)?;
        let inner = Utc.from_utc_datetime(&NaiveDateTime::new(date, time));
        Some(Self { inner })
    }

    /// From local time components
    pub fn from_local_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millis: u32,
    ) -> Option<Self> {
        let date = NaiveDate::from_ymd_opt(year, month, day)?;
        let time = NaiveTime::from_hms_milli_opt(hour, minute, second, millis)?;
        let inner = Local
            .from_local_datetime(&NaiveDateTime::new(date, time))
            .single()?
            .with_timezone(&Utc);
        Some(Self { inner })
    }

    /// From ISO 8601 / RFC 3339 string
    pub fn from_iso_date(s: &str) -> Option<Self> {
        ChronoDateTime::parse_from_rfc3339(s).ok().map(|dt| Self {
            inner: dt.with_timezone(&Utc),
        })
    }

    /// Unix timestamp in seconds
    #[inline]
    pub fn unix_timestamp(&self) -> i64 {
        self.inner.timestamp()
    }

    /// Unix timestamp in milliseconds
    #[inline]
    pub fn unix_timestamp_millis(&self) -> i64 {
        self.inner.timestamp_millis()
    }

    /// To ISO 8601 string
    #[inline]
    pub fn to_iso_date(&self) -> String {
        self.inner.to_rfc3339()
    }

    /// To universal time table
    pub fn to_universal_time(&self) -> DateTimeValues {
        DateTimeValues::from_chrono(&self.inner)
    }

    /// To local time table
    pub fn to_local_time(&self) -> DateTimeValues {
        DateTimeValues::from_chrono(&self.inner.with_timezone(&Local))
    }

    /// Format with universal time
    pub fn format_universal_time(&self, fmt: Option<&str>, _locale: Option<&str>) -> String {
        self.inner
            .format(fmt.unwrap_or("%Y-%m-%d %H:%M:%S"))
            .to_string()
    }

    /// Format with local time
    pub fn format_local_time(&self, fmt: Option<&str>, _locale: Option<&str>) -> String {
        self.inner
            .with_timezone(&Local)
            .format(fmt.unwrap_or("%Y-%m-%d %H:%M:%S"))
            .to_string()
    }
}

/// DateTimeValues table
#[derive(Debug, Clone, Copy)]
pub struct DateTimeValues {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
}

impl DateTimeValues {
    fn from_chrono<T: TimeZone>(dt: &ChronoDateTime<T>) -> Self {
        use chrono::Datelike;
        use chrono::Timelike;
        Self {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
            millisecond: dt.timestamp_subsec_millis(),
        }
    }
}

impl IntoLua for DateTimeValues {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        let t = lua.create_table()?;
        t.set("Year", self.year)?;
        t.set("Month", self.month)?;
        t.set("Day", self.day)?;
        t.set("Hour", self.hour)?;
        t.set("Minute", self.minute)?;
        t.set("Second", self.second)?;
        t.set("Millisecond", self.millisecond)?;
        Ok(LuaValue::Table(t))
    }
}

impl LuaUserData for DateTime {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("UnixTimestamp", |_, this| Ok(this.unix_timestamp()));
        fields.add_field_method_get("UnixTimestampMillis", |_, this| {
            Ok(this.unix_timestamp_millis())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_, this, other: LuaUserDataRef<Self>| {
            Ok(this.inner == other.inner)
        });
        methods.add_meta_method(LuaMetaMethod::Lt, |_, this, other: LuaUserDataRef<Self>| {
            Ok(matches!(this.cmp(&other), Ordering::Less))
        });
        methods.add_meta_method(LuaMetaMethod::Le, |_, this, other: LuaUserDataRef<Self>| {
            Ok(matches!(this.cmp(&other), Ordering::Less | Ordering::Equal))
        });
        methods.add_meta_method(
            LuaMetaMethod::ToString,
            |_, this, ()| Ok(this.to_iso_date()),
        );

        methods.add_method(
            "ToUniversalTime",
            |_, this, ()| Ok(this.to_universal_time()),
        );
        methods.add_method("ToLocalTime", |_, this, ()| Ok(this.to_local_time()));
        methods.add_method("ToIsoDate", |_, this, ()| Ok(this.to_iso_date()));
        methods.add_method(
            "FormatUniversalTime",
            |_, this, (fmt, locale): (Option<String>, Option<String>)| {
                Ok(this.format_universal_time(fmt.as_deref(), locale.as_deref()))
            },
        );
        methods.add_method(
            "FormatLocalTime",
            |_, this, (fmt, locale): (Option<String>, Option<String>)| {
                Ok(this.format_local_time(fmt.as_deref(), locale.as_deref()))
            },
        );
    }
}

/// Create the DateTime global table
pub fn create(lua: Lua) -> LuaResult<LuaValue> {
    TableBuilder::new(lua)?
        .with_function("now", |lua, ()| lua.create_userdata(DateTime::now()))?
        .with_function("fromUnixTimestamp", |lua, secs: f64| {
            DateTime::from_unix_timestamp(secs)
                .map(|dt| lua.create_userdata(dt))
                .transpose()?
                .ok_or_else(|| LuaError::external("invalid timestamp"))
        })?
        .with_function("fromUnixTimestampMillis", |lua, millis: f64| {
            DateTime::from_unix_timestamp_millis(millis.round() as i64)
                .map(|dt| lua.create_userdata(dt))
                .transpose()?
                .ok_or_else(|| LuaError::external("invalid timestamp"))
        })?
        .with_function("fromUniversalTime", |lua, args: LuaMultiValue| {
            let a: Vec<LuaValue> = args.into_vec();
            let year = a.first().and_then(|v| v.as_integer()).unwrap_or(1970) as i32;
            let month = a.get(1).and_then(|v| v.as_integer()).unwrap_or(1) as u32;
            let day = a.get(2).and_then(|v| v.as_integer()).unwrap_or(1) as u32;
            let hour = a.get(3).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let min = a.get(4).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let sec = a.get(5).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let ms = a.get(6).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            DateTime::from_universal_time(year, month, day, hour, min, sec, ms)
                .map(|dt| lua.create_userdata(dt))
                .transpose()?
                .ok_or_else(|| LuaError::external("invalid date/time"))
        })?
        .with_function("fromLocalTime", |lua, args: LuaMultiValue| {
            let a: Vec<LuaValue> = args.into_vec();
            let year = a.first().and_then(|v| v.as_integer()).unwrap_or(1970) as i32;
            let month = a.get(1).and_then(|v| v.as_integer()).unwrap_or(1) as u32;
            let day = a.get(2).and_then(|v| v.as_integer()).unwrap_or(1) as u32;
            let hour = a.get(3).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let min = a.get(4).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let sec = a.get(5).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            let ms = a.get(6).and_then(|v| v.as_integer()).unwrap_or(0) as u32;
            DateTime::from_local_time(year, month, day, hour, min, sec, ms)
                .map(|dt| lua.create_userdata(dt))
                .transpose()?
                .ok_or_else(|| LuaError::external("invalid date/time"))
        })?
        .with_function("fromIsoDate", |lua, s: String| {
            DateTime::from_iso_date(&s)
                .map(|dt| lua.create_userdata(dt))
                .transpose()?
                .ok_or_else(|| LuaError::external("invalid ISO date"))
        })?
        .build_readonly()
        .map(LuaValue::Table)
}
