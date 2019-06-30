#[derive(Copy, Clone)]
pub struct Kalaha {
    buckets: [u8; 14],
}

pub enum AfterMove {
    Regular {
        kalaha: Kalaha,
        next_move: u8,
    },
    Winner {
        winner: u8
    }
}

pub const PLAYER_ONE: u8 = 0;
pub const PLAYER_TWO: u8 = 1;
pub const REMIS: u8 = 2;

pub fn other_player(p: u8) -> u8 {
    1 - p
}

impl Kalaha {
    pub fn new() -> Kalaha {
        let mut buckets = [4; 14];
        buckets[6] = 0;
        buckets[13] = 0;
        Kalaha { buckets }
    }

    #[allow(dead_code)]
    fn of(buckets: [u8; 14]) -> Kalaha {
        Kalaha { buckets }
    }

    #[inline(always)]
    pub fn get_bucket(&self, idx: usize) -> u8 {
        self.buckets[idx]
    }

    fn set_bucket(&mut self, idx: usize, value: u8) {
        self.buckets[idx] = value;
    }

    fn inc_bucket(&mut self, idx: usize, inc: u8) {
        self.buckets[idx] += inc;
    }

    #[inline(always)]
    pub fn kalahas(player: u8) -> (usize, usize) {
        if player == PLAYER_ONE {
            (6, 13)
        } else if player == PLAYER_TWO {
            (13, 6)
        } else {
            panic!("Invalid player {}", player)
        }
    }

    fn opposite_bucket(idx: usize) -> usize {
        debug_assert!(idx <= 12);
        12 - idx
    }

    #[inline(always)]
    fn check_idx(idx: usize, player: u8) {
        if player == PLAYER_ONE {
            debug_assert!(idx < 6)
        } else if player == PLAYER_TWO {
            debug_assert!(idx > 6 && idx < 13)
        } else {
            panic!("Invalid player {}", player)
        }
    }

    fn stones_count(&self, player: u8) -> u8 {
        let range = if player == PLAYER_ONE { 0..=5 } else { 7..=12 };
        let mut sum = 0;
        for i in range {
            sum += self.get_bucket(i);
        }
        sum
    }

    pub fn make_move(&self, mut idx: usize, player: u8) -> AfterMove {
        Kalaha::check_idx(idx, player);
        let (my_kalaha, other_kalaha) = Kalaha::kalahas(player);
        let mut stones = self.get_bucket(idx);
        let mut copy = *self;
        copy.buckets[idx] = 0;
        while stones != 0 {
            if idx == 13 { idx = 0 } else { idx += 1 };
            if idx == other_kalaha { continue }
            copy.buckets[idx] += 1;
            stones -= 1;
        }
        let next_move: u8;
        if idx == my_kalaha {
            next_move = player
        } else {
            next_move = other_player(player);
            if copy.get_bucket(idx) == 1 {
                let opposite_idx = Kalaha::opposite_bucket(idx);
                copy.inc_bucket(my_kalaha, 1);
                copy.inc_bucket(my_kalaha, copy.get_bucket(opposite_idx));
                copy.set_bucket(idx, 0);
                copy.set_bucket(opposite_idx, 0);
            }
        }
        let my_stones = copy.stones_count(player);
        let other_stones = copy.stones_count(other_player(player));
        if my_stones == 0 { copy.inc_bucket(other_kalaha, other_stones) }
        else if other_stones == 0 { copy.inc_bucket(my_kalaha, my_stones) }
        let my_result = copy.get_bucket(my_kalaha);
        let other_result = copy.get_bucket(other_kalaha);
        if my_result > 24 {
            AfterMove::Winner { winner: player }
        } else if other_result > 24 {
            AfterMove::Winner { winner: other_player(player) }
        } else if other_result == 24 && my_result == 24 {
            AfterMove::Winner { winner: REMIS }
        } else {
            AfterMove::Regular { kalaha: copy, next_move }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod check_idx_tests {
        use super::*;

        #[test]
        fn check_idx_valid() {
            Kalaha::check_idx(5, PLAYER_ONE);
            Kalaha::check_idx(0, PLAYER_ONE);
            Kalaha::check_idx(7, PLAYER_TWO);
            Kalaha::check_idx(12, PLAYER_TWO);
        }

        #[test]
        #[should_panic]
        fn check_idx_6_fails() {
            Kalaha::check_idx(6, PLAYER_ONE);
        }

        #[test]
        #[should_panic]
        fn check_idx_13_fails() {
            Kalaha::check_idx(13, PLAYER_ONE);
        }

        #[test]
        #[should_panic]
        fn check_idx_0_fails_for_player_two() {
            Kalaha::check_idx(0, PLAYER_TWO);
        }

        #[test]
        #[should_panic]
        fn check_idx_12_fails_for_player_one() {
            Kalaha::check_idx(12, PLAYER_ONE);
        }
    }

    #[test]
    fn new_kalaha() {
        let kalaha = Kalaha::new();
        assert_eq!(kalaha.buckets, [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0]);
    }

    #[test]
    fn test_kalahas() {
        assert_eq!((6, 13), Kalaha::kalahas(PLAYER_ONE));
        assert_eq!((13, 6), Kalaha::kalahas(PLAYER_TWO));
    }

    mod move_tests {
        use super::*;

        #[test]
        fn test_regular_move() {
            let kalaha = Kalaha::new();
            if let AfterMove::Regular { kalaha, next_move } = kalaha.make_move(3, PLAYER_ONE) {
                assert_eq!(next_move, PLAYER_TWO);
                assert_eq!(kalaha.buckets, [4, 4, 4, 0, 5, 5, 1, 5, 4, 4, 4, 4, 4, 0]);
            } else { panic!() }
        }

        #[test]
        fn test_move_that_ends_in_kalaha() {
            let kalaha = Kalaha::new();
            if let AfterMove::Regular { kalaha, next_move } = kalaha.make_move(2, PLAYER_ONE) {
                assert_eq!(next_move, PLAYER_ONE);
                assert_eq!(kalaha.buckets, [4, 4, 0, 5, 5, 5, 1, 4, 4, 4, 4, 4, 4, 0]);
            } else { panic!() }
        }

        #[test]
        fn test_skips_other_kalaha() {
            let kalaha = Kalaha::of([0, 0, 1, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0]);
            if let AfterMove::Regular { kalaha, next_move: _ } = kalaha.make_move(5, PLAYER_ONE) {
                assert_eq!(kalaha.buckets, [1, 1, 2, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0]);
            } else { panic!() }
        }

        #[test]
        fn test_clear_other_bucket() {
            let kalaha = Kalaha::of([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0]);
            if let AfterMove::Regular { kalaha, next_move: _ } = kalaha.make_move(0, PLAYER_ONE) {
                assert_eq!(kalaha.buckets, [0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
            } else { panic!() }
        }
    }
}

