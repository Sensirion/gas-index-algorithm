algorithm_sources = sensirion_gas_index_algorithm.h sensirion_gas_index_algorithm.c


CFLAGS = -Os -Wall -fstrict-aliasing -Wstrict-aliasing=1 -Wsign-conversion -fPIC -I.

ifdef CI
    CFLAGS += -Werror
endif

.PHONY: all clean

all: algorithm_example_usage

algorithm_example_usage: clean
	$(CC) $(CFLAGS) -o $@  ${algorithm_sources} algorithm_example_usage.c

clean:
	$(RM) algorithm_example_usage
