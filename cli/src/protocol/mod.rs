/*
 * Created on Tue Aug 04 2020
 *
 * This file is a part of the source code for the Terrabase database
 * Copyright (c) 2020, Sayan Nandan <ohsayan at outlook dot com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

mod deserializer;
use bytes::{Buf, BytesMut};
use corelib::builders::query::*;
use corelib::de::*;
use corelib::TResult;
use deserializer::{ClientResult, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub async fn new(host: &str) -> TResult<Self> {
        let stream = TcpStream::connect(host).await?;
        Ok(Connection {
            stream,
            buffer: BytesMut::with_capacity(BUF_CAP),
        })
    }
    pub async fn run_query(&mut self, query: String) {
        let mut qbuilder = QueryBuilder::new_simple();
        qbuilder.from_cmd(query);
        match self.stream.write_all(&qbuilder.into_query()).await {
            Ok(_) => (),
            Err(_) => {
                eprintln!("ERROR: Couldn't write data to socket");
                return;
            }
        };
        loop {
            match self.stream.read_buf(&mut self.buffer).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                    return;
                }
            }
            match self.try_response().await {
                ClientResult::Empty(f) => {
                    self.buffer.advance(f);
                    eprintln!("ERROR: The remote end reset the connection");
                    return;
                }
                ClientResult::Incomplete(f) => {
                    self.buffer.advance(f);
                }
                ClientResult::Response(r, f) => {
                    self.buffer.advance(f);
                    if r.len() == 0 {
                        return;
                    }
                    for group in r {
                        println!("{}", group);
                    }
                    return;
                }
                x @ ClientResult::RespCode(_, _) => {
                    match x {
                        ClientResult::RespCode(_, f) => {
                            self.buffer.advance(f);
                        }
                        _ => unimplemented!(),
                    }
                    eprint!("{}", x);
                    return;
                }
                ClientResult::InvalidResponse(_) => {
                    self.buffer.clear();
                    eprintln!("{}", ClientResult::InvalidResponse(0));
                    return;
                }
            }
        }
    }
    async fn try_response(&mut self) -> ClientResult {
        if self.buffer.is_empty() {
            // The connection was possibly reset
            return ClientResult::Empty(0);
        }
        let nav = Navigator::new(&self.buffer);
        Response::from_navigator(nav)
    }
}
