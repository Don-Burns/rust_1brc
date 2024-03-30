use std::collections::{HashMap, HashSet};

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

pub fn process_data(data: String) -> String {
    let city_lines_iterator = data.lines().map(|line| CityLine::from_string(line));

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

    #[test]
    fn test_process_data() {
        let data = "Halifax;12.9
Zagreb;12.2
Cabo San Lucas;14.9
Adelaide;15.0
Ségou;25.7
Pittsburgh;9.7
Karachi;15.4
Xi'an;24.2
Dodoma;22.2
Tauranga;38.2
"
        .to_string();
        let expected = "{Adelaide=15.0/15.0/15.0, Cabo San Lucas=14.9/14.9/14.9, Dodoma=22.2/22.2/22.2, Halifax=12.9/12.9/12.9, Karachi=15.4/15.4/15.4, Pittsburgh=9.7/9.7/9.7, Ségou=25.7/25.7/25.7, Tauranga=38.2/38.2/38.2, Xi'an=24.2/24.2/24.2, Zagreb=12.2/12.2/12.2}\n"
        .to_string();
        let result = process_data(data);
        assert_eq!(result, expected);
    }

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
                        let data = fs::read_to_string(file.clone()).unwrap();
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
