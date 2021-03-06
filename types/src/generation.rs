use flags::*;
use types::*;
use value::*;

use cookie_factory::*;

pub fn gen_raw_value<'a>(x: (&'a mut [u8], usize), v: &AMQPValue) -> Result<(&'a mut [u8], usize), GenError> {
    match *v {
        AMQPValue::Boolean(ref b)        => gen_boolean(x, b),
        AMQPValue::ShortShortInt(ref i)  => gen_short_short_int(x, i),
        AMQPValue::ShortShortUInt(ref u) => gen_short_short_uint(x, u),
        AMQPValue::ShortInt(ref i)       => gen_short_int(x, i),
        AMQPValue::ShortUInt(ref u)      => gen_short_uint(x, u),
        AMQPValue::LongInt(ref i)        => gen_long_int(x, i),
        AMQPValue::LongUInt(ref u)       => gen_long_uint(x, u),
        AMQPValue::LongLongInt(ref i)    => gen_long_long_int(x, i),
        AMQPValue::Float(ref f)          => gen_float(x, f),
        AMQPValue::Double(ref d)         => gen_double(x, d),
        AMQPValue::DecimalValue(ref d)   => gen_decimal_value(x, d),
        AMQPValue::LongString(ref s)     => gen_long_string(x, s),
        AMQPValue::FieldArray(ref a)     => gen_field_array(x, a),
        AMQPValue::Timestamp(ref t)      => gen_timestamp(x, t),
        AMQPValue::FieldTable(ref t)     => gen_field_table(x, t),
        AMQPValue::ByteArray(ref a)      => gen_byte_array(x, a),
        AMQPValue::Void                  => Ok(x),
    }
}

pub fn gen_value<'a>(x: (&'a mut [u8], usize), v: &AMQPValue) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_type(&v.get_type()) >> gen_raw_value(v))
}

pub fn gen_type<'a>(x: (&'a mut [u8], usize), t: &AMQPType) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, t.get_id() as u8)
}

pub fn gen_id<'a>(x: (&'a mut [u8], usize), id: &ShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_short_uint(x, id)
}

pub fn gen_boolean<'a>(x: (&'a mut [u8], usize), b: &Boolean) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, if *b { 1 } else { 0 })
}

pub fn gen_short_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i8!(x, *i)
}

pub fn gen_short_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, *u)
}

pub fn gen_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i16!(x, *i)
}

pub fn gen_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u16!(x, *u)
}

pub fn gen_long_int<'a>(x: (&'a mut [u8], usize), i: &LongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i32!(x, *i)
}

pub fn gen_long_uint<'a>(x: (&'a mut [u8], usize), u: &LongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u32!(x, *u)
}

pub fn gen_long_long_int<'a>(x: (&'a mut [u8], usize), i: &LongLongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i64!(x, *i)
}

pub fn gen_long_long_uint<'a>(x: (&'a mut [u8], usize), i: &LongLongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u64!(x, *i)
}

pub fn gen_float<'a>(x: (&'a mut [u8], usize), f: &Float) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_f32!(x, *f)
}

pub fn gen_double<'a>(x: (&'a mut [u8], usize), d: &Double) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_f64!(x, *d)
}

pub fn gen_decimal_value<'a>(x: (&'a mut [u8], usize), d: &DecimalValue) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(&d.scale) >> gen_long_uint(&d.value))
}

pub fn gen_short_string<'a>(x: (&'a mut [u8], usize), s: &ShortString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(&(s.len() as ShortShortUInt)) >> gen_slice!(s.as_bytes()))
}

pub fn gen_long_string<'a>(x: (&'a mut [u8], usize), s: &LongString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(&(s.len() as LongUInt)) >> gen_slice!(s.as_bytes()))
}

pub fn gen_field_array<'a>(x: (&'a mut [u8], usize), a: &FieldArray) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        len:   gen_skip!(4)                >>
        start: gen_many_ref!(a, gen_value) >>
        end:   gen_at_offset!(len, gen_long_uint(&((end - start) as LongUInt)))
    )
}

pub fn gen_timestamp<'a>(x: (&'a mut [u8], usize), t: &Timestamp) -> Result<(&'a mut [u8], usize), GenError> {
    gen_long_long_uint(x, t)
}

pub fn gen_field_table<'a>(x: (&'a mut [u8], usize), t: &FieldTable) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        len:   gen_skip!(4)                      >>
        start: gen_many_ref!(t, gen_field_entry) >>
        end:   gen_at_offset!(len, gen_long_uint(&((end - start) as LongUInt)))
    )
}

fn gen_field_entry<'a>(x: (&'a mut [u8], usize), e: &(&LongString, &AMQPValue)) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_string(e.0) >> gen_value(e.1))
}

pub fn gen_byte_array<'a>(x: (&'a mut [u8], usize), a: &ByteArray) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(&(a.len() as LongUInt)) >> gen_slice!(a))
}

