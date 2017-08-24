// Copyright 2017 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! cmake wrapper for build

extern crate cmake;
extern crate fs_extra;

mod sanity;

use cmake::Config;
use std::{env,fs};
use std::path::{PathBuf};
use fs_extra::dir::*;
use sanity::Finder;

/// Tests whether source cuckoo directory exists

pub fn fail_on_empty_directory(name: &str){
	if fs::read_dir(name).unwrap().count()==0 {
		println!("The `{}` directory is empty. Did you forget to pull the submodules?", name);
		println!("Try `git submodule update --init --recursive`");
		panic!();
	}
}

fn main() {
	let mut command_finder=Finder::new();
	//dumb and quick test for windows, can parse later
	let windows_sysinfo = command_finder.maybe_have("systeminfo");
	if let Some(_) = windows_sysinfo {
		//Windows plugins not supported for now.. bye!
		return;
	}	


	fail_on_empty_directory("plugins/cuckoo");
	let path_str = env::var("OUT_DIR").unwrap();
	let mut out_path = PathBuf::from(&path_str);
	out_path.pop();
	out_path.pop();
	out_path.pop();
	let mut plugin_path = PathBuf::from(&path_str);
	plugin_path.push("build");
	plugin_path.push("plugins");
	//Collect the files and directories we care about
	let dir_content = get_dir_content("plugins").unwrap();
	for d in dir_content.directories {
		let file_content = get_dir_content(d).unwrap();
		for f in file_content.files {
			println!("cargo:rerun-if-changed={}",f);
		}
	}
	for f in dir_content.files{
		println!("cargo:rerun-if-changed={}",f);
	}
	//panic!("marp");
	let dst = Config::new("plugins")
	                      //.define("FOO","BAR") //whatever flags go here
	                      //.cflag("-foo") //and here
	                      .build_target("")
	                      .build();
	
	
	println!("Plugin path: {:?}", plugin_path);
	println!("OUT PATH: {:?}", out_path);
	let mut options = CopyOptions::new();
	options.overwrite=true;
	if let Err(e) = copy(plugin_path, out_path, &options) {
		println!("{:?}", e);
	}

	println!("cargo:rustc-link-search=native={}", dst.display());

}