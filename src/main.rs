use midly::num::u15;
use midly::{Format, Header, Smf, Timing};

use crate::generator::{BassDrumGenerator, Generator};

pub mod generator {
    use midly::num::{u28, u4, u7};
    use midly::MidiMessage::{NoteOff, NoteOn};
    use midly::{TrackEvent, TrackEventKind};

    /// [TrackEvent] Generator.
    pub trait Generator {
        /// Generates a [TrackEvent].
        /// If [None] is generated, generator is terminated.
        fn gen_next(&mut self) -> Option<TrackEvent<'static>>;
    }

    /// Generates bass drum note every 120 ticks.
    #[derive(Debug, Clone, Default)]
    pub struct BassDrumGenerator {
        index: usize,
    }

    impl BassDrumGenerator {
        const EVENTS: usize = 2;
        const BASS_EVENTS: [TrackEvent<'static>; Self::EVENTS] = [
            TrackEvent {
                delta: u28::new(30),
                kind: TrackEventKind::Midi {
                    channel: u4::new(9),
                    message: NoteOff {
                        key: u7::new(35),
                        vel: u7::new(127),
                    },
                },
            },
            TrackEvent {
                delta: u28::new(90),
                kind: TrackEventKind::Midi {
                    channel: u4::new(9),
                    message: NoteOn {
                        key: u7::new(35),
                        vel: u7::new(127),
                    },
                },
            },
        ];
    }

    impl Generator for BassDrumGenerator {
        fn gen_next(&mut self) -> Option<TrackEvent<'static>> {
            self.index += 1;
            Some(Self::BASS_EVENTS[(self.index - 1) % Self::EVENTS])
        }
    }
}

fn main() -> std::io::Result<()> {
    let header = Header::new(Format::Parallel, Timing::Metrical(u15::new(120)));
    let tracks = {
        let mut bdg = BassDrumGenerator::default();
        std::iter::from_fn(move || bdg.gen_next())
            .take(256)
            .collect::<Vec<_>>()
    };

    let mut midi = Smf::new(header);
    midi.tracks.push(vec![]);
    midi.tracks.push(tracks);
    midi.save("a.mid")?;
    Ok(())
}
