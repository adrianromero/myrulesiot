//    MyRulesIoT is a rules engine for MQTT
//    Copyright (C) 2021-2025 Adrián Romero Corchado.
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

use std::collections::HashMap;

use linkme::distributed_slice;

use crate::master::SliceFunction;

pub mod forward;
pub mod relay;
pub mod savelist;
pub mod startaction;
pub mod startikea;
pub mod timing;

#[distributed_slice]
pub static SLICEFUNCTIONS: [fn() -> (String, SliceFunction)];

pub fn distributed_engine_functions() -> HashMap<String, SliceFunction> {
    SLICEFUNCTIONS.into_iter().map(|f| f()).collect()
}
