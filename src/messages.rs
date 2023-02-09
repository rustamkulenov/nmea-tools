use std::{borrow::Cow, collections::HashMap};

pub trait MessageFields<'buf> {
    fn set_field(&mut self, idx: u8, value: &'buf [u8]) {
        self.get_field_mut(idx).set_from_slice(value);
    }

    fn get_field_mut(&mut self, idx: u8) -> &mut (dyn FromSlice<'buf> + 'buf);
    fn field_count(&self) -> u8;
    fn clear(&mut self);
    fn get_addr(&self) -> AddrField<'buf>;
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
pub struct MessagesMap<'a, 'buf: 'a> {
    /* Key shall have references with lifetime 'a. Actually it will be 'static.
       Values shall be structs implementing MessageFields<'a> with lifetime 'buf. I.e. shall live as long as buffer
       of parser.
    */
    pub msgs: HashMap<AddrField<'a>, Box<dyn MessageFields<'buf> + 'buf>>,
}

impl<'a, 'buf> MessagesMap<'a, 'buf> {
    pub fn get(
        &self,
        addr: AddrField<'buf>,
    ) -> Option<&Box<dyn MessageFields<'buf> + 'buf>> {
        self.msgs.get(&addr)
    }

    pub fn get_mut(
        &'_ mut self,
        addr: AddrField<'buf>,
    ) -> Option<&mut Box<dyn MessageFields<'buf> + 'buf>> {
        self.msgs.get_mut(&addr)
    }

    pub fn new() -> Self {
        MessagesMap {
            msgs: HashMap::new(),
        }
    }
}

//************************ Common for all types used in NMEA
pub trait FromSlice<'a> {
    fn set_from_slice(&'_ mut self, value: &'a [u8]);
    fn as_string(&self) -> Cow<'a, str>;
}

impl<'a> FromSlice<'a> for Option<&'a str> {
    fn set_from_slice(&mut self, value: &'a [u8]) {
        *self = Some(std::str::from_utf8(value).unwrap());
    }

    fn as_string(&self) -> Cow<'a, str> {
        match *self {
            Some(v) => Cow::Borrowed(v),
            None => Cow::Borrowed("-"),
        }
    }
}

impl<'a> FromSlice<'a> for f64 {
    fn set_from_slice(&mut self, value: &'a [u8]) {
        *self = std::str::from_utf8(value).unwrap().parse().unwrap();
    }

    fn as_string(&self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}

impl<'a> FromSlice<'a> for Option<f64> {
    fn set_from_slice(&mut self, value: &'a [u8]) {
        if value.len() == 0 {
            *self = None
        };
        *self = Some(std::str::from_utf8(value).unwrap().parse().unwrap());
    }

    fn as_string(&self) -> Cow<'a, str> {
        match *self {
            Some(v) => Cow::Owned(v.to_string()),
            None => Cow::Borrowed(""),
        }
    }
}

impl<'a> FromSlice<'a> for u8 {
    fn set_from_slice(&mut self, value: &'a [u8]) {
        assert!(value.len() == 1);
        *self = value[0];
    }

    fn as_string(&self) -> Cow<'a, str> {
        Cow::Owned(self.to_string())
    }
}

impl<'a> FromSlice<'a> for Option<u8> {
    fn set_from_slice(&mut self, value: &'a [u8]) {
        if value.len() == 0 {
            *self = None
        };
        *self = Some(value[0]);
    }

    fn as_string(&self) -> Cow<'a, str> {
        match *self {
            Some(v) => Cow::Owned(v.to_string()),
            None => Cow::Borrowed(""),
        }
    }
}
