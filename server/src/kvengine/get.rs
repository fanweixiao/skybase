/*
 * Created on Fri Aug 14 2020
 *
 * This file is a part of Skybase
 * Skybase (formerly known as TerrabaseDB) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2020, Sayan Nandan <ohsayan@outlook.com>
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

//! # `GET` queries
//! This module provides functions to work with `GET` queries

use crate::coredb::CoreDB;
use crate::dbnet::Con;
use crate::protocol::{responses, ActionGroup};
use crate::resp::{BytesWrapper, GroupBegin};
use bytes::Bytes;
use libsky::TResult;

/// Run a `GET` query
pub async fn get(handle: &CoreDB, con: &mut Con<'_>, act: ActionGroup) -> TResult<()> {
    let howmany = act.howmany();
    if howmany != 1 {
        return con.write_response(&**responses::fresp::R_ACTION_ERR).await;
    }
    // Write #<m>\n#<n>\n&1\n to the stream
    con.write_response(GroupBegin(1)).await?;
    let res: Option<Bytes> = {
        let rhandle = handle.acquire_read();
        let reader = rhandle.get_ref();
        unsafe {
            // UNSAFE(@ohsayan): act.get_ref().get_unchecked() is safe because we've already if the action
            // group contains one argument (excluding the action itself)
            reader
                .get(act.get_ref().get_unchecked(1))
                .map(|b| b.get_blob().clone())
        }
    };
    if let Some(value) = res {
        // Good, we got the value, write it off to the stream
        con.write_response(BytesWrapper(value)).await?;
    } else {
        // Ah, couldn't find that key
        con.write_response(&**responses::groups::NIL).await?;
    }
    Ok(())
}
