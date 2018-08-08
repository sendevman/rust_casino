// mehens_portable_casino. A gambling game made using ggez and Dicecoin
// Copyright (C) 2018  Ian L. Gore
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

//My imports
use game_logic::scene_return_values::SceneReturn;
use game_logic::utility_functions::*;
//We handle the dice by enums 'cuz that can be controlled easily
#[allow(unused_imports)]
use gambling::dice_type::DiceType;

//Ggez
use ggez::graphics::{FilterMode,Image, Point2, draw, set_default_filter};
use ggez::graphics::spritebatch::{SpriteBatch};

use ggez::event;
use ggez::event::{MouseButton, Keycode};

use ggez::audio::Source;

use ggez::{Context, GameResult};

/*
Here I define all the assets I will need to run a particular scene.
*/

pub struct DicecoinMPC {
    //Background image
    background_dc_mpc: SpriteBatch,
    //Sounds
    bad_boop: Source,
    good_boop: Source,
    //Enter buttons and offset variables
    enter: SpriteBatch,
    enter_offset: (f32,f32),
    go_up_enter: bool,
    enter_flip: SpriteBatch,
    enter_flip_offset: (f32,f32),
    go_up_enter_flip: bool,
    //Environment variables

}

#[allow(unused_variables)]
#[allow(dead_code)]
impl DicecoinMPC {
    //We should not worry about framerate limiting here since MainState handles calls
    pub fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        //Here we control the animation for the enter button
        let new_pos = float_animation(0.15, -0.15, 0.03, self.enter_offset.1, self.go_up_enter, ctx);
        self.enter_offset = (self.enter_offset.0, new_pos.1);
        self.go_up_enter = new_pos.0;

        //Here we control the animation for the flipped enter button
        let new_pos_flip = float_animation(0.15, -0.15, 0.03, self.enter_flip_offset.1, self.go_up_enter_flip, ctx);
        self.enter_flip_offset = (self.enter_flip_offset.0, new_pos_flip.1);
        self.go_up_enter_flip = new_pos_flip.0;

        Ok(())
    }

    //We won't worry about clearing or presenting either since MainState handles that too
    //Also you MUST add params to your Spritebatch every draw call for it to render.
    pub fn draw(&mut self, ctx: &mut Context, _screen_center: &(f32,f32)) -> GameResult<()> {
        //Draws MPC on screen
        self.background_dc_mpc.add(make_def_param());
        draw(ctx,&self.background_dc_mpc, Point2::new(0.0, 0.0), 0.0)?;
        self.background_dc_mpc.clear();

        //Draws Enter button on screen
        self.enter.add(make_param((649.0,414.0), (1.0,1.0), 0.0, (0.0, self.enter_offset.1)));
        draw(ctx,&self.enter, Point2::new(0.0, 0.0), 0.0)?;
        self.enter.clear();

        //Draws EnterReverse button on screen
        self.enter_flip.add(make_param((36.0,34.0), (1.0,1.0), 0.0, (0.0, self.enter_flip_offset.1)));
        draw(ctx,&self.enter_flip, Point2::new(0.0, 0.0), 0.0)?;
        self.enter_flip.clear();

        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32){}

    pub fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) -> SceneReturn{
        match keycode {
            Keycode::Return => SceneReturn::Finished,
            _               => SceneReturn::Good,
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::Keycode, _keymod: event::Mod, _repeat: bool) {}
}

impl DicecoinMPC {
    pub fn new(ctx: &mut Context) -> GameResult<DicecoinMPC> {
        //Note we should set defaults in each module so we can guarantee proper behavior
        set_default_filter(ctx, FilterMode::Nearest);

        //Background allocation
        let bg = Image::new(ctx, "/Dicecoin GameBoard.png")?;
        let bg_spr = SpriteBatch::new(bg);

        //Enter button allocations
        let enter = Image::new(ctx, "/EnterAdjusted.png")?;
        let enter_spr = SpriteBatch::new(enter);
        let enter_flipped = Image::new(ctx, "/EnterReverse.png")?;
        let enter_flipped_spr = SpriteBatch::new(enter_flipped);

        //Sound allocations
        let b_boop = Source::new(ctx, "/beep4.ogg")?;
        let g_boop = Source::new(ctx, "/Bleep Sound.wav")?;

        let x = DicecoinMPC {
            //Background
            background_dc_mpc: bg_spr,
            //Sounds
            bad_boop: b_boop,
            good_boop: g_boop,
            //Enter buttons and environment variables
            enter: enter_spr,
            enter_offset: (0.0,0.0),
            go_up_enter: true,
            enter_flip: enter_flipped_spr,
            enter_flip_offset: (0.0,0.0),
            go_up_enter_flip: false, //It looks better to have them travelling in opposite directions methinks
        };
        Ok(x)
    }
}
