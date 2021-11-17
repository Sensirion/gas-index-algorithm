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
   
3. **Read VOC or NOx raw value** (ticks) from SGP4x sensor. 
   Provide RH and T values as ticks from step 2 to the VOC read function to get temperature and humidity compensated raw index value.
   NOTE: do consider that e.g. the RH ticks for SHT4x are specified differently than the RH ticks requested by SGP41, checkout the sensor datasheets for more details.

4. **Process raw signal** to get index value
   
   ```
   int32_t voc_raw_value; // read from sensor in step 2-3
   int32_t voc_index_value; 
   GasIndexAlgorithm_process(&params, voc_raw_value, &voc_index_value)
   ```

## Raspberry Pi Example VOC and NOx Index

The example measures VOC and NOx ticks with a SGP41 sensor using a SHT4x to compensate temperature and humidity.
The raw VOC and NOx measurement signals are then processed with the gas index algorithm to get VOC Index and NOx Index values.

For more details about the sensors and breakout boards check out http://sensirion.com/my-sgp-ek/.

Steps to run the example on your Raspberry Pi:

1. Connect a SGP41 and SHT4x sensor over I2C to your Raspberry Pi
2. Download this package from [Sensirion Github Page](https://github.com/Sensirion/gas-index-algorithm).
   Extract the ZIP file and navigate to the subfolder `examples/raspberry-pi`
3. Get additional source files:
   1. Run `make download`
   2. If the download with make does not work, you can manually get the needed source files:
      - Download [Raspberyy PI I2C SHT4x driver package](https://github.com/Sensirion/raspberry-pi-i2c-sht4x). 
        Extract ZIP and add all `*.c` and `*.h` files to `examples/raspberry-pi/`
      - Download [Raspberyy PI I2C SGP41 driver package](https://github.com/Sensirion/raspberry-pi-i2c-sgp41).
        Extract ZIP and copy `sgp41.c` and `sgp41.h` to `examples/raspberry-pi/`
      - Copy content of folder `sensirion_gas_index_algorithm` to `examples/raspberry-pi`
4. Copy the `examples/raspberry-pi` folder to your Raspberry Pi
5. Compile the example on your Raspberry Pi
   1. Open a [terminal](https://www.raspberrypi.org/documentation/usage/terminal/?)
   2. Navigate to the example directory. E.g. `cd ~/examples/raspberry-pi`
   3. Run `make all` to compile the driver
6. Run the example with `./algorithm_example_usage`


# Python Example

Find the Python usage and documentation in the [Python README](python-wrapper/README.rst).


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

