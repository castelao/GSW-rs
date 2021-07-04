use std::collections::HashMap;

fn main() {
    let mut dataset = HashMap::new();

    let p: Vec<Vec<f64>> = vec![vec![
        0.0, 10., 20., 30., 40., 50., 76., 101., 126., 151., 176., 202.,
    ]];
    dataset.insert("p".to_string(), p);

    let sa: Vec<Vec<f64>> = vec![vec![
        34.4682, 34.4981, 34.5066, 34.5387, 34.5398, 34.5375, 34.7361, 34.9891, 35.1204, 35.1193,
    ]];
    dataset.insert("sa".to_string(), sa);

    let ct: Vec<Vec<f64>> = vec![vec![
        27.9964, 27.9939, 27.9440, 27.9484, 27.8838, 27.7933, 26.9473, 25.4643, 23.3798, 20.6385,
        18.1439,
    ]];
    dataset.insert("ct".to_string(), ct);

    let specvol: Vec<Vec<f64>> = vec![vec![
        0.9786, 0.9785, 0.9785, 0.9784, 0.9783, 0.9783, 0.9778, 0.9770, 0.9762, 0.9754, 0.9748,
    ]];
    dataset.insert("specvol".to_string(), specvol);
}
