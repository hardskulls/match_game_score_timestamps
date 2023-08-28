use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    // Any score changed.
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    // Home score changed specifically.
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

enum GetScoreErrors {
    ScoreNotFound = -1,
    OffsetIsNegative = -2,
    GameStampsSetEmpty = -3,
    GameStampsSetEmptyAndOffsetIsNeg = -4,
}

fn err_code(error: GetScoreErrors) -> i32 {
    error as i32
}

fn dup_err_code(code: i32) -> (i32, i32) {
    (code, code)
}

/// This func finds a game stamp by offset.
///
/// As this function returns 2 numbers, and not a struct, clarification is required
/// for which one means what.
///
/// We'll say that the first number is `home` score, and the second is `away` score,
/// as `away` seems secondary to the `home` value, and not the other way around.
///
/// # Errors
/// The smaller tne number is the more severe is the error.
/// 1) Returns (-1, -1) if score is not found.
/// 2) Returns (-2, -2) if `offset` is less than 0.
/// 3) Returns (-3, -3) if `game_stamps` array is empty.
/// 4) Returns (-4, -4) if `game_stamps` array is empty AND `offset` is less than 0.
///
// TODO: Change return type to a struct instead of tuple.
fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    use GetScoreErrors::*;

    let err = |error: GetScoreErrors| dup_err_code(err_code(error));

    match (game_stamps.is_empty(), offset.is_negative()) {
        (true, true) => err(GameStampsSetEmptyAndOffsetIsNeg),
        (true, false) => err(GameStampsSetEmpty),
        (false, true) => err(OffsetIsNegative),
        (false, false) => game_stamps
            .binary_search_by(|stamp| Ord::cmp(&stamp.offset, &offset))
            .map(|idx| game_stamps[idx].score)
            .map(|score| (score.home, score.away))
            .unwrap_or(err(ScoreNotFound)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use GetScoreErrors::*;

    fn err_code(error: GetScoreErrors) -> i32 {
        error as i32
    }

    fn dup_err_code(code: i32) -> (i32, i32) {
        (code, code)
    }

    #[test]
    fn enum_number_test() {
        assert_eq!(err_code(ScoreNotFound), -1);
        assert_eq!(err_code(OffsetIsNegative), -2);
        assert_eq!(err_code(GameStampsSetEmpty), -3);
        assert_eq!(err_code(GameStampsSetEmptyAndOffsetIsNeg), -4);
    }

    #[test]
    fn empty_game_stamps_array_offset_not_negative() {
        let empty_game = vec![];
        let expected_output = dup_err_code(err_code(GameStampsSetEmpty));
        assert_eq!(get_score(&empty_game, 0), expected_output);
        assert_eq!(get_score(&empty_game, 35), expected_output);
    }

    #[test]
    fn empty_game_stamps_array_and_offset_less_than_0() {
        let empty_game = vec![];
        let expected_output = dup_err_code(err_code(GameStampsSetEmptyAndOffsetIsNeg));
        assert_eq!(get_score(&empty_game, -1), expected_output);
        assert_eq!(get_score(&empty_game, -15), expected_output);
    }

    #[test]
    fn populated_game_stamps_offset_less_than_0() {
        let game = generate_game();
        let expected_output = dup_err_code(err_code(OffsetIsNegative));
        assert_eq!(get_score(&game, -1), expected_output);
        assert_eq!(get_score(&game, -15), expected_output);
    }

    #[test]
    fn populated_game_stamps_offset_is_zero() {
        let game = generate_game();
        assert_eq!(get_score(&game, 0), (0, 0));
    }

    #[test]
    fn populated_game_stamps_offset_is_natural_number() {
        let game = generate_game();
        let expected_output = err_code(ScoreNotFound);

        let (home_score, away_score) = get_score(&game, 1);
        dbg!((home_score, away_score));
        assert!(home_score >= expected_output);
        assert!(away_score >= expected_output);

        let (home_score, away_score) = get_score(&game, 35);
        dbg!((home_score, away_score));
        assert!(home_score >= expected_output);
        assert!(away_score >= expected_output);
    }
}
