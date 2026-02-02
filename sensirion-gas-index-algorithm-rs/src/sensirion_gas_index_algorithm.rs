#[cfg(not(feature = "std"))]
#[allow(unused)]
trait ExtF32 {
    fn exp(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, y: i32) -> Self;
}

#[cfg(not(feature = "std"))]
impl ExtF32 for f32 {
    fn exp(self) -> Self {
        libm::expf(self)
    }
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    fn powi(self, y: i32) -> Self {
        libm::powf(self, y as f32)
    }
}

const DEFAULT_SAMPLING_INTERVAL: f32 = 1.0;
const INITIAL_BLACKOUT: f32 = 45.0;
const INDEX_GAIN: f32 = 230.0;
const SRAW_STD_INITIAL: f32 = 50.0;
const SRAW_STD_BONUS_VOC: f32 = 220.0;
const SRAW_STD_NOX: f32 = 2000.0;
const TAU_MEAN_HOURS: f32 = 12.0;
const TAU_VARIANCE_HOURS: f32 = 12.0;
const TAU_INITIAL_MEAN_VOC: f32 = 20.0;
const TAU_INITIAL_MEAN_NOX: f32 = 1200.0;
const INIT_DURATION_MEAN_VOC: f32 = 3600.0 * 0.75;
const INIT_DURATION_MEAN_NOX: f32 = 3600.0 * 4.75;
const INIT_TRANSITION_MEAN: f32 = 0.01;
const TAU_INITIAL_VARIANCE: f32 = 2500.0;
const INIT_DURATION_VARIANCE_VOC: f32 = 3600.0 * 1.45;
const INIT_DURATION_VARIANCE_NOX: f32 = 3600.0 * 5.70;
const INIT_TRANSITION_VARIANCE: f32 = 0.01;
const GATING_THRESHOLD_VOC: f32 = 340.0;
const GATING_THRESHOLD_NOX: f32 = 30.0;
const GATING_THRESHOLD_INITIAL: f32 = 510.0;
const GATING_THRESHOLD_TRANSITION: f32 = 0.09;
const GATING_VOC_MAX_DURATION_MINUTES: f32 = 60.0 * 3.0;
const GATING_NOX_MAX_DURATION_MINUTES: f32 = 60.0 * 12.0;
const GATING_MAX_RATIO: f32 = 0.3;
const SIGMOID_L: f32 = 500.0;
const SIGMOID_K_VOC: f32 = -0.0065;
const SIGMOID_X0_VOC: f32 = 213.0;
const SIGMOID_K_NOX: f32 = -0.0101;
const SIGMOID_X0_NOX: f32 = 614.0;
const VOC_INDEX_OFFSET_DEFAULT: f32 = 100.0;
const NOX_INDEX_OFFSET_DEFAULT: f32 = 1.0;
const LP_TAU_FAST: f32 = 20.0;
const LP_TAU_SLOW: f32 = 500.0;
const LP_ALPHA: f32 = -0.2;
const VOC_SRAW_MINIMUM: f32 = 20000.0;
const NOX_SRAW_MINIMUM: f32 = 10000.0;
const PERSISTENCE_UPTIME_GAMMA: f32 = 3.0 * 3600.0;
const MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING: f32 = 64.0;
const MEAN_VARIANCE_ESTIMATOR_ADDITIONAL_GAMMA_MEAN_SCALING: f32 = 8.0;
const MEAN_VARIANCE_ESTIMATOR_FIX16_MAX: f32 = 32767.0;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum AlgorithmType {
    #[default]
    Voc,
    Nox,
}

#[derive(Default, Clone, Copy)]
pub struct TuningParameters {
    pub index_offset: f32,
    pub learning_time_offset_hours: f32,
    pub learning_time_gain_hours: f32,
    pub gating_max_duration_minutes: f32,
    pub std_initial: f32,
    pub gain_factor: f32,
}

impl TuningParameters {
    pub fn new(
        index_offset: f32,
        learning_time_offset_hours: f32,
        learning_time_gain_hours: f32,
        gating_max_duration_minutes: f32,
        std_initial: f32,
        gain_factor: f32,
    ) -> Self {
        Self {
            index_offset,
            learning_time_offset_hours,
            learning_time_gain_hours,
            gating_max_duration_minutes,
            std_initial,
            gain_factor,
        }
    }

