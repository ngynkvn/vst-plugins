#[macro_use]
extern crate vst;

use core::time;

use rand::random;
use vst::{
    api::{TimeInfo, TimeInfoFlags},
    buffer::AudioBuffer,
    channels::ChannelInfo,
    host::Host,
    plugin::{Category, HostCallback, Info, Plugin},
};

#[derive(Default)]
struct Whisper {
    host: HostCallback,
    sample_rate: f64,
    /// We expect num_samples to increase by [sample_rate] per second.
    num_samples: usize,
}

// We're implementing a trait `Plugin` that does all the VST-y stuff for us.
impl Plugin for Whisper {
    fn new(host: HostCallback) -> Self {
        let TimeInfo { sample_rate, .. } = host.get_time_info(TimeInfoFlags::all().bits()).unwrap();
        Whisper {
            host,
            sample_rate,
            ..Default::default()
        }
    }
    fn get_info(&self) -> Info {
        Info {
            name: "Whisper".to_string(),

            // Used by hosts to differentiate between plugins.
            unique_id: 1337,

            // We don't need inputs
            inputs: 0,

            // We do need two outputs though.  This is default, but let's be
            // explicit anyways.
            outputs: 2,

            // Set our category
            category: Category::Synth,

            // We don't care about other stuff, and it can stay default.
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let time_info = self
            .host
            .get_time_info(TimeInfoFlags::NANOSECONDS_VALID.bits());
        if let Some(time_info) = time_info {
            eprintln!(
                "{:?} - {}",
                time_info.nanoseconds,
                TimeInfoFlags::NANOSECONDS_VALID.bits() == time_info.flags
            );
        }
        // `buffer.split()` gives us a tuple containing the
        // input and output buffers.  We only care about the
        // output, so we can ignore the input by using `_`.
        let (_, mut output_buffer) = buffer.split();

        // Now, we want to loop over our output channels.  This
        // includes our left and right channels (or more, if you
        // are working with surround sound).
        let mut outputs = output_buffer.into_iter();
        let left_channel = outputs.next().unwrap();
        let right_channel = outputs.next().unwrap();
        let sample_rate = self.sample_rate;
        for (left_sample, right_sample) in left_channel.iter_mut().zip(right_channel) {
            let vol =
                ((self.num_samples as f32 / sample_rate as f32) * std::f32::consts::PI * 440.0)
                    .sin();
            *left_sample = vol;
            *right_sample = vol;
            self.num_samples += 1;
        }
    }
}

plugin_main!(Whisper);
