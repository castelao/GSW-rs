// use core::ops::Deref;
use heapless::{FnvIndexMap, String, Vec};
use postcard::{from_bytes, to_vec};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Deref;

// Structure of the validation dataset
#[derive(Serialize, Deserialize, Debug)]
struct DataRef {
    version: String<8>,
    src: String<32>,
    src_md5: String<32>,
    data_x: FnvIndexMap<String<24>, Vec<f64, 3>, 4>,
    data2d: FnvIndexMap<String<24>, Vec<Vec<f64, 45>, 3>, 32>,
}

fn main() {
    let file = std::fs::File::open("data/gsw_data_v3_0.chck_cast.mat").unwrap();
    let mat_file = matfile::MatFile::parse(file).unwrap();

    let mut data_x = FnvIndexMap::<heapless::String<24>, _, 4>::new();
    let mut data2d = FnvIndexMap::<heapless::String<24>, _, 32>::new();

    for variable in mat_file.arrays() {
        // dbg!(variable.name());
        // dbg!(variable.size());
        if let Some(array) = mat_file.find_by_name(variable.name()) {
            if let matfile::NumericData::Double { real, imag: _ } = array.data() {
                let varname: String<24> = String::from(variable.name());
                match array.size()[..] {
                    [1, 3] => {
                        // dbg!(array.name());
                        let values: Vec<f64, 3> = Vec::from_slice(&real).unwrap();
                        data_x.insert(varname, values).unwrap();
                    }
                    [45, 3] => {
                        // dbg!(array.name());
                        let values: Vec<_, 3> = real
                            .chunks(45)
                            .map(|x| Vec::<f64, 45>::from_slice(x).unwrap())
                            .collect();

                        data2d.insert(varname, values).unwrap();
                    }
                    _ => {
                        dbg!("MISSING");
                        dbg!(varname);
                        ()
                    }
                };
            };
        }
    }

    let dataset = DataRef {
        version: String::from("3.06.12"),
        src: String::from("gsw_data_v3_0.mat"),
        src_md5: String::from("f9b65955407a8ed00246de4871ef0505"),
        data_x,
        data2d,
    };

    // dbg!("{:?}", &dataset);

    let stream: Vec<u8, 100_000> = to_vec(&dataset).expect("Failed to vectorize dataset");

    let f = File::create("gsw_validation.bin").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(stream.deref()).expect("Unable to write data");
}
