/// shared_moose.rs contains general and shared functions which are
/// shared between modules, and certain inner game logic not related to
/// battle.
extern crate conrod;
extern crate rand;

use std::fs::File;
use std::env;
use std::io::{Read,Write};
use std::mem::{forget,transmute};
use rand::Rng;

use smoose::{MyStories,MyDungeons,KillList,Content,Story};
use lmoose::{Lifeform,Spell,Place};
use lmoose::*;


//Basic functions which will be put elsewhere afterwards:
//A bunch of unnecessary functions from I didn't know what I was doing.
//Some of them are actually useful.
//does vector contain b?
pub fn lhas<T:PartialOrd>(a:&Vec<T>, b:&T)->bool{
	let mut ihaz=false;
	for x in a.iter(){
		if x==b{
			ihaz=true;
			return ihaz}
		else{continue}
	}
	ihaz
}

pub fn vnmin<T:PartialOrd+Copy>(a:Vec<T>)->T{
	let mut min:T=a[0];
	for i in 0..a.len(){
		if min<=a[i]{min=min}
		else{min=a[i]};
	};
	min
}

pub fn vnmax<T:PartialOrd+Copy>(a:Vec<T>)->T{
	let mut max:T=a[0];
	for i in 0..a.len(){
		if max<=a[i]{max=a[i]}
		else{max=max};
	};
	max
}
//vwhich returns index of the first value in vector a which equals b.
pub fn vwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Option<usize>{
	for i in 0..a.len(){
		if a[i]==b{return Some(i)}
		else{continue}
	}
	None
}

//vwhich returns index of the first value in vector a which equals b.
pub fn vwhich_or<T:PartialOrd+Copy>(a:&Vec<T>, b:T, c:usize)->usize{
	for i in 0..a.len(){
		if a[i]==b{return i}
		else{continue}
	}
	c
}

//vvwhichinv returns indexs of the values in vector a which do not equal b.
pub fn vvwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//vvwhichinv returns indexs of the values in vector a which do not equal b.
pub fn vvwhichinv<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]!=b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//special vvwhichfor lifeform names
pub fn vvwhich_ln(a:&Vec<Lifeform>, b:&str)->Vec<usize>{
	let mut ivec:Vec<usize> = Vec::new();
	for i in 0..a.len(){
		if a[i].name==b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//special vvwhichfor lifeform ids
pub fn vwhich_ln_i(a:&Vec<Lifeform>, b:usize)->Option<usize>{

	for i in 0..a.len(){
		if a[i].id==b {return Some(i);};
	}
	None
}

pub fn uniq<T:Copy + PartialOrd>(mvec:&Vec<T>)->Vec<T>{
	let mut uniq:Vec<T>=Vec::new();
	for x in mvec.iter(){
		if uniq.len()==0{
			uniq.push(*x)
		}else if lhas(&uniq,x)==false{
			uniq.push(*x)
		}else{};
	};
	uniq
}

pub fn uniq_m<T:PartialOrd>(mvec:Vec<T>)->Vec<T>{
	let mut uniq:Vec<T>=Vec::new();
	for x in mvec.into_iter(){
		if uniq.len()==0{
			uniq.push(x)
		}else if lhas(&uniq,&x)==false{
			uniq.push(x)
		}else{};
	};
	uniq
}

//Auxillary functions for saving and loading games.
//Mainly transmutations.
pub fn by_f64(x:f64)->u64{
	let y=unsafe{transmute::<f64,u64>(x)};
	y
}
pub fn byteri32(x:i32)->[u8;4]{
    let y= unsafe {transmute(x.to_be())};
    y
}
pub fn byterus(x:usize)->[u8;8]{
    let y= unsafe {transmute(x.to_be())};
    y
}
pub fn byter64(x:u64)->[u8;8]{
    let y= unsafe {transmute(x.to_be())};
    y
}
pub fn unbyteus(x:[u8;8])->usize{
    let mut out:u64=0;
    for i in 0..8{out+=(x[7-i] as u64)*256u64.pow(i as u32)};
    let out:usize=out as usize;
    out
}
pub fn unbyte64(x:[u8;8])->u64 {
    let mut out:u64=0;
    for i in 0..8{out+=(x[7-i] as u64)*256u64.pow(i as u32)};
    let out:u64=out as u64;
    out
}
pub fn unbyte32(x:[u8;4])->i32 {
    let mut out:u32=0;
    for i in 0..4{out+=(x[3-i] as u32)*256u32.pow(i as u32)};
    let out:i32=unsafe {transmute::<u32,i32>(out)};
    out
}
pub fn un64_f64(x:u64)->f64{
	let y=unsafe{transmute::<u64,f64>(x)};
	y
}
pub fn byteru16(x:u16)->[u8;2]{
    let y= unsafe {transmute(x.to_be())};
    y
}

//Various spell indexing related functions.
pub fn spell_targets_to_indices(to_hit:&Vec<(bool,bool)>,targets:&mut Vec<usize>){
	*targets = Vec::with_capacity(25);
	for (i,x) in to_hit.iter().enumerate() {
		if x.1 | x.0 {targets.push(i);};
	}
}

pub fn arcana_index_from_spell_id(spell_list: &Vec<Spell>, id: i8) ->Option<usize> {	
	for i in 0..spell_list.len(){
		if spell_list[i].id==id {return Some(i)}
	}
	None
}

pub fn arcana_index_from_spell_name(spell_list: &Vec<Spell>, name: &str) ->Option<usize> {	
	for i in 0..spell_list.len(){
		if spell_list[i].name==name {return Some(i)}
	}
	None
}

pub fn arcana_type_from_spell_id<'a> (spell_list: &'a Vec<Spell>, id: i8) ->Option<u8> {	
	for x in spell_list{
		if x.id==id {return Some(x.Type)}
	}
	None
}

//A little unsafe
pub fn arcana_name_from_spell_id<'a> (spell_list: &'a Vec<Spell>, id: i8) -> String {	
	for x in spell_list{
		if x.id==id {return x.name.to_owned()}
	}
	String::new()
}

