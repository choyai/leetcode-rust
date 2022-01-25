/**
 * [1510] Stone Game IV
 *
 * Alice and Bob take turns playing a game, with Alice starting first.
 * Initially, there are n stones in a pile. On each player's turn, that player makes a move consisting of removing any non-zero square number of stones in the pile.
 * Also, if a player cannot make a move, he/she loses the game.
 * Given a positive integer n, return true if and only if Alice wins the game otherwise return false, assuming both players play optimally.
 *  
 * Example 1:
 * 
 * Input: n = 1
 * Output: true
 * Explanation: Alice can remove 1 stone winning the game because Bob doesn't have any moves.
 * Example 2:
 * 
 * Input: n = 2
 * Output: false
 * Explanation: Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).
 * 
 * Example 3:
 * 
 * Input: n = 4
 * Output: true
 * Explanation: n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stone-game-iv/
// discuss: https://leetcode.com/problems/stone-game-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// use std::collections::HashMap;
// impl Solution {
//     pub fn winner_square_game(n: i32) -> bool {
//         fn dp_helper(n: i32, memo: &mut HashMap<i32, bool>) -> bool {
//             match memo.get(&n) {
//                 Some(&val) => val,
//                 None => {
//                     let vec: Vec<i32> = (1..=n).filter(|&x| x*x <= n).collect();
//                     let res = match n {
//                         0 => false,
//                         k if vec.iter().any(|&i| k - i*i == 0) => true,
//                         _ => vec.iter().any(|&i| !dp_helper(n - i*i, memo)),
//                     };
//                     memo.insert(n, res);
//                     res
//                 }
//             }
//         }
//         let mut cache: HashMap<i32, bool> = HashMap::new();
//         dp_helper(n, &mut cache)
//     }
// }

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp = vec![false; n as usize + 1];
        let mut sqr = vec![1];
        for i in 1..=n {
            if i == sqr[0] {
                dp[i as usize] = true;
                continue;
            }
            if i > sqr[0] {
                sqr.insert(0, sqr[0] + 2 * sqr.len() as i32 + 1)
            }
            for j in 1..sqr.len() {
                if dp[(i - sqr[j]) as usize] == false {
                    dp[i as usize] = true;
                    break;
                }
            }
        }
        dp[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1510_base() {
        assert_eq!(Solution::winner_square_game(1), true);
    }

    #[test]
    fn test_1510_2() {
        assert_eq!(Solution::winner_square_game(2), false);
    }

    #[test]
    fn test_1510_4() {
        assert_eq!(Solution::winner_square_game(4), true);
    }

    #[test]
    fn test_1510_multi() {
        assert_eq!(Solution::winner_square_game(8), true);
        assert_eq!(Solution::winner_square_game(7), false);
        assert_eq!(Solution::winner_square_game(9), true);
        assert_eq!(Solution::winner_square_game(5), false);
        assert_eq!(Solution::winner_square_game(92719), true);
    }


}
