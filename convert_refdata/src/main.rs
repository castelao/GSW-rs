// use core::ops::Deref;
use heapless::FnvIndexMap;
use postcard::to_vec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufWriter, Write};
use std::ops::Deref;
use std::path::PathBuf;

// Structure of the validation dataset
#[derive(Serialize, Deserialize, Debug)]
struct DataRef {
    // created_at
    version: heapless::String<8>,
    src: heapless::String<32>,
    src_md5: heapless::String<32>,
    scalar: FnvIndexMap<heapless::String<64>, f64, 8>,
    data_x: FnvIndexMap<heapless::String<64>, heapless::Vec<f64, 3>, 2>,
    data2d: FnvIndexMap<heapless::String<64>, heapless::Vec<heapless::Vec<f64, 45>, 3>, 32>,
}

fn main() {
    let file = std::fs::File::open("data/gsw_data_v3_0.chck_cast.mat").unwrap();
    let mat_file = matfile::MatFile::parse(file).unwrap();

    let paths: Vec<PathBuf> = std::fs::read_dir("data/mods/")
        .unwrap()
        .filter_map(|f| f.ok())
        .map(|f| f.path())
        .filter(|f| f.is_file())
        .collect();

    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for p in paths.into_iter() {
        let f = std::fs::File::open(&p).unwrap();
        let reader = std::io::BufReader::new(f);
        let varlist = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
        groups.insert(p.file_name().unwrap().to_str().unwrap().into(), varlist);
    }
    dbg!(&groups);

    for (group, varnames) in groups {
        let mut scalar = FnvIndexMap::<heapless::String<64>, _, 8>::new();
        let mut data_x = FnvIndexMap::<heapless::String<64>, _, 2>::new();
        let mut data2d = FnvIndexMap::<heapless::String<64>, _, 32>::new();

        let mut scalar_size: isize = 0;
        let mut datax_size: isize = 0;
        let mut datax2_size: isize = 0;
        for variable in mat_file.arrays() {
            //dbg!(variable.name());
            //dbg!(variable.size());
            if let Some(array) = mat_file.find_by_name(variable.name()) {
                if let matfile::NumericData::Double { real, imag: _ } = array.data() {
                    if varnames.contains(&variable.name().to_string()) {
                        let varname: heapless::String<64> = heapless::String::from(variable.name());
                        match array.size()[..] {
                            [1, 1] => {
                                scalar_size += 1;
                                let value: f64 = real[0];
                                scalar.insert(varname, value).unwrap();
                            }
                            [1, 3] => {
                                datax_size += 1;
                                let values: heapless::Vec<f64, 3> =
                                    heapless::Vec::from_slice(&real).unwrap();
                                data_x.insert(varname, values).unwrap();
                            }
                            [45, 3] => {
                                datax2_size += 1;
                                let values: heapless::Vec<_, 3> = real
                                    .chunks(45)
                                    .map(|x| heapless::Vec::<f64, 45>::from_slice(x).unwrap())
                                    .collect();

                                data2d.insert(varname, values).unwrap();
                            }
                            _ => {
                                /*
                                dbg!("MISSING");
                                dbg!(varname);
                                dbg!(array.size());
                                */
                                ()
                            }
                        }
                    };
                };
            }
        }
        dbg!("DATA SCALAR", &scalar_size);
        dbg!("DATA 1D", &datax_size);
        dbg!("DATA 2D", &datax2_size);

        // let nd_arr: ndarray::ArrayD<f64> = specvol.try_into().expect("Fail converting into array");
        /*
        let gsw_cv = mat_file.find_by_name("gsw_cv");
        println!("{:#?}", gsw_cv);
        */

        let dataset = DataRef {
            version: heapless::String::from("3.06.16"),
            src: heapless::String::from("gsw_data_v3_0.mat"),
            src_md5: heapless::String::from("489365f8be6f2b7868f94fb0ccd1e8c2"),
            scalar,
            data_x,
            data2d,
        };

        let stream: heapless::Vec<u8, 100_000> =
            to_vec(&dataset).expect("Failed to vectorize dataset");
        // let serialized = serde_json::to_string(&dataset).unwrap();

        let outputfilename = format!("data/gsw_{}_validation.bin", group);
        let f = File::create(dbg!(outputfilename)).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        f.write_all(stream.deref()).expect("Unable to write data");
    }
}
