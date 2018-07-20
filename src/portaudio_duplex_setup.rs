//extern crate rand;
//use analyze::{Analyze, DetectionResult};
//use new_oscillator::{NewOscillator, StereoWaveform};
//use portaudio as pa;
//
//use ratios::mono_ratios;
//use ring_buffer::RingBuffer;
//use settings::{get_default_app_settings, Settings};
//
//pub fn setup_portaudio_duplex(
//    ref pa: &pa::PortAudio,
//) -> Result<pa::Stream<pa::NonBlocking, pa::Duplex<f32, f32>>, pa::Error> {
//    let settings = get_default_app_settings();
//
//    //    let (l_ratios, r_ratios) = simple_ratios();
//    let mut osc = Oscillator::new(10, mono_ratios(), get_default_app_settings());
//    let duplex_stream_settings = get_duplex_settings(&pa, &settings)?;
//
//    let mut input_buffer: RingBuffer<f32> = RingBuffer::<f32>::new(settings.yin_buffer_size);
//    let mut count = 0;
//    let duplex_stream = pa.open_non_blocking_stream(
//        duplex_stream_settings,
//        move |pa::DuplexStreamCallbackArgs {
//                  in_buffer,
//                  mut out_buffer,
//                  ..
//              }| {
//            if count < 20 {
//                count += 1;
//                if count == 20 {
//                    println!("{}", "* * * * * ready * * * * *");
//                }
//                pa::Continue
//            } else {
//                input_buffer.push_vec(in_buffer.to_vec());
//                let result: DetectionResult = input_buffer
//                    .to_vec()
//                    .analyze(settings.sample_rate, settings.probability_threshold);
//
//                osc.update(result.frequency, result.gain, result.probability);
//                let stereo_waveform = osc.generate();
//
//                write_duplex_buffer(&mut out_buffer, stereo_waveform);
//
//                pa::Continue
//            }
//        },
//    )?;
//
//    Ok(duplex_stream)
//}
//
//fn write_duplex_buffer(out_buffer: &mut [f32], stereo_waveform: StereoWaveform) {
//    let mut l_idx = 0;
//    let mut r_idx = 0;
//    for n in 0..out_buffer.len() {
//        if n % 2 == 0 {
//            out_buffer[n] = stereo_waveform.l_waveform[l_idx];
//            l_idx += 1
//        } else {
//            out_buffer[n] = stereo_waveform.r_waveform[r_idx];
//            r_idx += 1
//        }
//    }
//}
//
//fn get_duplex_settings(
//    ref pa: &pa::PortAudio,
//    ref settings: &Settings,
//) -> Result<pa::stream::DuplexSettings<f32, f32>, pa::Error> {
//    let def_input = pa.default_input_device()?;
//    let input_info = pa.device_info(def_input)?;
//    //    println!("Default input device info: {:#?}", &input_info);
//
//    let latency = input_info.default_low_input_latency;
//    let input_params = pa::StreamParameters::<f32>::new(
//        def_input,
//        settings.channels,
//        settings.interleaved,
//        latency,
//    );
//
//    let def_output = pa.default_output_device()?;
//    let output_info = pa.device_info(def_output)?;
//    //    println!("Default output device info: {:#?}", &output_info);
//
//    let latency = output_info.default_low_output_latency;
//    let output_params =
//        pa::StreamParameters::new(def_output, settings.channels, settings.interleaved, latency);
//
//    let duplex_settings = pa::DuplexStreamSettings::new(
//        input_params,
//        output_params,
//        settings.sample_rate as f64,
//        settings.buffer_size as u32,
//    );
//
//    Ok(duplex_settings)
//}
