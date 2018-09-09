/// shared_moose.rs contains general and shared functions which are
/// shared between modules, and certain inner game logic not related to
/// battle.
extern crate conrod;

use std::fs::File;
use std::env;
use std::io::{Read,Write};
use std::mem::{forget,transmute};

use smoose::MyStories;
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
				my_stories:&mut MyStories){	
	println!("filename: {}",file_name);
	//Initiate raw data constructs.
	let mut rlb = Vec::with_capacity(8000);
	let mut ltxt:Vec<String> = Vec::new();
	let mut rltxt = String::new();
	let to_open_a = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrb");
	let to_open_b = env::current_dir().unwrap().join("as/saves").join(file_name.clone()+".msqrtxt");
	let to_open_s = env::current_dir().unwrap().join("as/saves").join(file_name+".msqrp");

	println!("msqrb: {:?}",to_open_a);
	println!("msqrtxt: {:?}",to_open_b);
	println!("msqrp: {:?}",to_open_s);
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
			let mut rplot:Vec<u8> = Vec::with_capacity(500);
			fp.read_to_end(&mut rplot);
			let plot_len = rplot.len()/5; //Assumes vec<(u32,bool)>, hence 5 bytes,
			let plot_pointer = unsafe {transmute::<*mut u8,*mut (u32,bool)>(rplot.as_mut_ptr())};
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
		_		=> {println!("There is no plot.");},
	};
	println!("Got here");
	
	//reformat text file from &str to String.
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
		s:&MyStories){
	
	let mut s_name:String = nx[0].to_owned();
	let dir=env::current_dir().unwrap().join("as/saves");
	let mut f1 = dir.join(s_name.clone()+".msqrtxt");
	let mut f2 = dir.join(s_name.clone()+".msqrb");
	let mut fs = dir.join(s_name+".msqrp");
	
	
	//Need to "safetify" this.		
	let mut stxt = File::create(&f1).unwrap();
	let mut sfile = File::create(&f2).unwrap();
	let mut splot = File::create(&fs).unwrap();
	
	if s.len()>0 {
		let finlen = s.len()*5;
		let mut sids = s.get_ids();
		let ids_pointer = unsafe {transmute::<*mut (u32,bool),*mut u8>(sids.as_mut_ptr())};
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
		if l1>0{
			for j in 0..l1{
				stxt.write(arcana_name_from_spell_id(spl,xx[i].0.Spellist[j]).as_bytes()).expect("error writing stxt");
				stxt.write("\n".as_bytes()).expect("error writing stxt");
			};
		}else{};
//		if l2>0{
//			for j in 0..l2{
//				stxt.write(xx[i].0.Inventory[j].as_bytes());
//				stxt.write("\n".as_bytes());
//			};
//		}else{};
	};
		
}
