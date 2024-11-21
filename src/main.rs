#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

mod keymap;
mod prelude;
use std::time::Duration;

use prelude::*;
use rktk_keymanager::state::KeyChangeEvent;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut keymap = EMPTY_KEYMAP;
    keymap.layers[0].map[0][0] = KeyAction::Normal(KeyCode::Key(Key::A));
    let mut state = new_state(keymap);

    dbg!(state.update(
        &mut [KeyChangeEvent {
            col: 0,
            row: 0,
            pressed: true
        }],
        (0, 0),
        &[],
        time(0)
    ));

    std::thread::sleep(time(100));

    dbg!(state.update(
        &mut [KeyChangeEvent {
            col: 0,
            row: 0,
            pressed: false
        }],
        (0, 0),
        &[],
        time(1)
    ));

    std::thread::sleep(time(100));
}

fn new_state(
    keymap: Keymap<LAYER_COUNT, ROWS, COLS, ENC_COUNT>,
) -> State<LAYER_COUNT, ROWS, COLS, ENC_COUNT> {
    let mut tap_dance = [const { None }; 8];
    tap_dance[0] = Some(TapDanceConfig {
        tap: [
            Some(KeyCode::Key(Key::A)),
            Some(KeyCode::Key(Key::B)),
            Some(KeyCode::Layer(LayerOp::Toggle(2))),
            None,
        ],
        hold: [
            Some(KeyCode::Modifier(Modifier::LCtrl)),
            Some(KeyCode::Layer(LayerOp::Momentary(1))),
            None,
            None,
        ],
    });

    State::new(
        keymap,
        rktk_keymanager::state::config::StateConfig {
            mouse: MouseConfig {
                auto_mouse_layer: 1,
                auto_mouse_duration: 500,
                auto_mouse_threshold: 5,
                scroll_divider_x: 20,
                scroll_divider_y: -12,
            },
            key_resolver: KeyResolverConfig {
                tap_threshold: 500,
                tap_dance_threshold: 100,
                tap_dance,
            },
            initial_output: Output::Usb,
        },
    )
}

fn time(ms: u64) -> Duration {
    Duration::from_millis(ms)
}
