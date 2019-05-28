// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  MushuYoWushu
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/*
Declaring our files as 'pub mod' here allows them to be 'use'd outside of here. In main.rs
for example. Everything that you want to use must have the keyword 'pub', all the way down the function
level.
*/

//Controls the state of the game. I.E. advances the game from one scene to the next
pub mod color_palette;
pub mod main_state;
pub mod phase;
pub mod player;
pub mod player_assets;
pub mod scene_type;
pub mod scene_return_values;
pub mod turns;
pub mod utility_functions;