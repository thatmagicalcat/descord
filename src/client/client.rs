use reqwest::Method;
use std::collections::HashMap;

use std::sync::Mutex;

use json::object;
use nanoserde::SerJson;

use crate::consts::intents::GatewayIntent;
use crate::internals::{EventHandler, *};
use crate::models::interaction::ApplicationCommandOption;
use crate::prelude::{CreateMessageData, Message};
use crate::utils::send_request;
use crate::ws::WsManager;
use crate::{consts, internals, Event};

use log::{error, info};

// SAFETY: These will always be valid if accessed from an event.
lazy_static::lazy_static! {
    pub(crate) static ref BOT_ID: Mutex<Option<String>> = Mutex::new(None);
    pub(crate) static ref TOKEN: Mutex<Option<String>> = Mutex::new(None);
}

pub struct Client {
    intents: u32,
    ws: WsManager,
    token: String,
    commands: HashMap<String, Command>,
    slash_commands: HashMap<String, SlashCommand>,
    event_handlers: HashMap<Event, EventHandler>,
    prefix: String,
}

impl Client {
    pub async fn new(token: &str, intents: impl Into<u32>, prefix: &str) -> Self {
        *TOKEN.lock().unwrap() = Some(token.to_owned());

        Self {
            intents: intents.into(),
            token: token.to_owned(),
            ws: WsManager::new(token)
                .await
                .expect("Failed to initialize websockets"),
            prefix: prefix.to_owned(),

            commands: HashMap::new(),
            slash_commands: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    pub async fn login(self) {
        self.ws
            .connect(
                self.intents,
                self.event_handlers.into(),
                self.commands.into(),
                self.slash_commands.into(),
            )
            .await;
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn register_events(&mut self, events: Vec<EventHandler>) {
        events.into_iter().for_each(|event| {
            if self.event_handlers.contains_key(&event.event) {
                panic!("{:?} is already hooked", event.event);
            }

            self.event_handlers.insert(event.event, event);
        });
    }

    pub fn register_commands(&mut self, commands: Vec<Command>) {
        commands.into_iter().for_each(|mut command| {
            // if a custom prefix is not applied, add the default prefix
            if !command.custom_prefix {
                command.name = format!(
                    "{default_prefix}{name}",
                    default_prefix = self.prefix,
                    name = command.name
                );
            }

            self.commands.insert(command.name.clone(), command.clone());
        });
    }

    pub async fn register_slash_commands(&mut self, commands: Vec<SlashCommand>) {
        fn map_param_type_to_u8(param_type: &ParamType) -> u8 {
            match param_type {
                ParamType::String => 3,
                ParamType::Int => 4,
                ParamType::User => 6,
                ParamType::Channel => 7,
                _ => 3,
            }
        }

        let response = send_request(Method::GET, "users/@me", None).await;
        let bot_id =
            json::parse(response.unwrap().text().await.unwrap().as_str()).unwrap_or_else(|_| {
                error!("Failed to parse JSON response");
                json::JsonValue::Null
            })["id"]
                .as_str()
                .unwrap_or_else(|| {
                    error!("Failed to get 'id' from JSON response");
                    ""
                })
                .to_string();

        let registered_commands = json::parse(
            &send_request(
                Method::GET,
                format!("applications/{}/commands", bot_id).as_str(),
                None,
            )
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
        )
        .expect("Failed to parse JSON response");

        // Iterate over the local commands
        for local_command in &commands {
            let options = local_command
                .fn_param_names
                .iter()
                .zip(local_command.fn_param_renames.iter())
                .zip(local_command.fn_sig.iter())
                .zip(local_command.fn_param_descriptions.iter())
                .map(|(((name, rename), type_), description)| {
                    let name = rename.as_ref().unwrap_or_else(|| name);
                    json::object! {
                        name: name.clone(),
                        description: description.clone(),
                        type: map_param_type_to_u8(type_),
                        required: true,
                    }
                })
                .collect::<Vec<_>>();

            // If the command exists in the fetched commands
            if let Some(registered_command) = registered_commands
                .members()
                .find(|&cmd| cmd["name"].as_str().unwrap_or("") == local_command.name)
            {
                let registered_options = registered_command["options"].members();
                let registered_names: Vec<_> = registered_options
                    .clone()
                    .map(|opt| opt["name"].as_str().unwrap_or(""))
                    .collect();
                let registered_descriptions: Vec<_> = registered_options
                    .clone()
                    .map(|opt| opt["description"].as_str().unwrap_or(""))
                    .collect();
                let registered_types: Vec<_> = registered_options
                    .map(|opt| opt["type"].as_u8().unwrap_or(0))
                    .collect();

                let fn_param_names = local_command
                    .fn_param_names
                    .iter()
                    .zip(local_command.fn_param_renames.iter())
                    .map(|(name, rename)| rename.as_ref().unwrap_or(name))
                    .collect::<Vec<_>>();

                if local_command.description
                    != registered_command["description"].as_str().unwrap_or("")
                    || fn_param_names != registered_names
                    || local_command.fn_param_descriptions != registered_descriptions
                    || local_command
                        .fn_sig
                        .iter()
                        .map(map_param_type_to_u8)
                        .collect::<Vec<_>>()
                        != registered_types
                {
                    let response = send_request(
                        Method::PATCH,
                        format!(
                            "applications/{}/commands/{}",
                            bot_id, registered_command["id"]
                        )
                        .as_str(),
                        Some(json::object! {
                            name: local_command.name.clone(),
                            description: local_command.description.clone(),
                            options: options,
                        }),
                    )
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                    info!(
                        "Updated '{}' slash command, command id: {}",
                        local_command.name,
                        registered_command["id"].as_str().unwrap_or("").to_string()
                    );
                } else {
                    info!(
                        "No changes detected in '{}' slash command, command id: {}",
                        local_command.name,
                        registered_command["id"].as_str().unwrap_or("").to_string()
                    );
                    self.slash_commands.insert(
                        registered_command["id"].as_str().unwrap_or("").to_string(),
                        local_command.clone(),
                    );
                }
            } else {
                // If the command does not exist in the fetched commands, register it
                let response = send_request(
                    Method::POST,
                    format!("applications/{}/commands", bot_id).as_str(),
                    Some(json::object! {
                        name: local_command.name.clone(),
                        description: local_command.description.clone(),
                        options: options,
                    }),
                )
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

                let command_id = json::parse(&response).expect("Failed to parse JSON response")
                    ["id"]
                    .as_str()
                    .expect("Failed to get 'id' from JSON response")
                    .to_string();

                info!(
                    "Registered '{}' slash command, command id: {}",
                    local_command.name, command_id
                );

                self.slash_commands
                    .insert(command_id, local_command.clone());
            }
        }

        // Iterate over the fetched commands
        for registered_command in registered_commands.members() {
            // If the command does not exist in the local commands, remove it
            if commands
                .iter()
                .find(|&cmd| cmd.name == registered_command["name"].as_str().unwrap_or(""))
                .is_none()
            {
                send_request(
                    Method::DELETE,
                    format!(
                        "applications/{}/commands/{}",
                        bot_id,
                        registered_command["id"].as_str().unwrap_or("")
                    )
                    .as_str(),
                    None,
                )
                .await
                .unwrap();

                info!(
                    "Removed slash command '{}', command id: {}",
                    registered_command["name"].as_str().unwrap_or(""),
                    registered_command["id"].as_str().unwrap_or("")
                );
            }
        }
    }
}
