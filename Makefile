algorithm_src_folder = sensirion_gas_index_algorithm
algorithm_sources = ${algorithm_src_folder}/sensirion_gas_index_algorithm.c


CFLAGS = -Os -Wall -fstrict-aliasing -Wstrict-aliasing=1 -Wsign-conversion -fPIC -I.

ifdef CI
    CFLAGS += -Werror
endif

.PHONY: all clean

all: clean algorithm_example_usage

algorithm_example_usage:
	$(CC) $(CFLAGS) -o $@  ${algorithm_sources} algorithm_example_usage.c

clean:
	$(RM) algorithm_example_usage
