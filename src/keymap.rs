//! common keymap for test

use rktk_keymanager::{
    keycode::{KeyCode, _____},
    Keymap, Layer, LayerMap,
};

pub const ROWS: usize = 5;
pub const COLS: usize = 14;
pub const LAYER_COUNT: usize = 5;
pub const ENC_COUNT: usize = 1;

#[rustfmt::skip]
/// Auto mouse layer
pub const EMPTY_LAYER: LayerMap<ROWS,COLS> = [
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
    [ _____ , _____ , _____ , _____ , _____ , _____ , _____ , /**/ _____ , _____ , _____ , _____ , _____ , _____ , _____ ],
];

pub const EMPTY_KEYMAP: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT> = Keymap {
    layers: [
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: false,
        },
        Layer {
            map: EMPTY_LAYER,
            arrowmouse: true,
        },
    ],
    encoder_keys: [(KeyCode::None, KeyCode::None)],
};
