# rust_1brc
A rust implementation of the [1 billion row challenge](https://github.com/gunnarmorling/1brc/tree/main)

## Generating the data
To generate the data run the following command:
```bash
./create_measurements.sh 1000000000
```
This will generate 1,000,000,000 measurements in the `data` directory.
To generate a different number of measurements, replace `1000000000` with the desired number.