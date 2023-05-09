use rodio::{OutputStream, Sink};
use rodio::source::{Source};
use rodio::{buffer::SamplesBuffer};
use std::f32::consts::PI;

trait Synthesizer {
    fn new(playtime: f32, sample_rate: usize) -> Self;
    fn calc_buffer(&mut self, frequency: f32) -> Vec<f32>;
    fn calc_sample(&mut self, frequency: f32, phase: f32) -> f32;
}
enum SynthType {
    Sine(SineSynth),
    Square(SquareSynth)
}

struct SineSynth {
    playtime: f32,
    sample_rate: usize
}

impl Synthesizer for SineSynth {
    fn new(playtime: f32, sample_rate: usize) -> SineSynth {
        SineSynth {
            playtime:playtime,
            sample_rate: sample_rate
        }
    }
    fn calc_buffer(&mut self, frequency: f32) -> Vec<f32> {
        let mut out: Vec<f32> = Vec::with_capacity(0);
        for i in 0..(self.playtime as f32 * self.sample_rate as f32) as i32 {
            let t = i as f32 / self.sample_rate as f32;
            out.push((2.0 * PI * frequency * t).sin());
        }
        out
    }
    fn calc_sample(&mut self, frequency: f32, phase: f32) -> f32 {
        let phase_out = phase * frequency;
        phase_out.sin()
    }
}

struct SquareSynth {
    playtime: f32,
    sample_rate: usize
}

impl Synthesizer for SquareSynth {
    fn new(playtime: f32, sample_rate: usize) -> SquareSynth {
        SquareSynth {
            playtime:playtime,
            sample_rate: sample_rate
        }
    }
    fn calc_buffer(&mut self, frequency: f32) -> Vec<f32> {
        let mut out: Vec<f32> = Vec::with_capacity(0);
        for i in 0..(self.playtime as f32 * self.sample_rate as f32) as i32 {
            let t = i as f32 / self.sample_rate as f32;
            let mut pseudo_sample = (2.0 * PI * frequency * t).sin();
            let mut sample = 0.0;
            if pseudo_sample > 0.0 {
                sample = 0.7;
            }
            else if pseudo_sample < 0.0 {
                sample = -0.7;
            }
            else {
                sample = 0.0;
            }
            out.push(sample);
        }
        out
    }
    fn calc_sample(&mut self, frequency: f32, phase: f32) -> f32 {
        phase.sin()
    }
}



struct GeSynth {
    sample_rate: usize
}

impl GeSynth {
    fn new(sample_rate: i32) -> GeSynth {
        println!("GeSynth initialized!");
        GeSynth {
            sample_rate: sample_rate as usize
        }
    }
    fn play(&mut self, playtime: f32) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let sample_rate = 48000.0;
        let mut s = SineSynth::new(0.2, sample_rate as usize);
        let mut s2 = SineSynth::new(0.2, sample_rate as usize);
        let mut phase = 0.0;
        while true {
            /*let mut wave = s.calc_buffer(440.0);
            let source = SamplesBuffer::new(1, 48000, wave).buffered();
            sink.append(source);
            let mut wave = s.calc_buffer(220.0);
            let source = SamplesBuffer::new(1, 48000, wave).buffered();
            sink.append(source);
            let mut wave = s.calc_buffer(110.0);
            let source = SamplesBuffer::new(1, 48000, wave).buffered();
            sink.append(source);
            let mut wave = s.calc_buffer(55.0);
            let source = SamplesBuffer::new(1, 48000, wave).buffered();
            sink.append(source);*/
            let mut pb: Vec<f32> = Vec::with_capacity(0);
            for bs in 0..sample_rate as i32 {
                phase += (1.0 / sample_rate) * 2.0 * PI;
                let a = s2.calc_sample(1.0, phase);
                
                let x = s.calc_sample(220.0*a.abs(), phase);
                println!("{}", phase);
                pb.push(x);
            }
            let source = SamplesBuffer::new(1, sample_rate as u32, pb).buffered();
            sink.append(source);
        }
        sink.sleep_until_end();
    }
}

fn main() {
    
    let mut g = GeSynth::new(48000);
    g.play(3.0);
}

