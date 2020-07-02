/*
 * Created on Wed Jul 01 2020
 *
 * This file is a part of the source code for the Terrabase database
 * Copyright (c) 2020 Sayan Nandan
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

use std::io;
use std::io::prelude::*;
mod argparse;

const MSG_WELCOME: &'static str = "Terrabase | Version 0.1.0\nCopyright (c) 2020 Sayan Nandan";

fn main() {
    println!("{}", MSG_WELCOME);
    loop {
        let mut buffer = String::new();
        print!("terrabase> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => argparse::EXIT_ERROR("Failed to flush output stream"),
        };
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => argparse::EXIT_ERROR("Failed to read line and append to buffer"),
        };
        let cmds = match argparse::parse_args(buffer) {
            Ok(cmds) => cmds,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        println!("{:#?}", cmds);
    }
}