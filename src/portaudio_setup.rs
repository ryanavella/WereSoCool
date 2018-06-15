extern crate rand;
use input_output_setup::Oscillator;
use portaudio as pa;
use settings::Settings;
use sine::generate_sinewave;
use std;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

pub fn setup_portaudio_input(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
) -> Result<
    (
        pa::Stream<pa::NonBlocking, pa::Input<f32>>,
        std::sync::mpsc::Receiver<Vec<f32>>,
    ),
    pa::Error,
> {
    let (input_callback_tx, input_callback_rx) = channel();
    let input_settings = get_input_settings(&pa, &settings)?;

    let input_stream = pa.open_non_blocking_stream(input_settings, move |args| {
        input_callback_tx.send(args.buffer.to_vec()).unwrap();
        pa::Continue
    })?;

    Ok((input_stream, input_callback_rx))
}

fn get_input_settings(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
) -> Result<pa::stream::InputSettings<f32>, pa::Error> {
    let def_input = pa.default_input_device()?;
    let input_info = pa.device_info(def_input)?;
    println!("Default input device info: {:#?}", &input_info);

    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(
        def_input,
        settings.channels,
        settings.interleaved,
        latency,
    );

    let input_settings = pa::InputStreamSettings::new(
        input_params,
        settings.sample_rate as f64,
        settings.input_buffer_size as u32,
    );

    Ok(input_settings)
}

pub fn setup_portaudio_output(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
    frequency: Arc<std::sync::Mutex<isize>>,
) -> Result<pa::Stream<pa::NonBlocking, pa::Output<f32>>, pa::Error> {
    let settings = settings.clone();
    let output_settings = get_output_settings(&pa, &settings)?;
    let mut phase: f32 = 0.0;
    let output_stream = pa.open_non_blocking_stream(output_settings, move |args| {
        let mut idx = 0;

        let frequency = *frequency.lock().unwrap();
//        println!("{:?}", frequency);
//        let frequency = 440;
        if frequency < 2500 {
            let (waveform, new_phase) =
                generate_sinewave(frequency as f32, phase, 512.0 as usize, 44100.0);
                phase = new_phase;
//              println!("{:?}, {:?}", frequency, phase);

            for _ in 0..args.frames {
                args.buffer[idx] = waveform[idx];

                idx += 1;
            }
        } else {
            for _ in 0..args.frames {
                args.buffer[idx] = 0.0;

                idx += 1;
            }
        }

        pa::Continue
    })?;

    Ok(output_stream)
}

pub fn get_output_settings(
    ref pa: &pa::PortAudio,
    ref settings: &Settings,
) -> Result<pa::stream::OutputSettings<f32>, pa::Error> {
    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    println!("Default output device info: {:#?}", &output_info);

    let latency = output_info.default_low_output_latency;
    let output_params =
        pa::StreamParameters::new(def_output, settings.channels, settings.interleaved, latency);

    let output_settings = pa::OutputStreamSettings::new(
        output_params,
        settings.sample_rate as f64,
        settings.output_buffer_size as u32,
    );

    Ok(output_settings)
}