pub fn gen_flags<'a>(x: (&'a mut [u8], usize), f: &AMQPFlags) -> Result<(&'a mut [u8], usize), GenError> {
    f.get_bytes().iter().fold(Ok(x), |acc: Result<(&'a mut [u8], usize), GenError>, b| {
        acc.and_then(|x| gen_be_u8!(x, *b))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_raw_value() {
        assert_eq!(gen_raw_value((&mut [0, 0, 0, 0], 0), &AMQPValue::LongInt(42)),   Ok((&mut [0, 0, 0, 42][..], 4)));
        assert_eq!(gen_raw_value((&mut [0],          0), &AMQPValue::Boolean(true)), Ok((&mut [1][..],           1)));
    }

    #[test]
    fn test_gen_value() {
        assert_eq!(gen_value((&mut [0, 0, 0, 0, 0], 0), &AMQPValue::LongInt(42)),   Ok((&mut [73,  0, 0, 0, 42][..], 5)));
        assert_eq!(gen_value((&mut [0, 0],          0), &AMQPValue::Boolean(true)), Ok((&mut [116, 1][..],           2)));
    }

    #[test]
    fn test_gen_type() {
        assert_eq!(gen_type((&mut [0], 0), &AMQPType::ShortShortInt), Ok((&mut [98][..],  1)));
        assert_eq!(gen_type((&mut [0], 0), &AMQPType::ShortInt),      Ok((&mut [115][..], 1)));
    }

    #[test]
    fn test_gen_id() {
        assert_eq!(gen_id((&mut [0, 0], 0), &0),     Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_id((&mut [0, 0], 0), &65535), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(gen_boolean((&mut [0], 0), &false), Ok((&mut [0][..], 1)));
        assert_eq!(gen_boolean((&mut [0], 0), &true),  Ok((&mut [1][..], 1)));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(gen_short_short_int((&mut [0], 0), &0),  Ok((&mut [0][..],   1)));
        assert_eq!(gen_short_short_int((&mut [0], 0), &-1), Ok((&mut [255][..], 1)));
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(gen_short_short_uint((&mut [0], 0), &0),   Ok((&mut [0][..],   1)));
        assert_eq!(gen_short_short_uint((&mut [0], 0), &255), Ok((&mut [255][..], 1)));
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(gen_short_int((&mut [0, 0], 0), &0),  Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_short_int((&mut [0, 0], 0), &-1), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(gen_short_uint((&mut [0, 0], 0), &0),     Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_short_uint((&mut [0, 0], 0), &65535), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), &0),  Ok((&mut [0,   0,   0,   0][..],   4)));
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), &-1), Ok((&mut [255, 255, 255, 255][..], 4)));
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), &0),          Ok((&mut [0,   0,   0,   0][..],   4)));
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), &4294967295), Ok((&mut [255, 255, 255, 255][..], 4)));
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0),  Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &-1), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0),                    Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &18446744073709551615), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), &0.),    Ok((&mut [0,  0,  0,   0][..],  4)));
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), &42.42), Ok((&mut [66, 41, 174, 20][..], 4)));
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0.),    Ok((&mut [0,  0,  0,  0,   0,   0,  0,  0][..],   8)));
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &42.42), Ok((&mut [64, 69, 53, 194, 143, 92, 40, 246][..], 8)));
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), &DecimalValue { scale: 0, value: 0 }),  Ok((&mut [0, 0, 0, 0, 0][..], 5)));
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), &DecimalValue { scale: 2, value: 42 }), Ok((&mut [2, 0, 0, 0, 42][..], 5)));
    }

    #[test]
    fn test_gen_short_string() {
        assert_eq!(gen_short_string((&mut [0], 0), &"".to_string()),                 Ok((&mut [0][..], 1)));
        assert_eq!(gen_short_string((&mut [0, 0, 0, 0, 0], 0), &"test".to_string()), Ok((&mut [4, 116, 101, 115, 116][..], 5)));
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0], 0), &"".to_string()),                 Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &"test".to_string()), Ok((&mut [0, 0, 0, 4, 116, 101, 115, 116][..], 8)));
    }

    #[test]
    fn test_gen_field_array() {
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0], 0), &FieldArray::new()),                    Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0, 0, 0], 0), &vec![AMQPValue::Boolean(true)]), Ok((&mut [0, 0, 0, 2, 116, 1][..], 6)));
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &0),                    Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &18446744073709551615), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::new();
        table.insert("test".to_string(), AMQPValue::Float(42.42));
        table.insert("tt".to_string(),   AMQPValue::LongLongInt(42));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0],                                                                   0), &FieldTable::new()), Ok((&mut [0, 0, 0, 0][..],                                                                                         4)));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), &table),             Ok((&mut [0, 0, 0, 22, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 2, 116, 116, 108, 0, 0, 0, 0, 0, 0, 0, 42][..], 26)));
    }

    #[test]
    fn test_gen_byte_array() {
        assert_eq!(gen_byte_array((&mut [0, 0, 0, 0], 0), &ByteArray::new()),              Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_byte_array((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &vec![42, 1, 2, 3]), Ok((&mut [0, 0, 0, 4, 42 , 1, 2, 3][..], 8)));
    }

    #[test]
    fn test_gen_flags() {
        let mut flags = AMQPFlags::new();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), true);
        flags.add_flag("d".to_string(), true);
        assert_eq!(gen_flags((&mut [0], 0), &flags), Ok((&mut [0b00001101][..], 1)));
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), false);
        flags.add_flag("g".to_string(), true);
        flags.add_flag("h".to_string(), true);
        flags.add_flag("i".to_string(), false);
        flags.add_flag("j".to_string(), true);
        assert_eq!(gen_flags((&mut [0, 0], 0), &flags), Ok((&mut [0b11011101, 0b00000010][..], 2)));
    }
}
