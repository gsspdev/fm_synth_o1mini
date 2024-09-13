use hound;
use std::f32::consts::PI;

/// Configuration constants
const SAMPLE_RATE: u32 = 44100;          // Samples per second
const DURATION_SECONDS: f32 = 5.0;       // Duration of the audio in seconds

/// Structure representing the FM Synthesizer
struct FMSynth {
    carrier_freq: f32,     // Carrier frequency in Hz
    modulator_freq: f32,   // Modulator frequency in Hz
    modulation_index: f32, // Modulation index
}

impl FMSynth {
    /// Creates a new FM Synthesizer instance
    fn new(carrier_freq: f32, modulator_freq: f32, modulation_index: f32) -> Self {
        FMSynth {
            carrier_freq,
            modulator_freq,
            modulation_index,
        }
    }

    /// Generates the next sample at time `t`
    fn next_sample(&self, t: f32) -> f32 {
        // FM synthesis formula: y(t) = sin(2πf_c t + I * sin(2πf_m t))
        let modulator = (2.0 * PI * self.modulator_freq * t).sin();
        (2.0 * PI * self.carrier_freq * t + self.modulation_index * modulator).sin()
    }
}

fn main() {
    // Define WAV file specifications
    let spec = hound::WavSpec {
        channels: 1,                    // Mono audio
        sample_rate: SAMPLE_RATE,       // Sample rate
        bits_per_sample: 16,            // 16-bit audio
        sample_format: hound::SampleFormat::Int, // Integer samples
    };

    // Create a WAV writer
    let mut writer = hound::WavWriter::create("fm_synth.wav", spec)
        .expect("Failed to create WAV writer");

    // Initialize the FM Synthesizer
    // Example: A4 note (440 Hz) as carrier, A3 note (220 Hz) as modulator
    let synth = FMSynth::new(440.0, 220.0, 100.0);

    // Calculate total number of samples
    let total_samples = (SAMPLE_RATE as f32 * DURATION_SECONDS) as usize;

    // Generate and write each sample
    for n in 0..total_samples {
        let t = n as f32 / SAMPLE_RATE as f32; // Current time in seconds
        let sample = synth.next_sample(t);

        // Scale the sample from [-1.0, 1.0] to i16 range
        let amplitude = i16::MAX as f32;
        let sample_i16 = (sample * amplitude) as i16;

        // Write the sample to the WAV file
        writer.write_sample(sample_i16)
            .expect("Failed to write sample");
    }

    // Finalize the WAV file
    writer.finalize().expect("Failed to finalize WAV file");

    println!("FM synthesis complete! Output written to fm_synth.wav");
}