//gmoose::beast_name(encounter[ifast],p_names)

pub fn beast_name<'a>(encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ifast:usize,p_names:&'a Vec<String>)->&'a str {
	if (encounter[ifast].1==0) & (ifast<p_names.len()) {
		&p_names[ifast]
	}else{
		encounter[ifast].0.name
	}
}

//NB xx must be encounter and not party
pub fn exp_calc(xx: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, i:usize)->f32{
	let mut expgain:f32 = 0.0;
	let kachi = xx[i].0.HP*(xx[i].0.Attack+xx[i].0.Defence)
		+xx[i].0.MP*(xx[i].0.BM+xx[i].0.WM)
		+xx[i].0.Speed*(xx[i].0.HP+xx[i].0.MP);
	
	//Baxter to the soap factory.
	for i in xx.iter(){
		let baxter = i.0.HP*(i.0.Attack+i.0.Defence)
		+i.0.MP*(i.0.BM+i.0.WM)
		+i.0.Speed*(i.0.HP+i.0.MP);
		expgain+= baxter/kachi;
	};
	expgain-= 2.0;
	expgain = if expgain<0.0 {0.0}else{expgain};
	expgain
}

//Part of load game function.
fn place_loader(w:&Vec<[Place;19]>, coords:[i32;2])->(usize,usize){
	let mut answer:(usize,usize)=(10,8);
	for i in 0..w.len(){
		for j in 0..w[i].len(){
			if w[i][j].xy==coords{
				answer=(i,j);
				return answer
			}else{};
		};
	};
	answer
}

//state match return string.
pub fn sm_rets(x:&Lifeform)->String{
	let stater:f32 = x.HP_shade/x.HP;
	let state:i32 =
		if stater>= 1.0{5}
		else if (stater<1.0) & (stater>=0.75){4}
		else if (stater<0.75) & (stater>=0.5){3}
		else if (stater<0.5) & (stater>=0.25){2}
		else if(stater<0.25) & (stater>0.0){1}
		else{0};
	match state {
		5 => "uninjured".to_owned(),
		4 => "lightly wounded".to_owned(),
		3 => "wounded".to_owned(),
		2 => "gravely wounded".to_owned(),
		1 => "on death's door".to_owned(),
		0 => "dead".to_owned(),
		_ => "absent".to_owned(),
	}
}

