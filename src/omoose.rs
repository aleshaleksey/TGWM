//A crate for playback of music during various stages of mq.
//Currently uses a very basic playback function from rodio examples.
//based on the rodio example.
//I haven't done anything here.
#![allow(unused_imports)]
///
///TGWM:omoose
///
///The omoose library is a small library that contains the Q-ft-M
///music player, and will eventually contain the sound effect players.
///This library depends on gmoose (and therefore everything else).
///
///~Alek Zholobenko
///
//extern crate cpal;
//extern crate hound;
extern crate libc;
extern crate rodio;
extern crate find_folder;
extern crate std;

use gmoose;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader,Read};
use std::sync::mpsc::Receiver;
use std::time::Duration;


//NB experimental NB isekai deguchi requires a sender to be active.
//Opens and plays a song. If isekai_deguchi tells us to stop or buffer is empty
//The function returns true if it needs to play again and false if not.
pub fn play_song_rod(gone:(bool,usize),
					 postman:&mut Receiver<(bool,usize)>,
					 silent_postman:&mut Receiver<bool>)->(bool,usize) {
	let mut go = gone;
	let mut silence = false;
	let device = match rodio::default_output_device() {
		Some(dev) => {dev},
		None	  => {return go},
	};
    let sink = rodio::Sink::new(&device);
    
    //assign songs vector.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("as").unwrap();
        
    //make vector of songs
    let mut songs:Vec<String> = Vec::with_capacity(18);
    for _ in 0..19 {songs.push(assets.join("notes/a.wav").to_str().unwrap().to_owned());}
    
    //insert built in terrain-specific themes
    songs[1] = assets.join("notes/rat.wav").to_str().unwrap().to_owned();
    songs[2] = assets.join("notes/grim.wav").to_str().unwrap().to_owned();
    songs[7] = assets.join("notes/ic.wav").to_str().unwrap().to_owned();
    songs[8] = assets.join("notes/tu.wav").to_str().unwrap().to_owned();
    songs[10] = assets.join("notes/pl.wav").to_str().unwrap().to_owned();
    songs[11] = assets.join("notes/fo.wav").to_str().unwrap().to_owned();
    songs[12] = assets.join("notes/st.wav").to_str().unwrap().to_owned();
    songs[13] = assets.join("notes/de.wav").to_str().unwrap().to_owned();
    songs[14] = assets.join("notes/to.wav").to_str().unwrap().to_owned();
    songs[16] = assets.join("notes/mo.wav").to_str().unwrap().to_owned();
    songs[18] = assets.join("notes/ru.wav").to_str().unwrap().to_owned();
    
    //Read dukebox config file
    gmoose::parse_music_config(&mut songs);
	
	for (i,x) in songs.iter().enumerate() {println!("{}: {}",i,x);}

	//loop the music until told to stop.
	while go.0 & !silence {
		let file = match std::fs::File::open(&songs[go.1]) {
			Ok(f) => {f},
			_	  => {return go},
		};
		sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
		
		while go.0 & !sink.empty() & !silence {
			std::thread::sleep(Duration::from_millis(2000));
			go = gmoose::isekai_deguchi(go.clone(),postman);
			silence = gmoose::isekai_urusai(silence,silent_postman);
		};
		if !go.0 | silence {
			sink.stop();
			drop(sink);
			return go
		};
		
		sink.sleep_until_end();
	}
    drop(sink);
    (true,go.1)
}

//The actual module ends here.

////NB uses cpal 0.8 and does not currently work properly. 
////The event stream cannot be dropped, the device cannot be deactivated.
////Thy cluster is ravished.
//#[allow(non_snake_case)]
//pub fn play_song(P:&Path,){
	//let reader = hound::WavReader::open(&P).unwrap();
	//let spec = reader.spec();
	
	//let device = cpal::default_output_device().expect("Failed to get default output device");
    //let supformat:cpal::SupportedFormat = device.supported_input_formats().unwrap()
											  //.filter(|f| matches_format(f, &spec))
											  //.next()
											  //.expect("no supported playback format");
    //let format = cpal::Format
    //{
		//channels: supformat.channels,
		//sample_rate: cpal::SampleRate(spec.sample_rate*spec.channels as u32),
		//data_type: supformat.data_type,
	//};
    
    //println!("opened {:?}",P);
	
    //let event_loop = cpal::EventLoop::new();
    //let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    //event_loop.play_stream(stream_id.clone());
	
	//let r_samples = reader.into_samples();
	//let mut vc = Vec::with_capacity(1000000);
	//for x in r_samples {
		//vc.push(x);
	//};
	//let mut len:usize = vc.len();
	//let mut vci = vc.into_iter();
	//let _started = false;
	//let _stream_id2 = stream_id.clone();
	//let len2:usize = len;
	
	//let _t0 = std::thread::spawn(move||{
		//event_loop.run(move |stream_id2, data| {
			
				//if len<1 {
					//drop(stream_id2);
				//}else{
					//match data {
						//cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
							//for sample in buffer.chunks_mut(format.channels as usize).take(len) {
								//len-= 1;
								//let value = vci.next();
								//if value.is_none() {
									//len = 0;
								//}else{
									//let valu = value.unwrap().unwrap_or_default();
									//for out in sample.iter_mut() {
										//*out = valu;
									//};
								//};
							//};
							//drop(buffer);
						//},
						//cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
							//for sample in buffer.chunks_mut(format.channels as usize).take(len) {
								//len-= 1;
								//let value = vci.next();
								//if value.is_none() {
									//len = 0;
								//}else{
									//let valu = value.unwrap().unwrap_or_default() as f32;;
									//for out in sample.iter_mut() {
										//*out = valu;
									//};
								//};
							//};
							//drop(buffer);
						//},
						//_ => {  len = 0;},
					//};
				//}
		//});
	//});
	//std::thread::sleep(std::time::Duration::from_millis(1000*len2 as u64/(spec.sample_rate*spec.channels as u32) as u64));

	//println!("Exiting player function");
//}

////auxillary function from cpal-hound example for the cpal function, does the job.
//fn matches_format(format: &cpal::SupportedFormat, spec: &hound::WavSpec) -> bool {
    //if (format.min_sample_rate > cpal::SampleRate(spec.sample_rate)) 
     //& (format.max_sample_rate < cpal::SampleRate(spec.sample_rate)) {
        //return false
    //}

    //if format.channels != spec.channels {
        //return false
    //}

    //let data_type = match (spec.bits_per_sample, spec.sample_format) {
        //(16, hound::SampleFormat::Int) => Some(cpal::SampleFormat::I16),
        //(32, hound::SampleFormat::Float) => Some(cpal::SampleFormat::F32),
        //_ => None
    //};

    //if Some(format.data_type) != data_type {
        //return false
    //}

    //// If the sample rate, channel count, and sample format match, then we can
    //// play back the file in this format.
    //true
//}

