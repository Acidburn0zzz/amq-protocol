use value::AMQPValue;

use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPType {
    Boolean,
    ShortShortInt,
    ShortShortUInt,
    ShortInt,
    ShortUInt,
    LongInt,
    LongUInt,
    LongLongInt,
    LongLongUInt,
    Float,
    Double,
    DecimalValue,
    ShortString,
    LongString,
    FieldArray,
    Timestamp,
    FieldTable,
    ByteArray, /* ByteArray is specific to RabbitMQ */
    Void,
}

impl AMQPType {
    pub fn from_id(id: char) -> Option<AMQPType> {
        match id {
            't' => Some(AMQPType::Boolean),
            'b' => Some(AMQPType::ShortShortInt),
            'B' => Some(AMQPType::ShortShortUInt),
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            's' => Some(AMQPType::ShortInt),
            'U' => Some(AMQPType::ShortInt),
            'u' => Some(AMQPType::ShortUInt),
            'I' => Some(AMQPType::LongInt),
            'i' => Some(AMQPType::LongUInt),
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            'L' => Some(AMQPType::LongLongInt),
            'l' => Some(AMQPType::LongLongInt),
            'f' => Some(AMQPType::Float),
            'd' => Some(AMQPType::Double),
            'D' => Some(AMQPType::DecimalValue),
            'S' => Some(AMQPType::LongString),
            'A' => Some(AMQPType::FieldArray),
            'T' => Some(AMQPType::Timestamp),
            'F' => Some(AMQPType::FieldTable),
            'x' => Some(AMQPType::ByteArray),
            'V' => Some(AMQPType::Void),
            _   => None,
        }
    }

    pub fn get_id(&self) -> char {
        match *self {
            AMQPType::Boolean        => 't',
            AMQPType::ShortShortInt  => 'b',
            AMQPType::ShortShortUInt => 'B',
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            AMQPType::ShortInt       => 's',
            AMQPType::ShortUInt      => 'u',
            AMQPType::LongInt        => 'I',
            AMQPType::LongUInt       => 'i',
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            AMQPType::LongLongInt    => 'l',
            AMQPType::LongLongUInt   => 'l',
            AMQPType::Float          => 'f',
            AMQPType::Double         => 'd',
            AMQPType::DecimalValue   => 'D',
            /* ShortString only exists for internal usage, we shouldn't ever have to use this */
            AMQPType::ShortString    => '_',
            AMQPType::LongString     => 'S',
            AMQPType::FieldArray     => 'A',
            AMQPType::Timestamp      => 'T',
            AMQPType::FieldTable     => 'F',
            AMQPType::ByteArray      => 'x',
            AMQPType::Void           => 'V',
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for AMQPType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Boolean        = bool;
pub type ShortShortInt  = i8;
pub type ShortShortUInt = u8;
pub type ShortInt       = i16;
pub type ShortUInt      = u16;
pub type LongInt        = i32;
pub type LongUInt       = u32;
pub type LongLongInt    = i64;
pub type LongLongUInt   = u64;
pub type Float          = f32;
pub type Double         = f64;
pub type ShortString    = String;
pub type LongString     = String;
pub type FieldArray     = Vec<AMQPValue>;
pub type Timestamp      = LongLongUInt;
pub type FieldTable     = BTreeMap<ShortString, AMQPValue>;
pub type ByteArray      = Vec<u8>;
pub type Void           = ();

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecimalValue {
    pub scale: ShortShortUInt,
    pub value: LongUInt,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_from_id() {
        assert_eq!(AMQPType::from_id('T'), Some(AMQPType::Timestamp));
        assert_eq!(AMQPType::from_id('S'), Some(AMQPType::LongString));
        assert_eq!(AMQPType::from_id('s'), Some(AMQPType::ShortInt));
        assert_eq!(AMQPType::from_id('z'), None);
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(AMQPType::Boolean.to_string(), "Boolean");
        assert_eq!(AMQPType::Void.to_string(),    "Void");
    }
}
