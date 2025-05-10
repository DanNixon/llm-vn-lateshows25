use crate::{rpc::AppTx, ButtonResources};
use defmt::{info, warn};
use embassy_rp::gpio::{Input, Level, Pull};
use embassy_time::{Duration, Instant, Ticker};
use heapless::HistoryBuffer;
use icd::{ButtonAction, ButtonActionPerformed};
use postcard_rpc::server::Sender;

struct PhysicalButtonInputs {
    fn_1: Input<'static>,
    fn_2: Input<'static>,
    fn_3: Input<'static>,
    end_conversation: Input<'static>,
}

#[derive(PartialEq, Eq)]
struct PhysicalButtonReadings {
    fn_1: Level,
    fn_2: Level,
    fn_3: Level,
    end_conversation: Level,
}

impl PhysicalButtonReadings {
    const FN_1_PRESSED: Self = Self {
        fn_1: Level::Low,
        fn_2: Level::High,
        fn_3: Level::High,
        end_conversation: Level::High,
    };

    const FN_2_PRESSED: Self = Self {
        fn_1: Level::High,
        fn_2: Level::Low,
        fn_3: Level::High,
        end_conversation: Level::High,
    };

    const FN_3_PRESSED: Self = Self {
        fn_1: Level::High,
        fn_2: Level::High,
        fn_3: Level::Low,
        end_conversation: Level::High,
    };

    const END_CONVERSATION_PRESSED: Self = Self {
        fn_1: Level::High,
        fn_2: Level::High,
        fn_3: Level::High,
        end_conversation: Level::Low,
    };
}

struct ButtonReadingEvent {
    #[allow(unused)]
    time: Instant,
    readings: PhysicalButtonReadings,
}

impl ButtonReadingEvent {
    fn new(readings: PhysicalButtonReadings) -> Self {
        Self {
            time: Instant::now(),
            readings,
        }
    }
}

#[embassy_executor::task]
pub async fn run(r: ButtonResources, rpc_sender: Sender<AppTx>) {
    let inputs = PhysicalButtonInputs {
        fn_1: Input::new(r.fn_1, Pull::Up),
        fn_2: Input::new(r.fn_2, Pull::Up),
        fn_3: Input::new(r.fn_3, Pull::Up),
        end_conversation: Input::new(r.end_conversation, Pull::Up),
    };

    let mut ticker = Ticker::every(Duration::from_hz(20));

    let mut history = HistoryBuffer::<ButtonReadingEvent, 4>::new();
    let mut seq = 0u8;

    loop {
        ticker.next().await;

        let readings = PhysicalButtonReadings {
            fn_1: inputs.fn_1.get_level(),
            fn_2: inputs.fn_2.get_level(),
            fn_3: inputs.fn_3.get_level(),
            end_conversation: inputs.end_conversation.get_level(),
        };

        let changed = match history.recent() {
            Some(event) => {
                if event.readings != readings {
                    history.write(ButtonReadingEvent::new(readings));
                    true
                } else {
                    false
                }
            }
            None => {
                history.write(ButtonReadingEvent::new(readings));
                true
            }
        };

        if changed {
            info!("Buttons changed");

            let action = match history
                .recent()
                .expect("there will certainly be at least one history entry at this point")
                .readings
            {
                PhysicalButtonReadings::FN_1_PRESSED => Some(ButtonAction::Fn1),
                PhysicalButtonReadings::FN_2_PRESSED => Some(ButtonAction::Fn2),
                PhysicalButtonReadings::FN_3_PRESSED => Some(ButtonAction::Fn3),
                PhysicalButtonReadings::END_CONVERSATION_PRESSED => {
                    Some(ButtonAction::EndConversation)
                }
                _ => {
                    warn!("Unexpected button press combination");
                    None
                }
            };
            info!("Button action: {}", action);

            if let Some(action) = action {
                if rpc_sender
                    .publish::<ButtonActionPerformed>(seq.into(), &action)
                    .await
                    .is_err()
                {
                    warn!("Failed to publish button action");
                }
                seq = seq.wrapping_add(1);
            }
        }
    }
}
