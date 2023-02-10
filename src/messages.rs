use std::{any::Any, borrow::Cow, collections::HashMap};

pub trait MessageFields: 'static {
    fn set_field(&mut self, idx: u8, value: &[u8]) {
        self.get_field_mut(idx).set_from_slice(value);
    }

    fn get_field_mut(&mut self, idx: u8) -> &mut dyn FromSlice;
    fn field_count(&self) -> u8;
    fn clear(&mut self);
    fn get_addr(&self) -> AddrField<'static>;
    fn as_any(&self) -> &dyn Any;
}

/* Address field */
#[derive(PartialEq, Eq, Hash)]
pub struct AddrField<'a> {
    pub data: &'a [u8],
}

impl<'a> AddrField<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        AddrField { data }
    }
}

impl<'a> AsRef<[u8]> for AddrField<'a> {
    fn as_ref(&self) -> &[u8] {
        self.data
    }
}

/* Map of NMEA messages by addr field */
pub struct MessagesMap<'a> {
    /* Key shall have references with lifetime 'a. Actually it will be 'static.
       Values shall be structs implementing MessageFields<'a> with lifetime 'buf. I.e. shall live as long as buffer
       of parser.
    */
    pub msgs: HashMap<AddrField<'a>, Box<dyn MessageFields + 'static>>,
}

impl<'a> MessagesMap<'a> {
    pub fn get(&self, addr: AddrField<'a>) -> Option<&Box<dyn MessageFields + 'static>> {
        self.msgs.get(&addr)
    }

    pub fn get_mut(
        &mut self,
        addr: AddrField<'a>,
    ) -> Option<&mut Box<dyn MessageFields + 'static>> {
        self.msgs.get_mut(&addr)
    }

    pub fn new() -> Self {
        MessagesMap {
            msgs: HashMap::new(),
        }
    }
}

//************************ Common for all types used in NMEA
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