    fn default_for(algorithm_type: AlgorithmType) -> Self {
        let mut res = Self::default();
        match algorithm_type {
            AlgorithmType::Nox => {
                res.index_offset = NOX_INDEX_OFFSET_DEFAULT;
                res.gating_max_duration_minutes = GATING_NOX_MAX_DURATION_MINUTES;
            }
            AlgorithmType::Voc => {
                res.index_offset = VOC_INDEX_OFFSET_DEFAULT;
                res.gating_max_duration_minutes = GATING_VOC_MAX_DURATION_MINUTES;
            }
        }
        res.gain_factor = INDEX_GAIN;
        res.learning_time_offset_hours = TAU_MEAN_HOURS;
        res.learning_time_gain_hours = TAU_VARIANCE_HOURS;
        res.std_initial = SRAW_STD_INITIAL;

        res
    }
}

#[derive(Default)]
struct MoxModel {
    sraw_std: f32,
    sraw_mean: f32,
}

impl MoxModel {
    fn new(sraw_std: f32, sraw_mean: f32) -> Self {
        Self {
            sraw_std,
            sraw_mean,
        }
    }

    fn process(
        &self,
        algorithm_type: AlgorithmType,
        tuning_parameters: &TuningParameters,
        sraw: f32,
    ) -> f32 {
        match algorithm_type {
            AlgorithmType::Nox => {
                (sraw - self.sraw_mean) / SRAW_STD_NOX * tuning_parameters.gain_factor
            }
            AlgorithmType::Voc => {
                ((sraw - self.sraw_mean) / (-(self.sraw_std + SRAW_STD_BONUS_VOC)))
                    * tuning_parameters.gain_factor
            }
        }
    }
}

#[derive(Default)]
pub struct GasIndexAlgorithm {
    algorithm_type: AlgorithmType,
    sampling_interval: f32,
    tuning_parameters: TuningParameters,
    sraw_minimum: f32,
    init_duration_mean: f32,
    init_duration_variance: f32,
    gating_threshold: f32,
    uptime: f32,
    sraw: f32,
    gas_index: f32,
    mean_variance_estimator: MeanVarianceEstimator,
    mox_model: MoxModel,
    sigmoid_scaled: SigmoidScaled,
    adaptive_lowpass: AdaptiveLpf,
}

impl GasIndexAlgorithm {
    pub fn with_sampling_interval(algorithm_type: AlgorithmType, sampling_interval: f32) -> Self {
        let mut res = Self {
            algorithm_type,
            sampling_interval,
            ..Default::default()
        };

        match algorithm_type {
            AlgorithmType::Nox => {
                res.sraw_minimum = NOX_SRAW_MINIMUM;
                res.init_duration_mean = INIT_DURATION_MEAN_NOX;
                res.init_duration_variance = INIT_DURATION_VARIANCE_NOX;
                res.gating_threshold = GATING_THRESHOLD_NOX;
            }
            AlgorithmType::Voc => {
                res.sraw_minimum = VOC_SRAW_MINIMUM;
                res.init_duration_mean = INIT_DURATION_MEAN_VOC;
                res.init_duration_variance = INIT_DURATION_VARIANCE_VOC;
                res.gating_threshold = GATING_THRESHOLD_VOC;
            }
        }

        res.tuning_parameters = TuningParameters::default_for(algorithm_type);
        res.reset();

        res
    }

    pub fn new(algorithm_type: AlgorithmType) -> Self {
        Self::with_sampling_interval(algorithm_type, DEFAULT_SAMPLING_INTERVAL)
    }

    pub fn reset(&mut self) {
        self.uptime = 0.0;
        self.sraw = 0.0;
        self.gas_index = 0.0;
        self.init_instances();
    }

    fn init_instances(&mut self) {
        self.mean_variance_estimator = MeanVarianceEstimator::new(
            &self.tuning_parameters,
            self.sampling_interval,
            self.algorithm_type,
        );
        self.mox_model = MoxModel::new(
            self.mean_variance_estimator.std(),
            self.mean_variance_estimator.mean(),
        );
        self.sigmoid_scaled = SigmoidScaled::new(self.algorithm_type);
        self.adaptive_lowpass = AdaptiveLpf::new(self.sampling_interval);
    }

