use nalgebra::Vector2;

use crate::{Bounds, key::Key};

pub type KeyboardMatrix = [[Key; 21]; 6];

pub const W60HE_KEYS: [(u8, u8); 62] = [
    ESC,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    ZERO,
    PLUS,
    BACKTICK,
    BACKSPACE,
    TAB,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    SWEDISH_O,
    CARET,
    CAPSLOCK,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    O_WITH_DOTS,
    A_WITH_DOTS,
    STAR,
    ENTER,
    LEFT_SHIFT,
    LESS_THAN,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    COMMA,
    DOT,
    MINUS,
    RIGHT_SHIFT,
    LEFT_CONTROL,
    LEFT_MOD,
    LEFT_ALT,
    SPACE,
    RIGHT_ALT,
    RIGHT_MOD,
    FN,
    RIGHT_CONTROL,
];

// Maybe at some point:
// The whole matrix
// done
pub const RAW_MATRIX_60HE: [[(u8, u8); 14]; 5] = [
    [
        ESC, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, ZERO, PLUS, BACKTICK, BACKSPACE,
    ],
    [TAB, Q, W, E, R, T, Y, U, I, O, P, SWEDISH_O, CARET, (2, 13)],
    [
        CAPSLOCK,
        A,
        S,
        D,
        F,
        G,
        H,
        J,
        K,
        L,
        O_WITH_DOTS,
        A_WITH_DOTS,
        STAR,
        ENTER,
    ],
    [
        LEFT_SHIFT,
        LESS_THAN,
        Z,
        X,
        C,
        V,
        B,
        N,
        M,
        COMMA,
        DOT,
        MINUS,
        (4, 12),
        RIGHT_SHIFT,
    ],
    [
        LEFT_CONTROL,
        LEFT_MOD,
        LEFT_ALT,
        (5, 3),
        (5, 4),
        (5, 5),
        SPACE,
        (5, 7),
        (5, 8),
        (5, 9),
        RIGHT_ALT,
        RIGHT_MOD,
        RIGHT_CONTROL,
        FN,
    ],
];

/// Raw full matrix of any wooting
pub const RAW_MATRIX: [[(u8, u8); 21]; 6] = [
    [
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
        (0, 8),
        (0, 9),
        (0, 10),
        (0, 11),
        (0, 12),
        (0, 13),
        (0, 14),
        (0, 15),
        (0, 16),
        (0, 17),
        (0, 18),
        (0, 19),
        (0, 20),
    ],
    [
        (1, 0),
        (1, 1),
        (1, 2),
        (1, 3),
        (1, 4),
        (1, 5),
        (1, 6),
        (1, 7),
        (1, 8),
        (1, 9),
        (1, 10),
        (1, 11),
        (1, 12),
        (1, 13),
        (1, 14),
        (1, 15),
        (1, 16),
        (1, 17),
        (1, 18),
        (1, 19),
        (1, 20),
    ],
    [
        (2, 0),
        (2, 1),
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (2, 6),
        (2, 7),
        (2, 8),
        (2, 9),
        (2, 10),
        (2, 11),
        (2, 12),
        (2, 13),
        (2, 14),
        (2, 15),
        (2, 16),
        (2, 17),
        (2, 18),
        (2, 19),
        (2, 20),
    ],
    [
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (3, 4),
        (3, 5),
        (3, 6),
        (3, 7),
        (3, 8),
        (3, 9),
        (3, 10),
        (3, 11),
        (3, 12),
        (3, 13),
        (3, 14),
        (3, 15),
        (3, 16),
        (3, 17),
        (3, 18),
        (3, 19),
        (3, 20),
    ],
    [
        (4, 0),
        (4, 1),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
        (4, 6),
        (4, 7),
        (4, 8),
        (4, 9),
        (4, 10),
        (4, 11),
        (4, 12),
        (4, 13),
        (4, 14),
        (4, 15),
        (4, 16),
        (4, 17),
        (4, 18),
        (4, 19),
        (4, 20),
    ],
    [
        (5, 0),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (5, 6),
        (5, 7),
        (5, 8),
        (5, 9),
        (5, 10),
        (5, 11),
        (5, 12),
        (5, 13),
        (5, 14),
        (5, 15),
        (5, 16),
        (5, 17),
        (5, 18),
        (5, 19),
        (5, 20),
    ],
];

pub fn compute_bounds(matrix: &KeyboardMatrix) -> Bounds {
    let points = matrix.map(|row| row.map(|key| key.pos_norm));
    let points = points.as_flattened();
    let mut min_x = points[0].x;
    let mut max_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_y = points[0].y;

    for point in points.iter().skip(1) {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
    }

    Bounds {
        position: Vector2::new(min_x, min_y),
        size: Vector2::new(max_x - min_x, max_y - min_y),
    }
}

