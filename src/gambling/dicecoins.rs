//This file defines how a Dicecoin is made, and its functions
//Note the spirit of Dicecoin is based off standard RPG dice
//and this library was designed with the cap being the D20
//this should be fine as long as you aren't passing insane numbers

use rand::{thread_rng,Rng};

pub enum DiceType {
    D2,
    D4,
    D6,
    D8,
    D10,
    D10P,
    D12,
    D20,
}

pub struct Dicecoins {
    dice_type: DiceType,
    face_count: u32,
    face_values: Vec<u32>,

}

impl Dicecoins {
    pub fn new(&self, d_type: DiceType, face_vals: Vec<u32>) -> Dicecoins {
        let mut retval;

        //Count number of entries in face_vals and reject if not 2, 4, 6, 8, 10, 12, or 20
        if self.valid_dice(self.d_type, face_values){
            //
        }
        else{
            retval = None;
        }


        retval
    }

    //Rolls the dice the indicated number of times and returns the integer sum
    pub fn roll_x(&self, counter: u32) -> u64 {
        let mut sum = 0;

        //Reject roll request greater than 9,999 or something is amiss
        if counter < 9999 && self.valid_dice(self.dice_type, self.face_values) {
            for x in 0..counter {
                sum += self.roll();
            }
        }

        sum
    }

    //Rolls a dicecoin
    fn roll(&self) -> u64 {
        let dice_type: DiceType = self.dice_type;
        let face_vals: Vec<u32> = self.face_values;
        let index; //this is used to pick a face

        //This cast is ok because face_count will never be higher than 20
        index = thread_rng().gen_range(1, self.face_count) as usize; //Which face was rolled?
        //u32 to u64 is always ok
        face_vals[index] as u64
    }

    //Debugging
    fn valid_dice (&self, d_type: DiceType, d_vals: Vec<u32>) -> bool {
        let flag;

        match d_type {
            DiceType::D2 => {
                if d_vals.len() == 2 {
                    flag = true;
                }
            },
            DiceType::D4 => {
                if d_vals.len() == 4 {
                    flag = true;
                }
            },
            DiceType::D6 => {
                if d_vals.len() == 6 {
                    flag = true;
                }
            },
            DiceType::D8 => {
                if d_vals.len() == 8 {
                    flag = true;
                }
            },
            DiceType::D10 => {
                if d_vals.len() == 10 {
                    flag = true;
                }
            },
            DiceType::D10P => {
                if d_vals.len() == 10 {
                    flag = true;
                }
            },
            DiceType::D12 => {
                if d_vals.len() == 12 {
                    flag = true;
                }
            },
            DiceType::D20 => {
                if d_vals.len() == 20 {
                    flag = true;
                }
            },
            _ => flag = false,
        }
        false
    }
}