fn load_marker(){}
//rewritten load function.
//NB has some "illogical" stuff here to preserve backwards compatibility
pub fn load<'a,'b>( file_name:String, spl:&Vec<Spell>, world:&Vec<[Place;19]>, mons:&Vec<Lifeform>,
				mut party:&mut Vec<(Lifeform,usize)>,
				mut p_names:&mut Vec<String>,
				mut p_loc:&mut Place,
				mut pl:&mut (usize,usize),
				mut coords: &mut [i32;2],
				my_stories:&mut MyStories,
				my_dungeons:&mut MyDungeons,
				my_kills:&mut KillList,){	
	println!("filename: {}",file_name);
	//Initiate raw data constructs.
	let mut rlb = Vec::with_capacity(8000);
	let mut ltxt:Vec<String> = Vec::new();
	let mut rltxt = String::new();
	let to_open_a = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrb");
	let to_open_b = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrtxt");
	let to_open_s = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrp");
	let to_open_d = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrd");
	let to_open_k = env::current_dir().unwrap().join("as/saves").join(file_name+".msqrk");

	println!("msqrb: {:?}",to_open_a);
	println!("msqrtxt: {:?}",to_open_b);
	println!("msqrp: {:?}",to_open_s);
	println!("msqrd: {:?}",to_open_d);
	println!("msqrk: {:?}",to_open_k);
	//open files and read into raw data constructs.
	let mut loadb= File::open(to_open_a).unwrap();
	let mut loadtxt= File::open(to_open_b).unwrap();
	loadb.read_to_end(&mut rlb);
	loadtxt.read_to_string(&mut rltxt);
	
	//NB old files will not have a plot file initially, so for compat.
	//this is written to taken that into account.
	match File::open(to_open_s) {
		Ok(mut fp) => {
			println!("There is a plot");
			let mut rplot:Vec<u8> = Vec::with_capacity(50000);
			fp.read_to_end(&mut rplot);
			let plot_len = rplot.len()/8; //Assumes vec<(u32,u16,u16)>, hence 6 bytes,
			let plot_pointer = unsafe {transmute::<*mut u8,*mut (u32,u16,u16)>(rplot.as_mut_ptr())};
			forget(rplot);
			let rplottrans = unsafe {
				Vec::from_raw_parts(
					plot_pointer,
					plot_len,
					plot_len,
				)
			};
			my_stories.make_ids(rplottrans);
		},
		_		=> {
			println!("There is no plot.");
			*my_stories = MyStories::new();
		},
	};
	
	//NB old files will not have a plot file initially, so for compat.
	//this is written to taken that into account.
	match File::open(to_open_d) {
		Ok(mut fp) => {
			println!("There are dungeons");
			let mut packed_dungeons:Vec<u8> = Vec::with_capacity(2000);
			fp.read_to_end(&mut packed_dungeons);
			let dungeons_len = packed_dungeons.len()/16; //Assumes vec<[u32;4]>, hence 4*4=16 bytes,
			let dungeons_pointer = unsafe {transmute::<*mut u8,*mut [u32;4]>(packed_dungeons.as_mut_ptr())};
			forget(packed_dungeons);
			let dungeon_ids = unsafe {
				Vec::from_raw_parts(
					dungeons_pointer,
					dungeons_len,
					dungeons_len,
				)
			};
			my_dungeons.replace_ids(dungeon_ids);
		},
		_		=> {
			println!("There are no dungeons.");
			*my_dungeons = MyDungeons::new();
		},
	};

	let mut kill_n_vector:Vec<u64> = Vec::new();
	
	let kill_n:usize = match File::open(to_open_k) {
		Ok(mut fp) => {
			println!("There are kills");
			let mut packed_kills:Vec<u8> = Vec::with_capacity(2000);
			fp.read_to_end(&mut packed_kills);
			let kills_len = packed_kills.len()/8; //Assumes vec<u64>, hence 8 bytes per entry,
			let kills_pointer = unsafe {transmute::<*mut u8,*mut u64>(packed_kills.as_mut_ptr())};
			forget(packed_kills);
			let mut kills_counts = unsafe {
				Vec::from_raw_parts(
					kills_pointer,
					kills_len,
					kills_len,
				)
			};
			let count =  if kills_counts.len()>0 {kills_counts.remove(0) as usize}else{0};
			kill_n_vector = kills_counts;
			count
		},
		_		=> {
			println!("There are no kills.");
			0},
	};
	println!("Got here");
	// NB old files will not have a plot,dungeon,quest or killist files initially
	// so for compatibility this is written to taken that into account.
	// reformat text file from &str to String.
	let rrltxt:Vec<&str> = rltxt.split("\n").collect();
	for i in 0..rrltxt.len(){ltxt.push(rrltxt[i].to_owned())};
		
	//reset party, party location and party names.
	*p_names = Vec::with_capacity(5);
	*party = Vec::with_capacity(5);		
	*coords = [0,0];
	
    let mut indtrack:usize=0;
//reconstitute party number u8->u64
	let p_no=(rlb.remove(0)) as usize;
	for _ in 0..p_no{		
//reconstitute Exp.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let exp=un64_f64(unbyte64(temp)) as f32;
//reconstitute ExpUsed.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let expu=un64_f64(unbyte64(temp)) as f32;
//reconstitute MP.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let mp=un64_f64(unbyte64(temp)) as f32;		
//reconstitute HP.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let hp=un64_f64(unbyte64(temp)) as f32;
//reconstitute speed.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let speed=un64_f64(unbyte64(temp)) as f32;
//reconstitute attack.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let attack=un64_f64(unbyte64(temp)) as f32;		
//reconstitute defence.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let defence=un64_f64(unbyte64(temp)) as f32;			
//reconstitute wm.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let wm=un64_f64(unbyte64(temp)) as f32;	
//reconstitute bm.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let bm=un64_f64(unbyte64(temp)) as f32;	
//reconstitute spell list length.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let sp_len=unbyteus(temp);
		println!("{}",sp_len);
//reconstitute inventory length.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let inv_len=unbyteus(temp);		
//reconstitute Alive? Unclean?
		let temp=rlb.remove(0);
		let alive=if temp==0{false}else{true};
		let temp=rlb.remove(0);
		let unclean=if temp==0{false}else{true};
//reconstitute longitude
		let mut temp:[u8;4]=[0;4];
		for i in 0..4{temp[i]=rlb.remove(0)};
		let lox=unbyte32(temp);	
//reconstitute latitude
		let mut temp:[u8;4]=[0;4];
		for i in 0..4{temp[i]=rlb.remove(0)};
		let loy=unbyte32(temp);
//reconstitute Gold.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let lgp=unbyteus(temp);
//reconstitute id.
		let mut temp:[u8;8]=[0;8];
		for i in 0..8{temp[i]=rlb.remove(0)};
		let lid=unbyteus(temp);
//reconstitute name
		let true_name= indtrack;
		indtrack+= 1;
		let mut lname="";
		let mut ltype="";
		let mut lsubtype="";
		let mut laffinity="";
		let mut lspellist:Vec<i8>=Vec::new();
		let mut linventory= Vec::new();
		for x in mons.iter(){
			if ltxt[indtrack].as_str()==x.name{
				lname=x.name
			}else{lname=lname};
			if ltxt[indtrack+1].as_str()==convert_mon_type(x.Type){
				ltype=convert_mon_type(x.Type)
			}else{ltype=ltype};
			if ltxt[indtrack+2].as_str()==convert_affinity(x.SubType){
				lsubtype=convert_affinity(x.SubType)
			}else{lsubtype=lsubtype};
			if ltxt[indtrack+3].as_str()==convert_affinity(x.Affinity){
				laffinity=convert_affinity(x.Affinity)
			}else{laffinity=laffinity};
		};
		indtrack+=4;
		for i in 0..sp_len{
			for y in spl.iter(){
				//println!("{:?}",ltxt[i+indtrack].as_str());
				if ltxt[i+indtrack].as_str()==y.name{
					lspellist.push(y.id)
				}else{};
			};
		};
		println!("{:?}",lspellist);
		indtrack+=sp_len;
		//implement inventory later.
//export from iterator.
		
		//write final coords, names, party members and their location.
		*coords = [loy,lox];
		p_names.push(ltxt[true_name].clone());
		party.push((Lifeform{
					name: lname,
					Type: convert_affinity_rev(ltype),
					SubType: convert_affinity_rev(lsubtype),
					Affinity: convert_affinity_rev(laffinity),
					Exp: exp,
					ExpUsed: expu,
					Spellist: lspellist,
					MP: mp,
					HP: hp,
					Speed: speed,
					Attack: attack,
					Defence: defence,
					WM: wm,
					BM: bm,
					MP_shade: mp,
					HP_shade: hp,
					Speed_shade: speed,
					Attack_shade: attack,
					Defence_shade: defence,
					WM_shade: wm,
					BM_shade: bm,
					Alive: alive,
					Unclean: unclean,
					Inventory: linventory,
					Gold: lgp,
					id: lid,
					},0))	
	};
	
	//reconstitude kill list (very hackable file).
	let mut killist:Vec<(String,u64)> = Vec::with_capacity(kill_n);
	for i in 0..(kill_n) {
		killist.push((ltxt[i+indtrack].clone(),kill_n_vector[i]));
	};
	*my_kills = KillList::new();		
	my_kills.replace_kills(killist);
	println!("Killist reconstituted: {:?}",my_kills);
		
	*pl = place_loader(&world,[coords[1],coords[0]]);
	*pl = (world.len()-1-pl.0,pl.1);
	//*pl = (pl.0,pl.1);
	*p_loc = world[world.len()-1-pl.0][pl.1].clone();
}


