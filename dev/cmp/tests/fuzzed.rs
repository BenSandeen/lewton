// Vorbis decoder written in Rust
//
// Copyright (c) 2018 est31 <MTest31@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

extern crate test_assets;
#[macro_use]
extern crate cmp;
extern crate lewton;

macro_rules! try {
	($expr:expr) => (match $expr {
		$crate::std::result::Result::Ok(val) => val,
		$crate::std::result::Result::Err(err) => {
			panic!("Error: {:?}", err)
		}
	})
}

#[test]
fn test_malformed_fuzzed() {
	println!();
	test_assets::download_test_files(&cmp::get_fuzzed_asset_defs(),
		"test-assets", true).unwrap();
	println!();

	use lewton::VorbisError::*;
	use lewton::audio::AudioReadError::*;
	use lewton::header::HeaderReadError::*;

	ensure_malformed!("27_really_minimized_testcase_crcfix.ogg", BadAudio(AudioBadFormat));
	ensure_malformed!("32_minimized_crash_testcase.ogg", BadHeader(HeaderBadFormat));
}

// Ensures that a file is okay
macro_rules! ensure_okay {
	($name:expr) => {{
		use std::fs::File;
		use lewton::inside_ogg::OggStreamReader;
		// Read the file to memory
		let f = try!(File::open(format!("test-assets/{}", $name)));
		if let Some(mut ogg_rdr) = try!(OggStreamReader::new(f).map(|v| Some(v))) {
			loop {
				match try!(ogg_rdr.read_dec_packet_itl()) {
					Some(_) => (),
					None => break,
				};
			}
		}
	}}
}

#[test]
fn test_okay_fuzzed() {
	println!();
	test_assets::download_test_files(&cmp::get_fuzzed_asset_defs(),
		"test-assets", true).unwrap();
	println!();

	ensure_okay!("33_minimized_panic_testcase.ogg");
}