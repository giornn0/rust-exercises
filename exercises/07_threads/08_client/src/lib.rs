use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use core::panic;
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// #[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient {
    client_sender: Sender<Command>,
    client_receiver: Receiver<ServerResponse>,
    server_sender: Sender<ServerResponse>,
}

impl TicketStoreClient {
    fn new(
        client_sender: Sender<Command>,
        client_receiver: Receiver<ServerResponse>,
        server_sender: Sender<ServerResponse>,
    ) -> Self {
        TicketStoreClient {
            client_sender,
            server_sender,
            client_receiver,
        }
    }
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        self.client_sender
            .send(Command::Insert {
                draft,
                response_channel: self.server_sender.clone(),
            })
            .unwrap();
        let response = self.client_receiver.recv().unwrap();
        match response {
            ServerResponse::InsertSuccess(id) => id,
            _ => panic!("Unexpected!"),
        }
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        self.client_sender
            .send(Command::Get {
                id,
                response_channel: self.server_sender.clone(),
            })
            .unwrap();
        match self.client_receiver.recv().unwrap() {
            ServerResponse::GetSuccess(found) => found,
            _ => panic!("Unexpected!"),
        }
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    let (server_sender, receiver_client) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));

    TicketStoreClient::new(sender, receiver_client, server_sender)
}

// No longer public! This becomes an internal detail of the library now.
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<ServerResponse>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<ServerResponse>,
    },
}
#[derive(Clone)]
enum ServerResponse {
    InsertSuccess(TicketId),
    GetSuccess(Option<Ticket>),
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                response_channel
                    .send(ServerResponse::InsertSuccess(id))
                    .unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel
                    .send(ServerResponse::GetSuccess(ticket.cloned()))
                    .unwrap();
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
