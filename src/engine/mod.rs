//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021 Adrián Romero Corchado.
//
//    This file is part of MyRulesIoT.
//
//    MyRulesIoT is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//
//    MyRulesIoT is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with MyRulesIoT.  If not, see <http://www.gnu.org/licenses/>.
//

use std::fmt::{Debug, Display};

// pub trait Engine<A, R, S> {
//     fn reduce(&self, state: &S, action: &A) -> S;
//     fn template(&self, state: &S) -> R;
//     fn is_final(&self, result: &R) -> bool;
// }

// pub trait IOQueue<A, R> {
//     fn take(&self) -> Result<A, io::Error>;
//     fn put(&self, result: &R) -> Result<(), io::Error>;
// }

pub struct Engine<A, R, S> {
    reduce: fn(&S, &A) -> S,
    template: fn(&S) -> R,
    is_final: fn(&R) -> bool,
}

pub struct RuntimeEngine<A, R, S>
where
    A: Debug + Display,
    R: Debug + Display,
    S: Debug + Display,
{
    state: S,
    engine: Engine<A, R, S>,
}

impl<A, R, S> RuntimeEngine<A, R, S>
where
    A: Debug + Display,
    R: Debug + Display,
    S: Default + Debug + Display,
{
    fn new(engine: Engine<A, R, S>) -> Self {
        RuntimeEngine {
            state: Default::default(),
            engine: engine,
        }
    }

    fn step(&mut self, action: A) -> (R, bool) {
        log::info!("Persist action {:?}.", &action);
        self.state = (self.engine.reduce)(&self.state, &action);
        log::info!("Persist state {:?}.", &self.state);
        let result = (self.engine.template)(&self.state);
        log::info!("Persist result {:?}.", &result);
        let is_final = (self.engine.is_final)(&result);
        log::info!("Persist final {:?}.", is_final);
        (result, is_final)
    }
}