    pub fn sampling_interval(&self) -> f32 {
        self.sampling_interval
    }

    pub fn states(&self, mean: &mut f32, std: &mut f32) {
        *mean = self.mean_variance_estimator.mean();
        *std = self.mean_variance_estimator.std();
    }

    pub fn set_states(&mut self, mean: f32, std: f32) {
        self.mean_variance_estimator
            .set_states(mean, std, PERSISTENCE_UPTIME_GAMMA);
        self.mox_model = MoxModel::new(
            self.mean_variance_estimator.std(),
            self.mean_variance_estimator.mean(),
        );
        self.sraw = mean;
    }

    pub fn set_tuning_parameters(&mut self, tuning_parameters: TuningParameters) {
        self.tuning_parameters = tuning_parameters;
        self.init_instances();
    }

    pub fn tuning_parameters(&self) -> TuningParameters {
        self.tuning_parameters
    }

    /// Calculate the gas index value from the raw sensor value.
    /// sraw - raw value from the SGP4x sensor
    /// Returns Some(calculated gas index value), or None during initial blackout period.
    pub fn process(&mut self, sraw: u16) -> Option<i32> {
        if self.uptime <= INITIAL_BLACKOUT {
            self.uptime += self.sampling_interval;
            None
        } else {
            self.sraw = (sraw as f32 - self.sraw_minimum).clamp(1.0, i16::MAX as f32);

            if (self.algorithm_type == AlgorithmType::Voc)
                || self.mean_variance_estimator.is_initialized()
            {
                self.gas_index =
                    self.mox_model
                        .process(self.algorithm_type, &self.tuning_parameters, self.sraw);
                self.gas_index = self
                    .sigmoid_scaled
                    .process(self.gas_index, self.tuning_parameters.index_offset);
            } else {
                self.gas_index = self.tuning_parameters.index_offset;
            }
            self.gas_index = self.adaptive_lowpass.process(self.gas_index).max(0.5);
            if self.sraw > 0.0 {
                self.mean_variance_estimator.process(
                    self.sraw,
                    self.sampling_interval,
                    self.gating_threshold,
                    self.init_duration_variance,
                    &self.tuning_parameters,
                    self.init_duration_mean,
                    self.gas_index,
                );
                self.mox_model = MoxModel::new(
                    self.mean_variance_estimator.std(),
                    self.mean_variance_estimator.mean(),
                );
            }
            Some((self.gas_index + 0.5) as i32)
        }
    }
}

#[derive(Default)]
struct MeanVarianceEstimator {
    initialized: bool,
    mean: f32,
    sraw_offset: f32,
    std: f32,
    sigmoid: Sigmoid,
    uptime_gamma: f32,
    gamma_mean: f32,
    gamma_variance: f32,
    gamma_initial_mean: f32,
    gamma_initial_variance: f32,
    sigmoid_gamma_mean: f32,
    sigmoid_gamma_variance: f32,
    uptime_gating: f32,
    gating_duration_minutes: f32,
}

impl MeanVarianceEstimator {
    fn new(
        tuning_parameters: &TuningParameters,
        sampling_interval: f32,
        algorithm_type: AlgorithmType,
    ) -> Self {
        let mut res = Self {
            std: tuning_parameters.std_initial,
            gamma_mean: ((MEAN_VARIANCE_ESTIMATOR_ADDITIONAL_GAMMA_MEAN_SCALING
                * MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING)
                * (sampling_interval / 3600.0))
                / (tuning_parameters.learning_time_offset_hours + (sampling_interval / 3600.0)),
            gamma_variance: (MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING * (sampling_interval / 3600.0))
                / (tuning_parameters.learning_time_gain_hours + (sampling_interval / 3600.0)),
            ..Default::default()
        };

        match algorithm_type {
            AlgorithmType::Nox => {
                res.gamma_initial_mean = ((MEAN_VARIANCE_ESTIMATOR_ADDITIONAL_GAMMA_MEAN_SCALING
                    * MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING)
                    * sampling_interval)
                    / (TAU_INITIAL_MEAN_NOX + sampling_interval);
            }
            AlgorithmType::Voc => {
                res.gamma_initial_mean = ((MEAN_VARIANCE_ESTIMATOR_ADDITIONAL_GAMMA_MEAN_SCALING
                    * MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING)
                    * sampling_interval)
                    / (TAU_INITIAL_MEAN_VOC + sampling_interval);
            }
        }
        res.gamma_initial_variance = (MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING * sampling_interval)
            / (TAU_INITIAL_VARIANCE + sampling_interval);

        res
    }
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    fn std(&self) -> f32 {
        self.std
    }
    fn mean(&self) -> f32 {
        self.mean + self.sraw_offset
    }
    fn set_states(&mut self, mean: f32, std: f32, uptime_gamma: f32) {
        self.mean = mean;
        self.std = std;
        self.uptime_gamma = uptime_gamma;
        self.initialized = true;
    }

