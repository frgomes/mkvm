// mkvm Make virtual machine easily!
// Copyright (C) 2022 Richard Gomes <rgomes.info@gmail.com>
// Copyright (C) 2022 Mathminds Ltd <contact@mathminds.io>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the Mathminds Server Side Public License as
// published by the Mathminds Ltd, either version 1 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Mathminds Server Side Public License for more details.
//
// You should have received a copy of the Mathminds Server Side Public
// License along with this program.  If not, see:
// <http://mathminds.io/server-side-public-license/>.

#![allow(unused_parens)]

use anyhow::{Context,Result, anyhow};
use clap::{arg, App, AppSettings};
use std::ffi::OsStr;

fn main() -> Result<()> {
    let matches = App::new("mkvm")
        .about("Make virtual machines easily!")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("vm")
                .about("start virtual machine(s)")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(names: <NAME> ... "Virtual machines to start").allow_invalid_utf8(true))
                .arg(
                    arg!(files: [YAML])
                        .multiple_occurrences(true)
                        .allow_invalid_utf8(true)
                        .last(true),
                ),
        )
        .get_matches();

    let args: Args = validate(&matches, "vm")?;
    //XXX println!("Names: {:?}", args.names);
    //XXX println!("Files: {:?}", args.files);
    let yaml: String = args.render()?;
    //XXX println!("{}", yaml);
    let doc = parse(yaml)?;

    args.names
        .iter()
        .for_each(|name| {
            //XXX let vm: &VM = select(&doc, name.to_str().unwrap()).unwrap();
            //XXX let vm: &VM = doc.vm(name.to_str().unwrap()).unwrap();
            let vm: &VM = doc.vm(name.to_str().unwrap()).unwrap();
            let yaml: String = serde_yaml::to_string(&vm).unwrap();
            println!("{}", yaml);
        });
    Ok(())
}

fn validate<'a>(matches: &'a clap::ArgMatches, _subcommand: &str) -> Result<Args<'a>> {
    let mut stdin_seen  = false;
    let (names, files) = match matches.subcommand() {
        Some((_subcommand, sub_matches)) => {
            let names: Vec<&'a OsStr> = sub_matches
                .values_of_os("names").context("names of virtual machines is requered")?
                .collect::<Vec<_>>();
            let files: Vec<&'a OsStr> = sub_matches
                .values_of_os("files").context("a list of file names is requered")?
                .map(|path| if(path == "-" && stdin_seen) { Err(anyhow!("stdin specified multiple times")) } else { stdin_seen = true; Ok(path) })
                .map(|path| path.unwrap())
                .collect::<Vec<_>>();
            (names, files)
        }
        _ => unreachable!(),
    };
    Ok(Args{ names, files, })
}

fn parse(yaml: String) -> Result<Document> {
    Ok(serde_yaml::from_str(&yaml).context("parse error")?)
}

struct Args<'a> {
    names: Vec<&'a OsStr>,
    files: Vec<&'a OsStr>,
}

trait Reader {
    fn reader(&self) -> Result<String>;
}

impl Reader for &OsStr {
    fn reader(&self) -> Result<String> {
        use std::io::Read;
        let mut buffer = String::new();
        if (*self) == "-" {
            std::io::stdin().read_to_string(&mut buffer)?
        } else {
            std::fs::File::open(*self)?.read_to_string(&mut buffer)?
        };
        Ok(buffer)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

//FIXME: #![recursion_limit = "256"]
use handlebars::Handlebars;
use serde_yaml::Value;

trait Render {
    fn render(&self) -> Result<String>;
}

impl Render for String {
    fn render(&self) -> Result<String> {
        let mut tmpl: String = self.clone();
        let mut data: Value = serde_yaml::from_str(&tmpl)?;
        let handlebars = Handlebars::new();
        loop {
            let rendered = handlebars.render_template(&tmpl, &data).unwrap();
            let exit = rendered == tmpl;
            tmpl = rendered;
            data = serde_yaml::from_str(&tmpl)?;
            if exit { break; }
        }
        Ok(tmpl)
    }
}

impl Render for Vec<&OsStr> {
    fn render(&self) -> Result<String> {
        Ok((*self)
           .iter()
           .map(|path| path.reader().unwrap())
           .fold(String::new(), |mut acc, item| { acc.push_str("\n"); acc.push_str(&item); acc } )
           .render()?)
    }
}

impl<'a> Render for Args<'a> {
    fn render(&self) -> Result<String> {
        Ok((*self).files.render()?)
    }
}


//////////////////////////////////////////////////////////////////////////////////////////////////////////


use serde::{Serialize,Deserialize};
use std::collections::HashMap;

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct Document {
    api: String,
    controller: Controller,
    hypervisor: Hypervisor,
    secrets: HashMap<String,String>,
    vms: Vec<VM>,
}


#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct Controller {
    hostname: String,
    domain: String,
    email: String,
    pubkey: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct Hypervisor {
    hypervisor: String,
    hostname: String,
    network_bridge: String,
    network_mode: String,
    network_name: String,
    pool_name: String,
    storage_format: String,
    url: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct VM {
    hostname: String,
    image: String,
    cpu: String,
    vcpu: u16,
    memory: u64,
    disks: Vec<Disk>,
    networks: Vec<Network>,
    users: Vec<User>,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct Disk {
    device: String,
    label: String,
    #[serde(default = "vec_string")]
    fsopts: Vec<String>,
    fstype: String,
    mountpoint: String,
    #[serde(default = "vec_string")]
    opts: Vec<String>,
    size: u64,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct Network {
    name: String,
    zone: String,
    #[serde(default = "empty_string")]
    subdomain: String,
    qtype: String,
    #[serde(default = "vec_string")]
    dns: Vec<String>,
}

#[derive(Serialize,Deserialize)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
struct User {
    username: String,
    #[serde(default = "empty_string")]
    role: String,
    #[serde(default = "vec_string")]
    authorized_keys: Vec<String>,
}

fn vec_string() -> Vec<String> { Vec::new() }
fn empty_string() -> String { "".to_owned() }


impl Document {
    fn vm(&self, name: &str) -> Result<&VM> {
        (*self).vms
           .iter()
           .filter(|entry| entry.hostname == name)
           .take(1)
           .next().context(anyhow!("could not find hostname: {}", name))
    }
}
