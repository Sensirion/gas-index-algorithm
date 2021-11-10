sgp41_driver_base = raspberry-pi-i2c-sgp41
sht4x_driver_base = raspberry-pi-i2c-sht4x
driver_common_sources = ${sgp41_driver_base}/sensirion_common.c
driver_i2c_sources = ${sgp41_driver_base}/sensirion_i2c.h ${sgp41_driver_base}/sensirion_i2c.c
driver_i2c_implementation ?= ${sgp41_driver_base}/sensirion_i2c_hal.c
sgp41_sources = ${sgp41_driver_base}/sgp41_i2c.c
sht4x_sources = ${sht4x_driver_base}/sht4x_i2c.c

algorithm_src_folder = ../sensirion_gas_index_algorithm
algorithm_sources = ${algorithm_src_folder}/sensirion_gas_index_algorithm.c


CFLAGS = -Os -Wall -fstrict-aliasing -Wstrict-aliasing=1 -Wsign-conversion -fPIC -I..

ifdef CI
    CFLAGS += -Werror
endif

.PHONY: all clean

all: clean algorithm_example_usage

algorithm_example_usage:
	$(CC) $(CFLAGS) -o $@ ${driver_common_sources} ${driver_i2c_sources} ${driver_i2c_implementation} ${sgp41_sources} ${sht4x_sources} ${algorithm_sources} algorithm_example_usage.c

clean:
	$(RM) algorithm_example_usage
