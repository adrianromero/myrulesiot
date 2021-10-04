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

mod connection;
pub use connection::{new_connection, publication_loop, subscription_loop};
pub use connection::{ConnectionInfo, TopicInfo};

mod actions;
pub use actions::ConnectionMessage;
pub use actions::ConnectionResult;

mod createengine;
pub use createengine::create_engine;
pub use createengine::{ConnectionEngine, ConnectionReducer, ConnectionState};

mod connectionengine;
pub use connectionengine::connection_engine;
