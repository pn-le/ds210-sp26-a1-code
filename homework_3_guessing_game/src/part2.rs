use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // If max is exclusive, the valid search interval is [min, max-1].
        // Handle empty range first.
       
        let mut low = min;
        let mut high = max - 1; 

        while low <= high {
            let num = low + (high - low) / 2; 
            let guess_status = player.ask_to_compare(num);

            if guess_status == 0 {
                return num;
            } else if guess_status == -1 {
                if num == 0 {
                    break;
                }
                high = num - 1;
            } else if guess_status == 1 {
                low = num + 1;
            } else {
                break;
            }
        }

        min
    }
}