// use core::ops::Deref;
use heapless::{FnvIndexMap, String, Vec};
use postcard::{from_bytes, to_vec};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Deref;

fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    struct DataRef {
        version: String<8>,
        data: FnvIndexMap<String<8>, Vec<f64, 8>, 4>,
    }

    let mut data = FnvIndexMap::<_, _, 4>::new();

    let p: Vec<f64, 8> = Vec::from_slice(&[0.0, 10., 20., 30., 40., 50., 76., 101.]).unwrap();
    data.insert(String::from("p"), p).unwrap();

    let sa: Vec<f64, 8> = Vec::from_slice(&[
        34.468236430490606,
        34.498127066969268,
        34.506638187758568,
        34.53868101623447,
        34.539777830353195,
        34.537494144119954,
        34.73609471241204,
        34.989117262929604,
    ])
    .unwrap();
    data.insert(String::from("sa"), sa).unwrap();

    let ct: Vec<f64, 8> = Vec::from_slice(&[
        27.996436412058223,
        27.993857176639093,
        27.944017615967987,
        27.948372399994330,
        27.883806497845711,
        27.793319415508925,
        26.947345421342511,
        25.464306249345810,
    ])
    .unwrap();
    data.insert(String::from("ct"), ct).unwrap();

    let specvol: Vec<f64, 8> = Vec::from_slice(&[
        0.000978582446643,
        0.000978520259453,
        0.000978458407133,
        0.000978396896780,
        0.000978335707723,
        0.000978268875230,
        0.000977764021427,
        0.000977040353397,
    ])
    .unwrap();
    data.insert(String::from("specvol"), specvol).unwrap();

    let dataset = DataRef {
        version: String::from("3.06.12"),
        data,
    };

    let stream: Vec<u8, 300> = to_vec(&dataset).expect("Failed to vectorize dataset");

    let f = File::create("gsw_validation.bin").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(stream.deref()).expect("Unable to write data");
}
