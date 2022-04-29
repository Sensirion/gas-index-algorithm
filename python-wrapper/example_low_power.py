"""
This example uses an SGP40 and SHTC3 connected to a Sensirion SEK-SensorBridge

Install the dependencies
pip install sensirion-shdlc-sensorbridge sensirion-i2c-sht sensirion-i2c-sgp4x
"""

import time
from sensirion_shdlc_driver import ShdlcSerialPort, ShdlcConnection
from sensirion_shdlc_sensorbridge import SensorBridgePort, \
    SensorBridgeShdlcDevice, SensorBridgeI2cProxy
from sensirion_i2c_driver import I2cConnection
from sensirion_i2c_sgp4x import Sgp40I2cDevice
from sensirion_i2c_sht.shtc3 import Shtc3I2cDevice
from sensirion_gas_index_algorithm.voc_algorithm import VocAlgorithm

# Sampling interval in seconds
# Set to 1 second for 20% power mode (default)
# Set to 10 seconds for 2% power mode
sampling_interval = 1

voc_algorithm = VocAlgorithm(sampling_interval)

HEATER_WARMUP_DURATION = 0.17  # in seconds
# Connect to the SensorBridge with default settings:
#  - baudrate:      460800
#  - slave address: 0
with ShdlcSerialPort(port='COM1', baudrate=460800) as port:
    bridge = SensorBridgeShdlcDevice(ShdlcConnection(port), slave_address=0)
    print("SensorBridge SN: {}".format(bridge.get_serial_number()))

    # Configure SensorBridge port 1 for SGP40
    bridge.set_i2c_frequency(SensorBridgePort.ONE, frequency=100e3)
    bridge.set_supply_voltage(SensorBridgePort.ONE, voltage=3.3)
    bridge.switch_supply_on(SensorBridgePort.ONE)

    # Create SGP40 and SHTC3 devices
    i2c_transceiver = SensorBridgeI2cProxy(bridge, port=SensorBridgePort.ONE)
    connection = I2cConnection(i2c_transceiver)

    sgp40 = Sgp40I2cDevice(connection)
    print("SGP40 Serial Number: {}".format(sgp40.get_serial_number()))

    shtc3 = Shtc3I2cDevice(connection)
    print("SHTC3 Product ID: {}".format(shtc3.read_product_id()))
    shtc3.enter_sleep()

    print("Sampling interval: {}s\n".format(voc_algorithm.get_sampling_interval()))

    # Get 120 measurement values
    for _ in range(120):
        # Sleep to have a loop duration matching the sampling interval
        # taking into account heater and measurement delays
        time.sleep(sampling_interval - HEATER_WARMUP_DURATION - 0.06 - 0.01)
        temperature, humidity = shtc3.measure()

        # Request a VOC measurement to start the heater
        sgp40.measure_raw(temperature=temperature.degrees_celsius,
                          relative_humidity=humidity.percent_rh)

        time.sleep(HEATER_WARMUP_DURATION)

        # Request the actual VOC measurement
        sraw_voc = sgp40.measure_raw(temperature=temperature.degrees_celsius,
                                     relative_humidity=humidity.percent_rh)

        # Turn off the heater to save power
        sgp40.turn_heater_off()

        # use default formatting for printing output:
        voc_index = voc_algorithm.process(sraw_voc.ticks)
        print("VOC Index: {}".format(voc_index))

    bridge.switch_supply_off(SensorBridgePort.ONE)
