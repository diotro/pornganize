use std::iter::FromIterator as _;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use protobuf::{
    RepeatedField,
    SingularPtrField,
    Message,
    ProtobufEnum,
};
use super::messages::common::{
    ApplicableTo as MessageApplicableTo,
    DateTime as DateTimeMessage,
    Date as DateMessage,
    Time as TimeMessage,
};

use chrono::{
    NaiveTime as ChronoTime,
    DateTime as ChronoDateTime,
    Utc,
    TimeZone as _,
    Datelike as _,
    Timelike as _,
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
        field.unwrap_or_default().into()
    }
}

impl From<DateTime> for SingularPtrField<DateMessage> {
    fn from(datetime: DateTime) -> Self {
        SingularPtrField::some(datetime.into())
    }
}

impl From<SingularPtrField<DateMessage>> for DateTime  {
    fn from(field: SingularPtrField<DateMessage>) -> Self {
        field.unwrap_or_default().into()
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
        field.unwrap_or_default().into()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApplicableTo {
    Any = 0,
    Actor = 1,
    Network = 2,
    Site = 3,
    Studio = 4,
    Video = 5,
    Artist = 6,
    Character = 7,
    GameDeveloper = 8,
    Game = 9,
    GameStudio = 10,
}

impl From<MessageApplicableTo> for ApplicableTo {
    fn from(applicable_to: MessageApplicableTo) -> Self {
        match applicable_to {
            MessageApplicableTo::ANY => ApplicableTo::Any,
            MessageApplicableTo::ACTOR => ApplicableTo::Actor,
            MessageApplicableTo::NETWORK => ApplicableTo::Network,
            MessageApplicableTo::SITE => ApplicableTo::Site,
            MessageApplicableTo::STUDIO => ApplicableTo::Studio,
            MessageApplicableTo::VIDEO => ApplicableTo::Video,
            MessageApplicableTo::ARTIST => ApplicableTo::Artist,
            MessageApplicableTo::CHARACTER => ApplicableTo::Character,
            MessageApplicableTo::GAME_DEVELOPER => ApplicableTo::GameDeveloper,
            MessageApplicableTo::GAME => ApplicableTo::Game,
            MessageApplicableTo::GAME_STUDIO => ApplicableTo::GameStudio,
        }
    }
}

impl From<ApplicableTo> for MessageApplicableTo {
    fn from(applicable_to: ApplicableTo) -> Self {
        match applicable_to {
            ApplicableTo::Any => MessageApplicableTo::ANY,
            ApplicableTo::Actor => MessageApplicableTo::ACTOR,
            ApplicableTo::Network => MessageApplicableTo::NETWORK,
            ApplicableTo::Site => MessageApplicableTo::SITE,
            ApplicableTo::Studio => MessageApplicableTo::STUDIO,
            ApplicableTo::Video => MessageApplicableTo::VIDEO,
            ApplicableTo::Artist => MessageApplicableTo::ARTIST,
            ApplicableTo::Character => MessageApplicableTo::CHARACTER,
            ApplicableTo::GameDeveloper => MessageApplicableTo::GAME_DEVELOPER,
            ApplicableTo::Game => MessageApplicableTo::GAME,
            ApplicableTo::GameStudio => MessageApplicableTo::GAME_STUDIO,
        }
    }
}

pub struct MessageVec<M: Message>(pub Vec<M>);

impl<M: Message> MessageVec<M> {
    pub fn from_vec(v: Vec<M>) -> Self {
        Self(v)
    }
}

impl<M: Message> Deref for MessageVec<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M, T> From<Vec<T>> for MessageVec<M> where
    M: From<T> + Message,
{
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(M::from).collect())
    }
}

impl<M, T> From<MessageVec<M>> for Vec<T> where
    T: From<M>,
    M: Message,
{
    fn from(messages: MessageVec<M>) -> Self {
        messages.0.into_iter().map(T::from).collect()
    }
}

pub struct ProtobufEnumVec<M: ProtobufEnum>(pub Vec<M>);

impl<M: ProtobufEnum> ProtobufEnumVec<M> {
    pub fn from_vec(v: Vec<M>) -> Self {
        Self(v)
    }
}

impl<M: ProtobufEnum> Deref for ProtobufEnumVec<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M, T> From<Vec<T>> for ProtobufEnumVec<M> where
    M: From<T> + ProtobufEnum,
{
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(M::from).collect())
    }
}

impl<M, T> From<ProtobufEnumVec<M>> for Vec<T> where
    T: From<M>,
    M: ProtobufEnum,
{
    fn from(values: ProtobufEnumVec<M>) -> Self {
        values.0.into_iter().map(T::from).collect()
    }
}

pub fn to_vec<M: From<T>, T>(from: Vec<T>) -> Vec<M> {
    from.into_iter().map(M::from).collect()
}

pub fn from_vec<M, T: From<M>>(field: Vec<M>) -> Vec<T> {
    field.into_iter().map(T::from).collect()
}

pub fn to_repeated_field<M: From<T>, T>(from: Vec<T>) -> RepeatedField<M> {
    RepeatedField::from_iter(from.into_iter().map(M::from).into_iter())
}

pub fn from_repeated_field<M, T: From<M>>(field: RepeatedField<M>) -> Vec<T> {
    field.into_iter().map(T::from).collect()
}
