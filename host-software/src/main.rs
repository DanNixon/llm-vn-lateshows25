mod character;
mod controller;
mod conversation;
mod printer;

use character::{Character, CharacterCollection};
use clap::Parser;
use conversation::{Conversation, ConversationClient};
use escpos::driver::{Driver, SerialPortDriver};
use icd::{ButtonAction, CharacterSelectScreen};
use log::{debug, info, warn};
use ollama_rs::Ollama;
use printer::Printer;
use std::{path::PathBuf, time::Duration};

#[derive(Debug, Parser)]
struct Cli {
    /// Serial port the thermal printer is attached to
    #[arg(long, env)]
    printer_serial_port: String,

    /// Serial baud rate to use when communicating with the thermal printer
    #[arg(long, env, default_value = "38400")]
    printer_baud: u32,

    /// Ollama server host
    #[arg(long, env, default_value = "http://localhost")]
    ollama_host: String,

    /// Ollama server port
    #[arg(long, env, default_value = "11434")]
    ollama_port: u16,

    /// File containing character definitions
    #[arg(long, env)]
    character_file: PathBuf,

    /// Directory in which to save ended conversations
    #[arg(long, env)]
    conversation_directory: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    env_logger::init();

    let mut printer = Printer::new(
        SerialPortDriver::open(
            &args.printer_serial_port,
            args.printer_baud,
            Some(Duration::from_secs(5)),
        )
        .unwrap(),
    );

    let controller = controller::Client::new();

    // Ping controller on start up, just because I suppose...
    {
        let req = 42;
        let resp = controller.ping(req).await;
        assert!(resp == req, "Controller ping failed");
    }

    let ollama = Ollama::new(args.ollama_host, args.ollama_port);

    let models = tokio::time::timeout(Duration::from_secs(3), ollama.list_local_models())
        .await
        .expect("Ollama list model call should return quickly")
        .expect("Should be able to list Ollama models");
    info!("Available Ollama models: {models:?}");
    if models.is_empty() {
        panic!("No Ollama models loaded, this is certainly not intended");
    }

    let characters = CharacterCollection::load(&args.character_file);

    let model_names: Vec<&str> = models.iter().map(|m| m.name.as_str()).collect();
    printer
        .print_ready(&characters.characters, &model_names)
        .unwrap();

    loop {
        let character = select_character(&controller, &characters).await;

        let conversation = converse(&mut printer, &ollama, &controller, character).await;
        info!("Conversation ended: {conversation:#?}");

        if let Err(e) = conversation.save_in(&args.conversation_directory) {
            warn!("Failed to save conversation: {e}");
        }
    }
}

async fn select_character(
    controller: &controller::Client,
    characters: &CharacterCollection,
) -> Character {
    let mut selected_idx = 0;

    'character_select: loop {
        debug!("selected_idx = {selected_idx}");

        let charas = characters.pick_subset(selected_idx);
        controller
            .show_character_select_screen(CharacterSelectScreen {
                prev: charas[0].clone().into(),
                selected: charas[1].clone().into(),
                next: charas[2].clone().into(),
            })
            .await;

        let button = controller.wait_for_button_push().await;

        match button {
            ButtonAction::Fn1 => {
                debug!("Previous pressed");
                if selected_idx == 0 {
                    selected_idx = characters.characters.len() - 1;
                } else {
                    selected_idx = selected_idx.saturating_sub(1);
                }
            }
            ButtonAction::Fn2 => {
                break 'character_select;
            }
            ButtonAction::Fn3 => {
                debug!("Next pressed");
                selected_idx = selected_idx.saturating_add(1);
                if selected_idx == characters.characters.len() {
                    selected_idx = 0;
                }
            }
            ButtonAction::EndConversation => {}
        }
    }

    let chara = characters.characters[selected_idx].clone();
    info!("Selected character: {chara:?}");

    chara
}

async fn converse<D: Driver>(
    printer: &mut Printer<D>,
    ollama: &Ollama,
    controller: &controller::Client,
    character: Character,
) -> Conversation {
    let mut conversation = ConversationClient::new(ollama, character.clone());
    printer.print_chat_header(conversation.character()).unwrap();

    let mut vn_out = character.starting_phrases();

    'conversation: loop {
        controller
            .show_choice_screen(character.choice_screen(&vn_out))
            .await;

        const BUTTON_TIMEOUT: Duration = Duration::from_secs(60);
        let button = tokio::time::timeout(BUTTON_TIMEOUT, controller.wait_for_button_push())
            .await
            .unwrap_or_else(|_| {
                info!("No button pressed in {BUTTON_TIMEOUT:?}, ending conversation");
                ButtonAction::EndConversation
            });

        let user_text = match button {
            ButtonAction::Fn1 => vn_out.user_reply_1,
            ButtonAction::Fn2 => vn_out.user_reply_2,
            ButtonAction::Fn3 => vn_out.user_reply_3,
            ButtonAction::EndConversation => {
                break 'conversation;
            }
        };

        printer.print_user_message(&user_text).unwrap();

        vn_out = conversation.interact(user_text).await;

        printer
            .print_character_message(conversation.character(), &vn_out.response)
            .unwrap();

        if vn_out.is_end_of_conversation() {
            break 'conversation;
        }
    }

    printer.print_chat_footer().unwrap();

    conversation.conversation()
}
