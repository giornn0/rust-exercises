// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Debug, this)]
pub enum ClientError {
    InvalidResponse,
}

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, ClientError> {
        let (response_channel, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel,
            }).map_err(|_| );
        match response_receiver.recv().unwrap() {
            ServerResponse::SuccessInsert(id) => Ok(id),
            _ => Err(ClientError::InvalidResponse),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, ClientError> {
        let (response_channel, response_receiver) = std::sync::mpsc::sync_channel(1);
        self.sender
            .send(Command::Get {
                id,
                response_channel,
            })
            .unwrap();
        match response_receiver.recv().unwrap() {
            ServerResponse::SuccessGet(found) => Ok(found),
            _ => Err(ClientError::InvalidResponse),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<ServerResponse>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<ServerResponse>,
    },
}
pub enum ServerResponse {
    SuccessInsert(TicketId),
    SuccessGet(Option<Ticket>),
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
                    .send(ServerResponse::SuccessInsert(id))
                    .unwrap();
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                response_channel
                    .send(ServerResponse::SuccessGet(ticket.cloned()))
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