    fn calculate_gamma(
        &mut self,
        sampling_interval: f32,
        gating_threshold: f32,
        init_duration_variance: f32,
        tuning_parameters: &TuningParameters,
        init_duration_mean: f32,
        gas_index: f32,
    ) {
        let uptime_limit = MEAN_VARIANCE_ESTIMATOR_FIX16_MAX - sampling_interval;
        if self.uptime_gamma < uptime_limit {
            self.uptime_gamma += sampling_interval;
        }
        if self.uptime_gating < uptime_limit {
            self.uptime_gating += sampling_interval;
        }
        self.sigmoid = Sigmoid::new(init_duration_mean, INIT_TRANSITION_MEAN);
        let sigmoid_gamma_mean = self.sigmoid.process(self.uptime_gamma);

        let gating_threshold_mean = gating_threshold
            + ((GATING_THRESHOLD_INITIAL - gating_threshold)
                * self.sigmoid.process(self.uptime_gating));

        self.sigmoid = Sigmoid::new(gating_threshold_mean, GATING_THRESHOLD_TRANSITION);
        let sigmoid_gating_mean = self.sigmoid.process(gas_index);
        self.sigmoid_gamma_mean = sigmoid_gating_mean
            * (self.gamma_mean
                + ((self.gamma_initial_mean - self.gamma_mean) * sigmoid_gamma_mean));

        self.sigmoid = Sigmoid::new(init_duration_variance, INIT_TRANSITION_VARIANCE);
        let sigmoid_gamma_variance = self.sigmoid.process(self.uptime_gamma);

        let gamma_variance = self.gamma_variance
            + ((self.gamma_initial_variance - self.gamma_variance)
                * (sigmoid_gamma_variance - sigmoid_gamma_mean));

        let gating_threshold_variance = gating_threshold
            + ((GATING_THRESHOLD_INITIAL - gating_threshold)
                * self.sigmoid.process(self.uptime_gating));

        self.sigmoid = Sigmoid::new(gating_threshold_variance, GATING_THRESHOLD_TRANSITION);
        let sigmoid_gating_variance = self.sigmoid.process(gas_index);
        self.sigmoid_gamma_variance = sigmoid_gating_variance * gamma_variance;
        self.gating_duration_minutes += sampling_interval / 60.0
            * (((1.0 - sigmoid_gating_mean) * (1.0 + GATING_MAX_RATIO)) - GATING_MAX_RATIO)
                .max(0.0);
        if self.gating_duration_minutes < 0.0 {
            self.gating_duration_minutes = 0.0;
        }
        if self.gating_duration_minutes > tuning_parameters.gating_max_duration_minutes {
            self.uptime_gating = 0.0;
        }
    }

