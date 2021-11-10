# Sensirion Embedded Gas Index Algorithm

SGP4x VOC/NOx Engine Algorithm applies a gain-offset normalization algorithm to a SGP tick signal. The algorithm 
assumes a humidity compensated raw tick signal, applies state estimation, tick-to-GasIndex conversion and an 
adaptive lowpass filter.

The main goal of the VOC/NOX Engine algorithm is to calculate a VOC/NOx Index signal that enables robust detection of ambient
VOC/NOx changes, with minimal sensor-to-sensor variations. The Algorithm Engine calculates the VOC/NOx Index signal 
recursively from a single raw tick value Sout that is measured by the SGP sensor at each time step, as well as internal 
states that are updated at each time step. These internal states are most importantly the recursively estimated mean and 
variance of the Sout signal, as well as some additional internal states such as uptime and other counters.


# Quick Start

Steps to calculate a VOC/NOx gas index value:

1. Allocate a struct of type **GasIndexAlgorithmParams** and initialize the parameters to their default values.
   
   a. For **VOC Engine Algorithm** initialize with: 
   
   ```
   GasIndexAlgorithmParams params;
   GasIndexAlgorithm_init(&params, GasIndexAlgorithm_ALGORITHM_TYPE_VOC);
   ```
   
   b. For **NOx Engine Algorithm** initialze with    
   ```
   GasIndexAlgorithmParams params;
   GasIndexAlgorithm_init(&params, GasIndexAlgorithm_ALGORITHM_TYPE_NOX);
   ```
2. Read RH and T values, e.g. from a SHT4x sensor
3. **Read VOC or NOx raw value** (ticks) from SGP4x sensor. Provide RH and T values from step 2 (do consider the specified scaling factors for the different sensors) to the VOC read function to get temperature and humidity compensated raw index value.
4. **Process raw signal** to get index value
   
   ```
   int32_t voc_raw_value; // read from sensor in step 2-3
   int32_t voc_index_value; 
   GasIndexAlgorithm_process(&params, voc_raw_value, &voc_index_value)
   ```

## Rasperry Pi Example VOC and NOx Index

Steps to run the example on your Raspberry Pi:

1. Connect a SGP41 and SHT4x sensor over I2C to your Raspberry Pi
2. Download this package from [Sensirion Github Page](https://github.com/Sensirion/sensirion-gas-index-algorithm)
3. Unzip and extract the .zip, copy the raspberry-pi-algorithm-example folder to your Raspberry Pi
4. Download the [Raspberyy PI I2C SHT4x driver package](https://github.com/Sensirion/raspberry-pi-i2c-sht4x) and [Raspberyy PI I2C SGP41 driver package](https://github.com/Sensirion/raspberry-pi-i2c-sgp41) from Sensirion Github Page. Extract the ZIPs to subfolders of raspberry-pi-algorithm-example on your Raspberry Pi.
5. Compile the example
   1. Open a [terminal](https://www.raspberrypi.org/documentation/usage/terminal/?)
   2. Navigate to the example directory. E.g. `cd ~/raspberry-pi-algorithm-example`
   3. Run the `make` command to compile the driver
6. Run the example with `./algorithm_example_usage` in the same directory you used to compile the example.


# Contributing

**Contributions are welcome!**

We develop and test this algorithm using our company internal tools (version
control, continuous integration, code review etc.) and automatically
synchronize the master branch with GitHub. But this doesn't mean that we don't
respond to issues or don't accept pull requests on GitHub. In fact, you're very
welcome to open issues or create pull requests :)

This Sensirion library uses
[`clang-format`](https://releases.llvm.org/download.html) to standardize the
formatting of all our `.c` and `.h` files. Make sure your contributions are
formatted accordingly:

The `-i` flag will apply the format changes to the files listed.

```bash
clang-format -i */*.c */*.h
```

Note that differences from this formatting will result in a failed build until
they are fixed.

# License

See [LICENSE](LICENSE).

