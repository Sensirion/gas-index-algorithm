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
   Pass GasIndexAlgorithm_ALGORITHM_TYPE_NOX in case you want to calculate NOx Index.
   
   ```
   GasIndexAlgorithmParams params;
   GasIndexAlgorithm_init(&params, GasIndexAlgorithm_ALGORITHM_TYPE_VOC);
   ```
2. Read RH and T values, e.g. from a SHT4x sensor
3. **Read VOC or NOx raw value** (ticks) from SGP4x sensor. Provide RH and T values from step 1 (do consider the specified scaling factors for the different sensors) to the VOC read function to get temperature and humidity compensated raw index value.
4. **Process raw signal** to get index value
   
   ```
   int32_t voc_raw_value; // read from sensor in step 2-3
   int32_t voc_index_value; 
   GasIndexAlgorithm_process(&params, voc_raw_value, &voc_index_value)
   ```