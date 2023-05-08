use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::collections::HashMap;
use std::time::Instant;
use num_enum::TryFromPrimitive;
struct Coordinates {
    x: f32,
    y: f32,
    z: f32,
    rx: f32,
    ry: f32,
    rz: f32,
    rot: u16,
}

enum Types {
    Coordinates(Coordinates),
    Float(f32),
    U16(u16),
    Dict(HashMap<u16, Option<Types>>),
}
#[derive(TryFromPrimitive)]
#[repr(u16)]
enum TypeEncoding{
    Coordinates=100,
    Float=8,
    U16=1
}

fn main() {
    let coor = Coordinates {
        x: 1.0,
        y: 3.0,
        z: 5.0,
        rx: 0.0,
        ry: 0.0,
        rz: 0.0,
        rot: 5,
    };
    let now = Instant::now();

    let message: Bytes = encode_to_keyid(coor, 1);
    println!("{}", now.elapsed().as_micros());

    let now = Instant::now();
    let x = decode_from_keyid(message);
    println!("{}", now.elapsed().as_micros());

    match x {
        Some(Types::Dict(dict)) => match &dict[&1] {
            Some(Types::Coordinates(coordinates)) => {
                println!("{}", coordinates.y)
            }
            _ => {
                println!("something else",)
            }
        },
        _ => {
            println!("hello ")
        }
    }
}
fn encode_to_keyid(coor: Coordinates, key_id: u16) -> Bytes {
    let mut body = BytesMut::with_capacity(1024);

    body.put(encode_f32(coor.x, 1));
    body.put(encode_f32(coor.y, 2));
    body.put(encode_f32(coor.z, 3));
    body.put(encode_f32(coor.rx, 4));
    body.put(encode_f32(coor.ry, 5));
    body.put(encode_f32(coor.rz, 6));
    body.put(encode_u16(coor.rot, 7));

    let mut message: BytesMut = BytesMut::with_capacity(6 + body.len());

    message.put_u16_ne(key_id);
    message.put_u16_ne(100);
    message.put_u16_ne(body.len() as u16);
    message.put(body);
    return message.freeze();
}
fn decode_from_keyid(mut message: Bytes) -> Option<Types> {
    let mut values = HashMap::new();

    while message.remaining() > 0 {
        let value: Option<Types>;
        let key_id = message.get_u16_ne();
        let encoding:TypeEncoding = match TypeEncoding::try_from_primitive(message.get_u16_ne()){
            Ok(x)=>{x},
            Err(x)=>{panic!("Bad input")}
        };
        
        let length = message.get_u16_ne() as usize;
        let pair = message.split_to(length);

        match encoding {
            TypeEncoding::Float => {
                value = Some(Types::Float(decode_f32(pair)));
            },
            TypeEncoding::U16 => {
                value = Some(Types::U16(decode_u16(pair)));
            },
            TypeEncoding::Coordinates => {
                let object_ = decode_from_keyid(pair);
                value = match object_ {
                    Some(Types::Dict(dict)) => Some(Types::Coordinates(decode_coordinates(dict))),
                    _ => Some(Types::Coordinates(Coordinates {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        rx: 0.0,
                        ry: 0.0,
                        rz: 0.0,
                        rot: 0,
                    })),
                };
            },
            _ => {
                panic!("Unrecongnized Type");
            }
        }
        values.insert(key_id, value);
    }
    return Some(Types::Dict(values));
}
fn encode_u16(value: u16, key_id: u16) -> Bytes {
    let element_size: u16 = 2;
    let capacity: u16 = element_size * 1;
    let encoding: u16 = 1;
    let mut data = BytesMut::with_capacity(capacity.into());
    data.put_u16_ne(key_id);
    data.put_u16_ne(encoding);
    data.put_u16_ne(element_size * 1);
    data.put_u16_ne(value);
    return data.freeze();
}
fn decode_u16(mut message: Bytes) -> u16 {
    let value: u16 = message.get_u16_ne();
    return value;
}
fn encode_f32(value: f32, key_id: u16) -> Bytes {
    let element_size: u16 = 4;
    let capacity: u16 = element_size * 1;
    let encoding: u16 = 8;
    let mut data = BytesMut::with_capacity(capacity.into());
    data.put_u16_ne(key_id);
    data.put_u16_ne(encoding);
    data.put_u16_ne(element_size * 1);
    data.put_f32_ne(value);

    return data.freeze();
}
fn decode_f32(mut message: Bytes) -> f32 {
    let value: f32 = message.get_f32_ne();
    return value;
}
fn decode_coordinates(object: HashMap<u16, Option<Types>>) -> Coordinates {
    let mut coordinates = Coordinates {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        rx: 0.0,
        ry: 0.0,
        rz: 0.0,
        rot: 0,
    };

    coordinates.x = match &object[&1] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 0");
            0.0
        }
    };
    coordinates.y = match &object[&2] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 1");
            0.0
        }
    };
    coordinates.z = match &object[&3] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 2");
            0.0
        }
    };
    coordinates.rx = match &object[&4] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 3");
            0.0
        }
    };

    coordinates.ry = match &object[&5] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 4");
            0.0
        }
    };
    coordinates.rz = match &object[&6] {
        Some(Types::Float(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 5");
            0.0
        }
    };
    coordinates.rot = match &object[&7] {
        Some(Types::U16(x)) => *x,
        _ => {
            print!("Couldn't find Key ID: 7");
            0
        }
    };

    return coordinates;
}
