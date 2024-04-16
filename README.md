# rust_1brc
A rust implementation of the [1 billion row challenge](https://github.com/gunnarmorling/1brc/tree/main)

## Generating the data
To generate the data run the following command:
```bash
./create_measurements.sh 1000000000
```
This will generate 1,000,000,000 measurements in the `data` directory.
To generate a different number of measurements, replace `1000000000` with the desired number.

## Attempts on the challenge

The project has 1 module per approach to the challenge.
And each approach can be run by passing a string when running the program.
e.g. for the Naive approach:
```bash
cargo run --release ./data/test_measurements.txt naive
```
If not specified a default will be used.

### Approaches

- Naive: Reads the file into memory all at once as a string and iterates line by line, storing all the measurements in a vector.
    Calculates the average, min and max values all at the end by processing the vector once the file is fully processed and parsed.
- Naive_v2: Reads the file into memory all at once as a string and iterates line by line, difference with `Naive` is that it calculates the average, min and max values as it processes the file.
- buffer_lines: Reads the file line by line, but as a buffer of lines. Uses the same logic as `Naive_v2` to calculate the average, min and max values.

### Times

Machine: Windows 11, WSL2, Ubuntu 22.04.3 LTS, i5-13600k (stock clock), 32GB RAM

Testing is not the most scientific, but it is a rough idea of the performance of each approach relative to each other.
Multiples runs are listed to give an idea of the variance in the times running against the same file on my machine.

- Naive: ?? Have issues running this on my WSL instance as it is killed by the system. Presumably due to memory usage.
- Naive_v2: 3m 52.210s, 5m 08.251s, 4m 39.995s, 4m 17.518s
- buffer_lines: 2m 16.603s, 2m 17.833s

