use ratios::{R, StereoRatios};
use ring_buffer::RingBuffer;
use settings::Settings;
use sine::Generator;

pub struct Oscillator {
    pub f_buffer: RingBuffer<f32>,
    pub stereo_ratios: StereoRatios,
//    pub l_ratios: Vec<R>,
//    pub r_ratios: Vec<R>,
    pub l_phases: Vec<f32>,
    pub r_phases: Vec<f32>,
    pub generator: Generator,
    pub gain: Gain,
    pub settings: Settings,
}


#[derive(Debug)]
pub struct Gain {
    pub past: f32,
    pub current: f32,
}

impl Gain {
    pub fn new(past: f32, current: f32) -> Gain {
        Gain { past, current }
    }

    pub fn update(&mut self, mut new_gain: f32) -> () {
        self.past = self.current;
        if (self.current - new_gain).abs() > 0.5 {
            new_gain = new_gain * 0.51;
        }
        self.current = new_gain;
    }
}

impl Oscillator {
    pub fn new(
        f_buffer_size: usize,
        stereo_ratios: StereoRatios,
//        l_ratios: Vec<R>,
//        r_ratios: Vec<R>,
        settings: Settings,
    ) -> Oscillator {
        println!("{}", "Left Generated Ratios");
        for r in stereo_ratios.l_ratios.iter() {
            println!("   - {} offset: {}", r.ratio, r.offset);
        }

        println!("{}", "Right Generated Ratios");
        for r in stereo_ratios.r_ratios.iter() {
            println!("   - {} offset: {}", r.ratio, r.offset);
        }

        Oscillator {
            f_buffer: RingBuffer::<f32>::new_full(f_buffer_size as usize),
//            l_ratios,
//            r_ratios,
            l_phases: vec![0.0; stereo_ratios.l_ratios.len()],
            r_phases: vec![0.0; stereo_ratios.r_ratios.len()],
            stereo_ratios,
            generator: Generator::new(),
            gain: Gain::new(0.0, 0.0),
            settings,
        }
    }

    pub fn update(&mut self, frequency: f32, gain: f32, _probability: f32) {
        let new_freq = if frequency < self.settings.max_freq && frequency > self.settings.min_freq {
            frequency
        } else {
            0.0
        };

        let mut new_gain = if new_freq != 0.0 { gain } else { 0.0 };

        if new_gain < self.settings.gain_threshold_min {
            new_gain = 0.0
        };

        self.f_buffer.push(new_freq);
        self.gain.update(new_gain);
        //                self.f_buffer.push(220.0);
        //                self.gain.update(1.0);
    }

    fn f_buffer_to_ratios(&mut self) {
        let base_frequency = self.f_buffer.current();

        for freq in self.f_buffer.to_vec() {
            let mut value = freq / base_frequency;
            if value.is_infinite() || value.is_nan() || value == 0.0 {
                value = 1.0;
            }
            //                println!("{}", value);
        }
    }

    pub fn generate(&mut self) -> (Vec<f32>, Vec<f32>) {
        //           println!("{:?}", self.f_buffer.to_vec());
        self.f_buffer_to_ratios();
        let current_frequency = self.f_buffer.current();
        let previous_frequency = self.f_buffer.previous();

        let mut frequency = current_frequency;

        if current_frequency == 0.0 && previous_frequency != 0.0 {
            frequency = previous_frequency
        }

        let (l_waveform, l_new_phases, _loudness) = (self.generator.generate)(
            frequency,
            &self.gain,
            &self.stereo_ratios.l_ratios,
            &self.l_phases,
            &self.settings,
        );

        let (r_waveform, r_new_phases, loudness) = (self.generator.generate)(
            frequency,
            &self.gain,
            &self.stereo_ratios.r_ratios,
            &self.r_phases,
            &self.settings,
        );

        self.gain.current *= loudness;

        self.l_phases = l_new_phases;
        self.r_phases = r_new_phases;

        ( l_waveform, r_waveform )
    }
}

fn silence(buffer_size: usize) -> (Vec<f32>, Vec<f32>) {
    (vec![0.0; buffer_size], vec![0.0; buffer_size])
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_ratio() {
        let r: R = R::atio(3, 2, 0.0, 1.0);
        let result = r.ratio;
        let expected = "3/2";
        assert_eq!(result, expected);
        let result = r.decimal;
        let expected = 1.5;
        assert_eq!(result, expected);
    }
}