/// Get the full keyboard matrix where every position is included with key data
pub fn get_matrix() -> KeyboardMatrix {
    let mut matrix = [[Key::default(); 21]; 6];
    let min: Vector2<f64> = Vector2::new(0.0, 0.0);
    let max: Vector2<f64> = Vector2::new(14.0, 4.0);
    let size = max - min;
    let aspect = size.x / size.y;

    for (x, row) in RAW_MATRIX.iter().enumerate() {
        for (y, key_pos) in row.iter().enumerate() {
            let key = &mut matrix[x][y];
            key.key = *key_pos;
            key.physical_position = match key.key {
                BACKSPACE => Vector2::new(13.5, 1.0),
                ENTER => Vector2::new(13.5, 2.5),
                RIGHT_SHIFT => Vector2::new(12.5, 4.0),
                _ => Vector2::new(key_pos.1 as f64, key_pos.0 as f64),
            };
            key.pos_norm = Vector2::new(
                (key.physical_position.x - min.x) / size.x,
                (key.physical_position.y - min.y) / size.y,
            );
            // normalize
            key.pos_norm -= Vector2::new(0.5, 0.5);
            // Apply manual offset for centre correction
            key.pos_norm += Vector2::new(0.05, 0.0);
            // aspect ratio corrected
            key.pos_norm_aspect.x = key.pos_norm.x * aspect;
            key.pos_norm_aspect.y = key.pos_norm.y;
        }
    }

    matrix
}

pub const ESC: (u8, u8) = (1, 0);
pub const ONE: (u8, u8) = (1, 1);
pub const TWO: (u8, u8) = (1, 2);
pub const THREE: (u8, u8) = (1, 3);
pub const FOUR: (u8, u8) = (1, 4);
pub const FIVE: (u8, u8) = (1, 5);
pub const SIX: (u8, u8) = (1, 6);
pub const SEVEN: (u8, u8) = (1, 7);
pub const EIGHT: (u8, u8) = (1, 8);
pub const NINE: (u8, u8) = (1, 9);
pub const ZERO: (u8, u8) = (1, 10);
pub const PLUS: (u8, u8) = (1, 11);
pub const BACKTICK: (u8, u8) = (1, 12);
pub const BACKSPACE: (u8, u8) = (1, 13);

pub const TAB: (u8, u8) = (2, 0);
pub const Q: (u8, u8) = (2, 1);
pub const W: (u8, u8) = (2, 2);
pub const E: (u8, u8) = (2, 3);
pub const R: (u8, u8) = (2, 4);
pub const T: (u8, u8) = (2, 5);
pub const Y: (u8, u8) = (2, 6);
pub const U: (u8, u8) = (2, 7);
pub const I: (u8, u8) = (2, 8);
pub const O: (u8, u8) = (2, 9);
pub const P: (u8, u8) = (2, 10);
pub const SWEDISH_O: (u8, u8) = (2, 11);
pub const CARET: (u8, u8) = (2, 12);

pub const CAPSLOCK: (u8, u8) = (3, 0);
pub const A: (u8, u8) = (3, 1);
pub const S: (u8, u8) = (3, 2);
pub const D: (u8, u8) = (3, 3);
pub const F: (u8, u8) = (3, 4);
pub const G: (u8, u8) = (3, 5);
pub const H: (u8, u8) = (3, 6);
pub const J: (u8, u8) = (3, 7);
pub const K: (u8, u8) = (3, 8);
pub const L: (u8, u8) = (3, 9);
pub const O_WITH_DOTS: (u8, u8) = (3, 10);
pub const A_WITH_DOTS: (u8, u8) = (3, 11);
pub const STAR: (u8, u8) = (3, 12);
pub const ENTER: (u8, u8) = (3, 13);

pub const LEFT_SHIFT: (u8, u8) = (4, 0);
pub const LESS_THAN: (u8, u8) = (4, 1);
pub const Z: (u8, u8) = (4, 2);
pub const X: (u8, u8) = (4, 3);
pub const C: (u8, u8) = (4, 4);
pub const V: (u8, u8) = (4, 5);
pub const B: (u8, u8) = (4, 6);
pub const N: (u8, u8) = (4, 7);
pub const M: (u8, u8) = (4, 8);
pub const COMMA: (u8, u8) = (4, 9);
pub const DOT: (u8, u8) = (4, 10);
pub const MINUS: (u8, u8) = (4, 11);
pub const RIGHT_SHIFT: (u8, u8) = (4, 13);

pub const LEFT_CONTROL: (u8, u8) = (5, 0);
pub const LEFT_MOD: (u8, u8) = (5, 1);
pub const LEFT_ALT: (u8, u8) = (5, 2);
pub const SPACE: (u8, u8) = (5, 6);
pub const RIGHT_ALT: (u8, u8) = (5, 10);
pub const RIGHT_MOD: (u8, u8) = (5, 11);
pub const RIGHT_CONTROL: (u8, u8) = (5, 12);
pub const FN: (u8, u8) = (5, 13);
