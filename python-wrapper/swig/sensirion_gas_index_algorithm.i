%module sensirion_gas_index_algorithm_wrap


%include cpointer.i
%include carrays.i
%include cstring.i
%include stdint.i
%include "typemaps.i"

%apply int32_t *OUTPUT { int32_t *state0, int32_t *state1 };
%apply int32_t *OUTPUT { int32_t* index_offset,
                         int32_t* learning_time_offset_hours,
                         int32_t* learning_time_gain_hours,
                         int32_t* gating_max_duration_minutes,
                         int32_t* std_initial,
                         int32_t* gain_factor } ;
%apply float *OUTPUT {float* samp_int};
%apply int32_t *OUTPUT {int32_t* gas_index};

%{
#include "sensirion_gas_index_algorithm.h"
%}

%include "sensirion_gas_index_algorithm.h"
%extend GasIndexAlgorithmParams {
	GasIndexAlgorithmParams(int32_t algorithm_type) {
		GasIndexAlgorithmParams *params;
		params = (GasIndexAlgorithmParams *) malloc(sizeof(GasIndexAlgorithmParams));
		GasIndexAlgorithm_init(params, algorithm_type);
		return params;
	}
	~GasIndexAlgorithmParams() {
		free($self);
	}

	void init(int32_t algorithm_type) {
		GasIndexAlgorithm_init($self, algorithm_type);
	}

    void get_states(int32_t* state0, int32_t* state1) {
        GasIndexAlgorithm_get_states($self, state0, state1);
    }

    void set_states(int32_t state0, int32_t state1) {
        GasIndexAlgorithm_set_states($self, state0, state1);
    }

    void get_tuning_parameters(int32_t* index_offset,
                               int32_t* learning_time_offset_hours,
                               int32_t* learning_time_gain_hours,
                               int32_t* gating_max_duration_minutes,
                               int32_t* std_initial,
                               int32_t* gain_factor) {
        GasIndexAlgorithm_get_tuning_parameters($self, index_offset,
            learning_time_offset_hours, learning_time_gain_hours,
            gating_max_duration_minutes, std_initial, gain_factor);
   }

    void set_tuning_parameters(int32_t index_offset,
                               int32_t learning_time_offset_hours,
                               int32_t learning_time_gain_hours,
                               int32_t gating_max_duration_minutes,
                               int32_t std_initial,
                               int32_t gain_factor) {
        GasIndexAlgorithm_set_tuning_parameters($self, index_offset,
            learning_time_offset_hours, learning_time_gain_hours,
            gating_max_duration_minutes, std_initial, gain_factor);
    }

    void get_sampling_interval(float* samp_int) {
        GasIndexAlgorithm_get_sampling_interval($self, samp_int);
    }
    
    void set_sampling_interval(float samp_int) {
        GasIndexAlgorithm_set_sampling_interval($self, samp_int);
    }

	int32_t process(int32_t sraw) {
		int32_t gas_index;
		GasIndexAlgorithm_process($self, sraw, &gas_index);
		return gas_index;
	}
	const char* get_version() {
		return LIBRARY_VERSION_NAME;
	}
};
