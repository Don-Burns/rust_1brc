use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufRead, BufReader, SeekFrom},
    os::unix::fs::{FileExt, MetadataExt},
    sync::{atomic::AtomicU64, Arc, Mutex},
    thread,
};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
const CHUNK_SIZE: u64 = 500000;

struct CityLine {
    name: String,
    temperature: f32,
}

impl CityLine {
    fn new(name: String, temperature: f32) -> CityLine {
        CityLine { name, temperature }
    }

    // TODO: consider what to do if data is malformed
    fn from_string(data: &str) -> CityLine {
        let parts: Vec<&str> = data.split(';').collect();
        let name = parts[0].to_string();
        let temperature = parts[1].parse().unwrap();
        CityLine::new(name, temperature)
    }
}

struct CityMeasurements {
    name: String,
    min: f32,
    max: f32,
    sum: f64,
    count: u64,
}

impl CityMeasurements {
    fn new(name: String) -> CityMeasurements {
        CityMeasurements {
            name,
            min: 0.0,
            max: 0.0,
            sum: 0.0,
            count: 0,
        }
    }

    fn to_string(&self) -> String {
        let mut average = self.sum / self.count as f64;
        // round to 1 decimal place always rounding up
        average = ((average * 10.0) + 0.4).round() / 10.0;

        format!(
            "{}={:.1}/{:.1}/{:.1}",
            self.name, self.min, average, self.max
        )
    }
}

/// Split the file into chunks based on the number of threads that will be used
fn chunk_file<'a>(data: &File, offset: u64, buffer: &'a mut [u8]) -> Result<BufReader<&'a [u8]>> {
    let metadata = data.metadata()?;
    let file_size = metadata.size();
    if offset > file_size {
        return Ok(BufReader::new(&[]));
    }
    dbg!(file_size);
    dbg!(offset);
    dbg!(buffer.len());
    let mut head = offset;
    data.read_exact_at(buffer, offset)?;

    // step back to the last newline character
    while head > 0 {
        if buffer[head as usize] == b'\n' {
            break;
        }
        head -= 1;
    }
    dbg!(head);

    let mut tail = min(CHUNK_SIZE, file_size);
    while tail > head {
        tail -= 1;
        if buffer[tail as usize] == b'\n' {
            break;
        }
    }
    dbg!(tail);

    Ok(BufReader::new(&buffer[head as usize..tail as usize]))
}

pub fn process_data(data: File) -> String {
    let mut buffer = vec![0; min(data.metadata().unwrap().size(), CHUNK_SIZE) as usize];
    let chunk_file = chunk_file(&data, 0, &mut buffer);
    let city_lines_iterator = chunk_file.expect("Failed to chunk file")
        .lines()
        .map(|line| CityLine::from_string(line.unwrap().as_str()));

    let mut map: HashMap<String, CityMeasurements> = HashMap::new();

    let city_list = city_lines_iterator
        .map(|city| {
            let city_measurements = map
                .entry(city.name.clone())
                .or_insert(CityMeasurements::new(city.name.clone()));
            city_measurements.count += 1;
            let cur_temp = city.temperature;
            city_measurements.sum += cur_temp as f64;
            if city_measurements.count == 1 {
                city_measurements.min = cur_temp;
                city_measurements.max = cur_temp;
            } else {
                if cur_temp < city_measurements.min {
                    city_measurements.min = cur_temp;
                }
                if cur_temp > city_measurements.max {
                    city_measurements.max = cur_temp;
                }
            }

            city.name
        })
        .collect::<HashSet<_>>();

    let mut ordered_city_list = city_list.into_iter().collect::<Vec<_>>();
    ordered_city_list.sort();

    format!(
        "{{{}}}\n",
        ordered_city_list
            .into_iter()
            .map(|city_name| {
                let city_measurements = map.get(&city_name).unwrap();
                city_measurements.to_string()
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

mod tests {

    use super::*;
    use crate::threads::read::read;

    #[test]
    fn test_process_data_1brc_repo_samples() {
        use std::fs;
        use std::path::Path;

        let test_path = Path::new("./src/test_data/samples").canonicalize().unwrap();

        let files = fs::read_dir(test_path.canonicalize().unwrap())
            .expect("Unable to read test_data/samples directory");

        for file in files {
            let file = file.expect("Failed to read file").path();
            let file_extension = file.extension();
            match file_extension {
                Some(ext) => match ext.to_str() {
                    Some("txt") => {
                        let data = read(file.to_str().unwrap());
                        let file_name = file
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .split(".")
                            .collect::<Vec<_>>()[0];

                        let out_file_path = test_path.join(format!("{}.out", file_name));
                        let expected = fs::read_to_string(&out_file_path).expect(
                            format!("Could not read expected output file {:?}", out_file_path)
                                .as_str(),
                        );
                        let result = process_data(data);
                        assert_eq!(result, expected, "Failed for file {:?}", file);
                        println!("Passed for file {:?}", file);
                    }
                    _ => continue,
                },
                None => continue,
            }
        }
    }
}
