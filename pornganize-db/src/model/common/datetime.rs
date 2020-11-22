use std::ops::Deref;
use chrono::{
    NaiveTime as ChronoTime,
    DateTime as ChronoDateTime,
    Utc,
    TimeZone as _,
    Datelike as _,
    Timelike as _,
};
use protobuf::{
    RepeatedField,
    SingularPtrField,
    Message,
    ProtobufEnum,
};
use serde::{Deserialize, Serialize};

use crate::model::messages::common::{
    DateTime as DateTimeMessage,
    Date as DateMessage,
    Time as TimeMessage,
};


#[derive(Debug, Serialize, Deserialize)]
pub struct DateTime(ChronoDateTime<Utc>);

impl DateTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn today() -> Self {
        Self(Utc::today().and_hms(0, 0, 0))
    }

    pub fn or_none(field: SingularPtrField<DateTimeMessage>) -> Option<Self> {
        match field.into_option() {
            Some(dt) => Some(Self::from(dt)),
            None => None,
        }
    }

    pub fn or_now(field: SingularPtrField<DateTimeMessage>) -> Self {
        match field.into_option() {
            Some(dt) => Self::from(dt),
            None => Self::now(),
        }
    }
}

impl Deref for DateTime {
    type Target = ChronoDateTime<Utc>;

    fn deref(&self) -> &ChronoDateTime<Utc> {
        &self.0
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Time(ChronoTime);

impl Deref for Time {
    type Target = ChronoTime;

    fn deref(&self) -> &ChronoTime {
        &self.0
    }
}

impl DateTimeMessage {
    pub fn to_single_pointer(self) -> SingularPtrField<Self> {
        SingularPtrField::some(self)
    }
}

impl From<DateTimeMessage> for DateTime {
    fn from(msg: DateTimeMessage) -> Self {
        DateTime(Utc.ymd(msg.year, msg.month, msg.day)
            .and_hms_milli(msg.hour, msg.minute, msg.second, msg.millisecond))
    }
}

impl From<DateTime> for DateTimeMessage {
    fn from(datetime: DateTime) -> Self {
        Self {
            year: datetime.year(),
            month: datetime.month(),
            day: datetime.day(),
            hour: datetime.hour(),
            minute: datetime.minute(),
            second: datetime.second(),
            millisecond: datetime.nanosecond() / 1_000_000,
            ..Self::default()
        }
    }
}

impl From<DateMessage> for DateTime {
    fn from(msg: DateMessage) -> Self {
        DateTime(Utc.ymd(msg.year, msg.month, msg.day).and_hms(0, 0, 0))
    }
}

impl From<DateTime> for DateMessage {
    fn from(datetime: DateTime) -> Self {
        Self {
            year: datetime.year(),
            month: datetime.month(),
            day: datetime.day(),
            ..Self::default()
        }
    }
}

impl From<DateTime> for SingularPtrField<DateTimeMessage> {
    fn from(datetime: DateTime) -> Self {
        SingularPtrField::some(datetime.into())
    }
}

impl From<SingularPtrField<DateTimeMessage>> for DateTime  {
    fn from(field: SingularPtrField<DateTimeMessage>) -> Self {
        Self::from(field.clone().unwrap_or_default())
    }
}

impl From<DateTime> for SingularPtrField<DateMessage> {
    fn from(datetime: DateTime) -> Self {
        SingularPtrField::some(datetime.into())
    }
}

impl From<SingularPtrField<DateMessage>> for DateTime  {
    fn from(field: SingularPtrField<DateMessage>) -> Self {
        Self::from(field.clone().unwrap_or_default())
    }
}

impl From<TimeMessage> for Time {
    fn from(msg: TimeMessage) -> Self {
        Time(ChronoTime::from_hms_milli(msg.hour, msg.minute, msg.second, msg.millisecond))
    }
}

impl From<Time> for TimeMessage {
    fn from(time: Time) -> Self {
        Self {
            hour: time.hour(),
            minute: time.minute(),
            second: time.second(),
            millisecond: time.nanosecond() / 1_000_000,
            ..Self::default()
        }
    }
}

impl From<Time> for SingularPtrField<TimeMessage> {
    fn from(datetime: Time) -> Self {
        SingularPtrField::some(datetime.into())
    }
}

impl From<SingularPtrField<TimeMessage>> for Time  {
    fn from(field: SingularPtrField<TimeMessage>) -> Self {
        Self::from(field.clone().unwrap_or_default())
    }
}