    fn process(
        &mut self,
        mut sraw: f32,
        sampling_interval: f32,
        gating_threshold: f32,
        init_duration_variance: f32,
        tuning_parameters: &TuningParameters,
        init_duration_mean: f32,
        gas_index: f32,
    ) {
        if !self.initialized {
            self.initialized = true;
            self.sraw_offset = sraw;
            self.mean = 0.0;
        } else {
            if self.mean >= 100.0 || self.mean <= -100.0 {
                self.sraw_offset += self.mean;
                self.mean = 0.0;
            }
            sraw -= self.sraw_offset;
            self.calculate_gamma(
                sampling_interval,
                gating_threshold,
                init_duration_variance,
                tuning_parameters,
                init_duration_mean,
                gas_index,
            );
            let delta_sgp = (sraw - self.mean) / MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING;
            let c = self.std + delta_sgp.abs();
            let additional_scaling = if c <= 1440.0 {
                1.0
            } else {
                (c / 1440.0).powi(2)
            };
            self.std = (additional_scaling
                * (MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING - self.sigmoid_gamma_variance))
                .sqrt()
                * (self.std
                    * (self.std / (MEAN_VARIANCE_ESTIMATOR_GAMMA_SCALING * additional_scaling))
                    + (((self.sigmoid_gamma_variance * delta_sgp) / additional_scaling)
                        * delta_sgp))
                    .sqrt();
            self.mean += (self.sigmoid_gamma_mean * delta_sgp)
                / MEAN_VARIANCE_ESTIMATOR_ADDITIONAL_GAMMA_MEAN_SCALING;
        }
    }
}

#[derive(Default)]
struct Sigmoid {
    k: f32,
    x0: f32,
}

impl Sigmoid {
    fn new(x0: f32, k: f32) -> Self {
        Self { k, x0 }
    }

    fn process(&self, sample: f32) -> f32 {
        let x = self.k * (sample - self.x0);
        if x < -50.0 {
            1.0
        } else if x > 50.0 {
            0.0
        } else {
            1.0 / (1.0 + x.exp())
        }
    }
}

#[derive(Default)]
struct SigmoidScaled {
    k: f32,
    x0: f32,
    offset_default: f32,
}

impl SigmoidScaled {
    fn new(algorithm_type: AlgorithmType) -> Self {
        match algorithm_type {
            AlgorithmType::Nox => Self {
                x0: SIGMOID_X0_NOX,
                k: SIGMOID_K_NOX,
                offset_default: NOX_INDEX_OFFSET_DEFAULT,
            },
            AlgorithmType::Voc => Self {
                x0: SIGMOID_X0_VOC,
                k: SIGMOID_K_VOC,
                offset_default: VOC_INDEX_OFFSET_DEFAULT,
            },
        }
    }

    fn process(&mut self, sample: f32, index_offset: f32) -> f32 {
        let x = self.k * (sample - self.x0);
        if x < -50.0 {
            SIGMOID_L
        } else if x > 50.0 {
            0.0
        } else if sample >= 0.0 {
            let shift = if self.offset_default == 1.0 {
                500.0 / 499.0 * (1.0 - index_offset)
            } else {
                (SIGMOID_L - (5.0 * index_offset)) / 4.0
            };
            ((SIGMOID_L + shift) / (1.0 + x.exp())) - shift
        } else {
            index_offset / self.offset_default * (SIGMOID_L / (1.0 + x.exp()))
        }
    }
}

#[derive(Default)]
struct AdaptiveLpf {
    initialized: bool,
    sampling_interval: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    x3: f32,
}

impl AdaptiveLpf {
    fn new(sampling_interval: f32) -> Self {
        Self {
            initialized: false,
            sampling_interval,
            a1: sampling_interval / (LP_TAU_FAST + sampling_interval),
            a2: sampling_interval / (LP_TAU_SLOW + sampling_interval),
            x1: Default::default(),
            x2: Default::default(),
            x3: Default::default(),
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        if !self.initialized {
            self.x1 = sample;
            self.x2 = sample;
            self.x3 = sample;
            self.initialized = true;
        }
        self.x1 = (1.0 - self.a1) * self.x1 + (self.a1 * sample);
        self.x2 = ((1.0 - self.a2) * self.x2) + (self.a2 * sample);

        let abs_delta = (self.x1 - self.x2).abs();

        let f1 = (LP_ALPHA * abs_delta).exp();
        let tau_a = (LP_TAU_SLOW - LP_TAU_FAST) * f1 + LP_TAU_FAST;
        let a3 = self.sampling_interval / (self.sampling_interval + tau_a);
        self.x3 = ((1.0 - a3) * self.x3) + (a3 * sample);

        self.x3
    }
}
