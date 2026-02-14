use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        let mut num: u32 = 8;

        loop {
        let guess_status = player.ask_to_compare(num);

        if guess_status == 0 {
            return num

    
        } else if guess_status == 1 {
            if num < max-1 {
                num = num + 1;
            } else {
                return max-1
            }
            
        } else if guess_status == -1 {
            if num > min {
                num = num - 1;
            } else {
                return min   
            }
        
        } else {
            return num
        }

            }

            
        }
    }

