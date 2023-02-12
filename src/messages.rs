use std::{
    any::Any,
    borrow::{Borrow, Cow},
    collections::HashMap,
    hash::Hash,
};

use crate::generated::{is_talker_id, nmea3::NmeaMessages};

/// Represents a NMEA message with list of values.
pub trait MessageFields {
    fn set_field(&mut self, idx: u8, value: &[u8]) {
        self.get_field_mut(idx).set_from_slice(value);
    }

    fn get_field_mut(&mut self, idx: u8) -> &mut dyn FromSlice;
    fn field_count(&self) -> u8;
    fn clear(&mut self);
    fn get_addr(&self) -> AddrField<'static>;
    fn as_any(&self) -> &dyn Any;
    fn message_type(&self) -> NmeaMessages;
}

/// Address field. May contain talker_id (e.g. 'GP' or 'GL').
#[derive(PartialEq, Eq)]
pub struct AddrField<'a> {
    /// Message type\address (e.g. 'GLL' or 'RMC').
    pub data: &'a [u8],
    pub talker_id: &'a str,
}

impl<'a> AddrField<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        if data.len() > 3 && is_talker_id(&data[0..2]) {
            AddrField {
                data: &data[2..],
                talker_id: std::str::from_utf8(&data[0..2]).unwrap(),
            }
        } else {
            AddrField {
                data,
                talker_id: "",
            }
        }
    }
}

/// Hash implemented for AddrField to compare only message type without talker_id.
impl<'a> Hash for AddrField<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

/// This trait implemented to fix hash.Get_mut() issue and to get value by &[u8] to reduce lifetime.
impl<'a> Borrow<[u8]> for AddrField<'a> {
    fn borrow(&self) -> &[u8] {
        &self.data
    }
}

/// Map of NMEA messages by address\message type.
pub struct MessagesMap {
    /// Key shall have references with lifetime 'static.
    /// Values shall be structs implementing MessageFields with lifetime 'static.
    pub msgs: HashMap<AddrField<'static>, Box<dyn MessageFields + 'static>>,
}

impl MessagesMap {
    pub fn get(&self, addr: &AddrField<'_>) -> Option<&Box<dyn MessageFields + 'static>> {
        self.msgs.get(addr.data)
    }

    pub fn get_mut(
        &mut self,
        addr: &AddrField<'_>,
    ) -> Option<&mut Box<dyn MessageFields + 'static>> {
        self.msgs.get_mut(addr.data)
    }

    pub fn new() -> Self {
        MessagesMap {
            msgs: HashMap::new(),
        }
    }
}

//************************ Common for all types used in NMEA   ************************************

/// Trait for message fields to set field value regardless of message type.
pub trait FromSlice {
    fn set_from_slice(&mut self, value: &[u8]);
    fn as_string(&self) -> Cow<str>;
}

impl FromSlice for Option<String> {
    fn set_from_slice(&mut self, value: &[u8]) {
        *self = Some(String::from_utf8(value.to_vec()).unwrap());
    }

    fn as_string(&self) -> Cow<str> {
        match self {
            Some(v) => Cow::Borrowed(v),
            None => Cow::Borrowed("-"),
        }
    }
}

impl FromSlice for f64 {
    fn set_from_slice(&mut self, value: &[u8]) {
        *self = std::str::from_utf8(value).unwrap().parse().unwrap();
    }

    fn as_string(&self) -> Cow<str> {
        Cow::Owned(self.to_string())
    }
}

impl FromSlice for Option<f64> {
    fn set_from_slice(&mut self, value: &[u8]) {
        if value.len() == 0 {
            *self = None
        };
        *self = Some(std::str::from_utf8(value).unwrap().parse().unwrap());
    }

    fn as_string(&self) -> Cow<str> {
        match *self {
            Some(v) => Cow::Owned(v.to_string()),
            None => Cow::Borrowed(""),
        }
    }
}

impl FromSlice for u8 {
    fn set_from_slice(&mut self, value: &[u8]) {
        assert!(value.len() == 1);
        *self = value[0];
    }

    fn as_string(&self) -> Cow<str> {
        Cow::Owned(self.to_string())
    }
}

impl FromSlice for Option<u8> {
    fn set_from_slice(&mut self, value: &[u8]) {
        if value.len() == 0 {
            *self = None
        };
        *self = Some(value[0]);
    }

    fn as_string(&self) -> Cow<str> {
        match *self {
            Some(v) => Cow::Owned(v.to_string()),
            None => Cow::Borrowed(""),
        }
    }
}