//SAVE FUNCTION.
pub fn save(xx:&Vec<(Lifeform,usize)>,
		nx:&Vec<String>,
		spl:&Vec<Spell>,
		p:&Place,
		s:&MyStories,
		d:&MyDungeons,
		k:&KillList){
	
	let mut s_name:String = nx[0].to_owned();
	let dir=env::current_dir().unwrap().join("as/saves");
	let mut f1 = dir.join(s_name.clone()+".msqrtxt");
	let mut f2 = dir.join(s_name.clone()+".msqrb");
	let mut fs = dir.join(s_name.clone()+".msqrp");
	let mut fd = dir.join(s_name.clone()+".msqrd");
	let mut fk = dir.join(s_name+".msqrk");
	
	//Need to "safetify" this.		
	let mut stxt = File::create(&f1).unwrap();
	let mut sfile = File::create(&f2).unwrap();
	let mut splot = File::create(&fs).unwrap();
	let mut sdung = File::create(&fd).unwrap();
	let mut skill = File::create(&fk).unwrap();
	
	//Write plot completion file.
	if s.len()>0 {
		let finlen = s.len()*8;  //8 bytes per entry.
		let mut sids = s.get_ids();
		let ids_pointer = unsafe {transmute::<*mut (u32,u16,u16),*mut u8>(sids.as_mut_ptr())};
		forget(sids);
		let sids = unsafe {
			 Vec::from_raw_parts(
				ids_pointer,
				finlen,
				finlen,
			)
		};
	
		splot.write_all(&sids).expect("Tried to save plot, but lost it.");
	};	
	
	//Write dungeon completion file.
	if d.len()>0 {
		let finlen = d.len()*16;  //[u32;4]=4*4=16 bytes per entry.
		let mut dids = d.get_ids();
		let ids_pointer = unsafe {transmute::<*mut [u32;4],*mut u8>(dids.as_mut_ptr())};
		forget(dids);
		let dids = unsafe {
			 Vec::from_raw_parts(
				ids_pointer,
				finlen,
				finlen,
			)
		};
	
		sdung.write_all(&dids).expect("Tried to save dungeons, but couldn't get out.");
	};
	
	//Write kill list (number) file.
	let klen = k.len();
	let kills = k.take_kills();
	if klen>0 {
		let klen = k.len();
		let mut kill_numbers = Vec::with_capacity(klen+1);
		kill_numbers.push(klen as u64); //Important step.
		
		for x in kills.iter() {kill_numbers.push(x.1);};
		
		let finlen = k.len()*8+8;  //u64 therefore 8 bytes per entry. First entry is the length as u64 (another 8 bytes).
		let ids_pointer = unsafe {transmute::<*mut u64,*mut u8>(kill_numbers.as_mut_ptr())};
		forget(kill_numbers);
		let kill_numbers = unsafe {
			Vec::from_raw_parts(
				ids_pointer,
				finlen,
				finlen,
			)
		};
	
		skill.write_all(&kill_numbers).expect("Tried to save kills, but didn't survive.");
	};					

	let n_party:u8=xx.len() as u8;
	sfile.write_all(&[n_party]);
	for i in 0..xx.len(){
		let l1=xx[i].0.Spellist.len();
		let l2=xx[i].0.Inventory.len();
		let exp:[u8;8]=byter64(by_f64(xx[i].0.Exp as f64));
		let expu:[u8;8]=byter64(by_f64(xx[i].0.ExpUsed as f64));
		let mp:[u8;8]=byter64(by_f64(xx[i].0.MP as f64));
		let hp:[u8;8]=byter64(by_f64(xx[i].0.HP as f64));
		let speed:[u8;8]=byter64(by_f64(xx[i].0.Speed as f64));
		let att:[u8;8]=byter64(by_f64(xx[i].0.Attack as f64));
		let def:[u8;8]=byter64(by_f64(xx[i].0.Defence as f64));
		let wm:[u8;8]=byter64(by_f64(xx[i].0.WM as f64));
		let bm:[u8;8]=byter64(by_f64(xx[i].0.BM as f64));
		let lspl:[u8;8]=byterus(l1);
		let linv:[u8;8]=byterus(l2);
		let live:u8=if xx[i].0.Alive==true{1}else{0};
		let uncl:u8=if xx[i].0.Unclean==true{1}else{0};
		let lox:[u8;4]=byteri32(p.xy[0]);
		let loy:[u8;4]=byteri32(p.xy[1]);
		let lgp:[u8;8]=byterus(xx[i].0.Gold);
		let lid:[u8;8]=byterus(xx[i].0.id);
		
		sfile.write_all(&exp).expect("error writing sfile");
		sfile.write_all(&expu).expect("error writing sfile");
		sfile.write_all(&mp).expect("error writing sfile");
		sfile.write_all(&hp).expect("error writing sfile");
		sfile.write_all(&speed).expect("error writing sfile");
		sfile.write_all(&att).expect("error writing sfile");
		sfile.write_all(&def).expect("error writing sfile");
		sfile.write_all(&wm).expect("error writing sfile");
		sfile.write_all(&bm).expect("error writing sfile");
		sfile.write_all(&lspl).expect("error writing sfile");
		sfile.write_all(&linv).expect("error writing sfile");
		sfile.write_all(&[live]).expect("error writing sfile");
		sfile.write_all(&[uncl]).expect("error writing sfile");
		sfile.write_all(&lox).expect("error writing sfile");
		sfile.write_all(&loy).expect("error writing sfile");
		sfile.write_all(&lgp).expect("error writing sfile");
		sfile.write_all(&lid).expect("error writing sfile");
		
		//write bits and bobs as txt.
		stxt.write(&nx[i].as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&xx[i].0.name.as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_mon_type(xx[i].0.Type).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_affinity(xx[i].0.SubType).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		stxt.write(&convert_affinity(xx[i].0.Affinity).as_bytes()).expect("error writing stxt");
		stxt.write("\n".as_bytes()).expect("error writing stxt");
		//Write spellist as txt.
		if l1>0 {
			for j in 0..l1{
				stxt.write(arcana_name_from_spell_id(spl,xx[i].0.Spellist[j]).as_bytes()).expect("error writing stxt spellist");
				stxt.write("\n".as_bytes()).expect("error writing stxt spellist");
			};
		}else{};
//		if l2>0{
//			for j in 0..l2{
//				stxt.write(xx[i].0.Inventory[j].as_bytes());
//				stxt.write("\n".as_bytes());
//			};
//		}else{};
	};
	//write killist names as txt.
	if klen>0 {
		for k in 0..klen {
			stxt.write(kills[k].0.as_bytes()).expect("error writing stxt killist");
			stxt.write("\n".as_bytes()).expect("error writing stxt killist");
		};
	};
	
}

//Various game related functions.


//This fucntion is for picking sidekick's name from a thematic list.
fn sidekick_namer(x:&Vec<(Lifeform,usize)>)-> String{
	let mut names:Vec<&str>=Vec::new();
	if x[1].0.name=="Witch"{
		names.extend(["Madoka","Walpurgis","Yaga","Maud","Cassandra","Circe","Morgan","Hecate","Medea","Agamede","Ariadne"].iter());
	}else if x[1].0.name=="Warrior"{
		names.extend(["Andromachos","Hector","Achilles","Bruce","Jackie","Musashi","Ilya","Alexander","Gengis","Julius"].iter());
	}else{
		names.extend(["Kino","Odysseos","Scott","Piri","Amsund","Marco","Yuri","Neil","Orpheus","Jason","Mallory","Messner","Kirk"].iter());
	};
    let picker=rand::thread_rng().gen_range(0,names.len());
    let nome=names[picker].to_string();
    println!("\nYour sidekick happens to be {} the {}.", &nome, x[1].0.name);
    //Class and name setter. OVER.
    nome
}

//Sidekick maker
pub fn sidekick_maker(mut party: &mut Vec<(Lifeform,usize)>, mut p_names: &mut Vec<String>) {
	let dlb = rand::thread_rng().gen_range(-12,13) as f32;
	party[1].0.Attack-= party[1].0.Attack/25.0*dlb;
	party[1].0.Defence+= party[1].0.Defence/25.0*dlb;
	party[1].0.WM+= party[1].0.WM/25.0*dlb;
	party[1].0.BM-= party[1].0.BM/25.0*dlb;
	party[1].0.Attack_shade= party[1].0.Attack;
	party[1].0.Defence_shade= party[1].0.Defence;
	party[1].0.WM_shade= party[1].0.WM;
	party[1].0.BM_shade= party[1].0.BM;
	println!("Self Attack:{}",party[1].0.Attack);
	if dlb<0.0{
		party[1].0.Spellist = match party[1].0.name{
			"Witch"=>vec![S_CURE,S_EMBER,S_DARKNESS,S_SPARK],
			"Warrior"=>vec![S_DARKNESS],
			"Wonderer"=>vec![S_EMBER,S_DARKNESS,S_SLOW],
			_=>vec![],
		};
	}else{
		party[1].0.Spellist = match party[1].0.name{
			"Witch"=>vec![S_CURE,S_LIGHT,S_EMBER,S_EXORCISM],
			"Warrior"=>vec![S_LIGHT],
			"Wonderer"=>vec![S_CURE,S_HASTE,S_LIGHT],
			_=>vec![],
		};
	};
	p_names.push(sidekick_namer(party));
}

//engenA (generates battle variable y).
fn engenA()->Vec<usize> {
	
	println!("Entering engenA");
    let ngroups=rand::thread_rng().gen_range(0,10);
    let ngroups=[1,1,1,2,2,2,3,3,3,4][ngroups];
    let mut gsizes: Vec<usize>=vec!(0;ngroups);
    for i in 0..ngroups{
		let ggen=rand::thread_rng().gen_range(0,10);
        gsizes[i]=[1,1,1,2,2,2,3,3,4,4][ggen]
    };
	println!("Exiting engenA");
    gsizes
}

//engenA for dungeons (generates battle variable y).
fn engenA_dun(locus: &Place)->Vec<usize> {
	println!("Entering engenA_dun");
    let ngroups=rand::thread_rng().gen_range(0,10);
    let ngroups=locus.engenA[ngroups];
    let mut gsizes: Vec<usize>=vec!(0;ngroups);
    for i in 0..ngroups {
		let ggen=rand::thread_rng().gen_range(0,10);
        gsizes[i]=locus.engenG[ggen]
    };
	println!("Exiting engenA_dun");
    gsizes
}

//engenB (generates battle variable x).
fn engenB<'a,'b>(A:&'a Vec<usize>,B:&'b Place,bestiary:&Vec<Lifeform>)->Vec<(Lifeform,usize)>{
	println!("Entering engenB. Locale: {}. Populatations {}. engenA: {:?}",B.name,B.popu.len(),A);
    let mut enemies: Vec<(Lifeform,usize)> = Vec::new();
    let mut totapop = 0;
    let mut tomoty: Vec<u8> = Vec::new();
    let mut tomo: Vec<&str> = Vec::new();
    let mut mon_type:u8 = GOBLIN;
    let n_groups = A.len();
     println!("EngenB initiated");
     //generate the thread_gen max value and the type it corresponds to.
    for l in 0..B.popu.len() {
		let subpop=vec!(B.popu[l].0; B.popu[l].2 as usize);
		let subtyp=vec!(B.popu[l].1; B.popu[l].2 as usize);
        totapop +=B.popu[l].2;
        tomo.extend(subpop);
        tomoty.extend(subtyp);
    };
    println!("threadgen value obtained.tomo: {}. tomoty: {}.",tomo.len(),tomoty.len());
    //generate group type and govern inner loops.
    for i in 0..n_groups{
        //generate type for group [i]
        let i_type=rand::thread_rng().gen_range(0,totapop)  as usize;
        mon_type=tomoty[i_type];
        //generate each entity in group [i]. NEED TO:select monsters.
        println!("A: {:?}",A);
        for k in 0..A[i]{
			let mut enemy_n: &str = "";
			let mut k_name:usize = 0;
			//Make sure that all monsters are of the same type, this will get complex.
			loop{
			    k_name=rand::thread_rng().gen_range(0,totapop) as usize;
			    if tomoty[k_name]==mon_type{break}else{}
			    println!("Post cleansing tomoty length: {}",tomoty.len());	   
			};
			enemy_n = tomo[k_name];
			let enemy:Lifeform = bestiary[vvwhich_ln(&bestiary,enemy_n)[0]].clone();
			enemies.push((enemy,i+1))
		};
    };
	println!("Exiting engenB");
    enemies
}

//generates the enemy vector for a scripted story encounter.
fn engen_story_marker(){}
fn engen_story(content:&Content) -> Vec<(Lifeform,usize)> {
	let mut enemies:Vec<(Lifeform,usize)> = Vec::with_capacity(23);
	
	for x in content.actors.iter(){
		enemies.push((x.1.clone(),x.2));
	};

	enemies
}

//lvl up function. Does not currently cover spells.
pub fn lvl_upg (mut party:&mut Vec<(Lifeform,usize)>,
			r:usize,
			mut t_e_c_i_ll:&mut [bool;8]) {
	
	let i:usize = r/10;
	let mut expble:f32 = party[i].0.Exp-party[i].0.ExpUsed;
	println!("Exp to use:{}",expble);
	match r%10 {
		1 => {party[i].0.HP+= expble*2.0;
			  party[i].0.ExpUsed+= expble;},
		2 => {party[i].0.MP+= expble*2.0;;
			  party[i].0.ExpUsed+= expble;},
		3 => {party[i].0.Speed+= expble/5.0;
			  party[i].0.ExpUsed+= expble;},
		4 => {party[i].0.Attack+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		5 => {party[i].0.Defence+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		6 => {party[i].0.WM+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		7 => {party[i].0.BM+= expble/2.0;
			  party[i].0.ExpUsed+= expble;},
		_ => {},
	};
	expble = 0.0;
	for x in party.iter() {
		println!("Exp left on {}:{}",x.0.name,x.0.Exp-x.0.ExpUsed);
		expble+= x.0.Exp-x.0.ExpUsed;
	};
	if expble==0.0 {t_e_c_i_ll[6] = false;};
}

fn encounter_starter_marker(){}
//standard encounter generator.
pub fn encounter_starter(party: &mut Vec<(Lifeform,usize)>,
					 mut enemies: &mut Vec<(Lifeform,usize)>,
					 mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 p_loc: &Place,
					 mons: &Vec<Lifeform>) {
	*enemies = engenB(&engenA(),&p_loc,mons);
	for x in party.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in enemies.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in encounter.iter() {println!("{}: {}",x.1,x.0.name)};
}

//Dungeon encounter generator.
pub fn encounter_starter_dun(party: &mut Vec<(Lifeform,usize)>,
					 mut enemies: &mut Vec<(Lifeform,usize)>,
					 mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 p_loc: &Place,
					 mons: &Vec<Lifeform>) {
	*enemies = engenB(&engenA_dun(p_loc),&p_loc,mons);
	for x in party.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in enemies.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in encounter.iter() {println!("{}: {}",x.1,x.0.name)};
}

//standard encounter generator.
pub fn encounter_starter_story(party: &mut Vec<(Lifeform,usize)>,
					 mut enemies: &mut Vec<(Lifeform,usize)>,
					 mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
					 content: &Content,
					 mons: &Vec<Lifeform>) {
	*enemies = engen_story(content);
	for x in party.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in enemies.iter() {encounter.push((x.0.clone(),x.1,[None,None]))};
	for x in encounter.iter() {println!("{}: {}",x.1,x.0.name)};
}

pub fn character_dl_mod(mut character: &mut Lifeform, dl: isize) {
	let dlb = dl as f32;
	character.Attack-= character.Attack/25.0*dlb;
	character.Defence+= character.Defence/25.0*dlb;
	character.WM+= character.WM/25.0*dlb;
	character.BM-= character.BM/25.0*dlb;
	character.Attack_shade= character.Attack;
	character.Defence_shade= character.Defence;
	character.WM_shade= character.WM;
	character.BM_shade= character.BM;
	println!("Self Attack:{}",character.Attack);
	if dlb<0.0{
		character.Spellist = match character.name {
			"Witch"=>vec![S_CURE,S_EMBER,S_DARKNESS,S_SPARK],
			"Warrior"=>vec![S_DARKNESS],
			"Wonderer"=>vec![S_EMBER,S_DARKNESS,S_SLOW],
			_=>vec![],
		};	
	}else{
		character.Spellist = match character.name {
			"Witch"=>vec![S_CURE,S_LIGHT,S_EMBER,S_EXORCISM],
			"Warrior"=>vec![S_LIGHT],
			"Wonderer"=>vec![S_CURE,S_HASTE,S_LIGHT],
			_=>vec![],
		};
	};
}

//function to get a story from its id.
pub fn get_a_story<'a>(id:u32,stories:&'a Vec<Story<'a>>)-> Option<&'a Story<'a>> {
	for x in stories.iter() {
		if x.id==id {return Some(x)};
	}
	None
}

//function to get a dungeon from its id.
pub fn get_a_dungeon(id:u32,dungeons:&Vec<Dungeon>)-> Option<&Dungeon> {
	for x in dungeons.iter() {
		if x.id==id {return Some(x)};
	}
	None
}
