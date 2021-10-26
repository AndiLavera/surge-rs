ix!();

use crate::{
    SurgeVoice,
};

impl SurgeVoice {

    //TODO: can we factor this out?
    pub fn free_allocated_elements(&mut self) 
    {
        for i in 0..3 {

            //instead, this call resets the shared ptr in c
            match self.osc[i] {
                Some(ref mut x) => x.init(),
                _ => panic!("where is the oscillator!?"),
            }

            self.osctype[i] = OscillatorType::Off;
        }
    }
}
