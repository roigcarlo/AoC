# AOC-2023

This year in C++23 (or at least the features implemented in LLVM-17...)

## How to build

You may ask how to compile and run this code little Timmy, its easy. While working with computers everythings is inter-operable, compatible and standarized. Despite this amazing fact of our community, I've prepared a Dockerfile with everyting you need. Just build the container and run `./build.sh`

## How to run

Locate the directory in which the files may have been build. It should match the directoy in which you have told cmake to put them.

For example. If you cmake file contains this line (It should if you did not touch anything):

```cmake 
install(TARGETS d01 DESTINATION "${CMAKE_SOURCE_DIR}/bin")
```

In one of this directories will `d01` be installed?

1. `bin/d01`
2. `/usr/local/lib/x86-64/d01`
3. `/home/usr/santa/images/wedding-photos/a73kd02h3s4(1).jpeg`

Yes! the correct answer is `1`

Once you have located the file, just run it with the desired input. Inputs are divided into `sample[N]` and `data`. Sample files are samples and data file is the data.

### Example 1

```bash
./bin/d01 d01/sample
```

### Example 2

Will run the day one with the `sample` file

```bash
./bin/d01 d01/sample2
```

### Example 3

Will run the day one with the `sample2` file

```bash
./bin/d01 d01/data
```

### Example 4

Will run the day one with the `data` file

```bash
./bin/d01 /home/usr/santa/images/wedding-photos/a73kd02h3s4(1).jpeg
```

Will not work because the file `/home/usr/santa/images/wedding-photos/a73kd02h3s4(1).jpeg` does not exists and is not an input
