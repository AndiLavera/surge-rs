use crate::*;

pub struct SpencaPlugin {
    sample_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
}

impl SpencaPlugin {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    /// Process an incoming midi event.
    ///
    /// The midi data is split up like so:
    ///
    /// `data[0]`: Contains the status and the channel. Source: [source]
    /// `data[1]`: Contains the supplemental data for the message - so, if this was a NoteOn then
    ///            this would contain the note.
    /// `data[2]`: Further supplemental data. Would be velocity in the case of a NoteOn message.
    ///
    /// [source]: http://www.midimountain.com/midi/midi_status.htm
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }
}

impl Default for SpencaPlugin {
    fn default() -> SpencaPlugin {
        SpencaPlugin {
            sample_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: Some(59),
        }
    }
}

// We're implementing a trait `Plugin` that does all the VST-y stuff for us.
impl Plugin for SpencaPlugin {

    fn get_info(&self) -> Info {
        Info {
            name: "SpencaPlugin".to_string(),
            vendor: "KlebsSoftware".to_string(),
            unique_id: 6667,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    // Supresses warning about match statment onlt having one arm
    #[allow(unknown_lints)]
    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate.into();
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();

        let per_sample = self.time_per_sample();

        for (input_buffer, output_buffer) in buffer.zip() {
            let mut t = self.time;

            for (_, output_sample) in input_buffer.iter().zip(output_buffer) {

                if let Some(current_note) = self.note {

                    let signal = (t * midi_pitch_to_freq(current_note) * TAU).sin();

                    // Apply a quick envelope to the attack of the signal to avoid popping.
                    let attack = 0.5;

                    let alpha = if self.note_duration < attack {
                        self.note_duration / attack
                    } else {
                        1.0
                    };

                    *output_sample = (signal * alpha) as f32;

                    t += per_sample;

                } else {
                    *output_sample = 0.0;
                }
            }
        }
        self.time          += samples as f64 * per_sample;
        self.note_duration += samples as f64 * per_sample;
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(SpencaPlugin);

#[test] fn test_midi_pitch_to_freq() 
{
    for i in 0..127 {
        // expect no panics
        midi_pitch_to_freq(i);
    }
}
