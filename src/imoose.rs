#![allow(unused_imports)]
#![allow(non_snake_case)]

///Quest for the Moose:imoose
///
///The imoose module deals with monster AI.
///The current method takes a record of "historic" (ie randomly generated
///battles and processes it to find which action should result in the
///heighest chance of victory. It is a very simple, inefficient and
///fairly universal AI.
///
///Once surface polishing is finished, I will return to this module.
///imoose depends on lmoose, but strictly doesn't need to.
///
///~Alek Zholobenko
///

extern crate time;
extern crate num;
extern crate std;

//mod lmoose;
use lmoose::{Spell,Item,Lifeform,Shade,Place,cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
			 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
			 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
			 apocalypse,timestop,world,goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
			 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
			 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,shortstaff};
use lmoose::{ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
		     DEATH,DESERT,ELF,EVIL,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
		     HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
			 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
			 TELEPORTATION,TIME,TUNDRA,UNDEAD,WATER,WITCH,WHITE,NONE};
use shared_moose::*;
use shared_moose::{lhas};

			 
use num::Num;
use std::collections::HashMap;
use std::mem::transmute;
use std::thread;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use time::PreciseTime;

const SWITCH_1:bool = true;


//Since the "brain" now lives in a seperate thread,
//it is worth taking the time at the start of each battle to make
//these hashes to speed up all subsequent battles.
//Thus this function doesn't speed things up now.
//But it allows for the fast use of the simple cause-effect protocol.
//PS lore_hash_by_end is probably completely useless. Will think on it.
fn ai_accelerator_hash_marker(){}
pub fn ai_accelerator_hash<'a,'b> (lore: &'a Vec<Vec<[u8;28]>>,
							differences: &'b Vec<Vec<[i8;23]>>,
							last_lines: &mut Vec<&'a [u8;28]>,
					    	lore_hash_by_end: &mut HashMap<&'a[u8;28],Vec<&'a Vec<[u8;28]>>>,
						    cause_effect: &mut HashMap<&'a[u8],Vec<&'b [i8;23]>>,
						    all_causes: &mut Vec<&'a[u8]>,
						) -> HashMap<&'a[u8],[f32;23]> {
							
	println!("lore.len() = {}\ndifferences.len() = {}",lore.len(),differences.len());
	//Initiate the vectors which will then be dismantled.	
	
	//For cause_effect. NB, this is also a complex structure. Use my head.
	
	//Build up battles_by_end and ends. Differences should be same length as battles.
	//a difference table, however, should be shorter.
	for (x,y) in lore.iter().zip(differences.iter()) {
		
		let lstx = x.len()-1;
		let lsty = y.len()-1;
		
		//insert next reference into lore_hash_by_end.
		let mut insert_into_lhbe = false;
		match lore_hash_by_end.get_mut(&x[lstx]) {
			Some(battle) => {
				battle.push(&x);
			},
			_			 => {
				insert_into_lhbe = true;
			},
		};
		if insert_into_lhbe {lore_hash_by_end.insert(&x[lstx],vec![&x]);};
		
		//work to make cause_effect.
		//cycle through one battle difference tables.
		for j in 0..lsty {
			//insert into cause_effects.
			let mut insertion = false;
			match cause_effect.get_mut(&x[j][2..5]) {
				Some(effects) => {
					effects.push(&y[j]);
				},
				_			  => {
					insertion = true;
					all_causes.push(&x[j][2..5]);
				},
			};
			
			if insertion {cause_effect.insert(&x[j][2..5],vec![&y[j]]);};		
		};				
	};
	
	//create averages hash
	let mut averages_hash:HashMap<&[u8],[f32;23]> = HashMap::with_capacity(200000);
	//Insert into cause effect hashmap.
	for cause in all_causes.iter() {
		averages_hash.insert(
			cause,mean23(cause_effect.get(cause).unwrap())
		);
	};
	println!("Accelerator finished.");			
	averages_hash		
}

// Generates the difference tables used in part_b2
// To be done in the initial part of the battle.
// cause_effect is a complex tuple. The second half is a collection of 
// all possible outcomes of an action.
// NB lore_has_by_end I think is of limited value.
fn ai_part_b1_marker(){}
pub fn ai_part_b1<'a,'b>(lore: &'a Vec<Vec<[u8;28]>>,
						differences: &'b mut Vec<Vec<[i8;23]>>) {
	
	//Defensive: Reset differences.
	*differences = Vec::with_capacity(lore.len());
	
	//Fill differences using lore.
	for x in lore.iter() {
		let mut diff_n:Vec<[i8;23]> = Vec::with_capacity(x.len());
		
		for j in 0..(x.len()-1) {
			let mut slice = [0;23];
			for i in 5..28 {
				slice[i-5] = (x[j][i] as i16 - x[j+1][i] as i16) as i8;
			};
			diff_n.push(slice);
		};
		diff_n.push([0;23]);
		differences.push(diff_n);
	}
}

//The winning team from the winning team's perspective;
//different lifeforms have different victory conditions.
//Simple version.
fn goal_marker(){}
fn goal(xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, 		 //participating battles
		ii:usize,						 //self index.
		l_line:&[u8;28],
		all_groups:&Vec<usize>)->bool{
	let ii8= ii as u8;
	let owng= xx[ii].1;
	let mut victory=true;
	
	if !xx[ii].0.Alive {         //undead victory condition.
		let mut alive=false;
		for i in 0..xx.len(){
			if (all_groups[i]!=owng)
			 & ((l_line[i+6]>0) & (l_line[i+6]<255)) 
			 & xx[i].0.Alive {
				alive=true
			}else{};
		};
		if alive {victory=false}else{}
	}else if xx[ii].0.Type==ANGEL{  //victory conditions for angels. (KIll giants, undeaad and humans first).
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for (i,y) in xx.iter().enumerate() {
			if (y.1!=xx[ii].1)
			 & ((y.0.Type==GIANT)||(y.0.Type==UNDEAD)||(y.0.Type==HUMAN))
			 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {hate=true}else{};
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& ((xx[i].0.Type==GIANT)||(xx[i].0.Type==UNDEAD)||(xx[i].0.Type==HUMAN))
				& ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & ((l_line[i+6]>0) & (l_line[i+6]<255)) {allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
		}
	}else if xx[ii].0.Type==BEAST{ //Victory conditions for beasts: Kill all unclean and undead, then everything else.
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for (i,y) in xx.iter().enumerate() {
			if (y.1!=xx[ii].1)
			 & (!y.0.Alive || y.0.Unclean)
			 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {hate=true}else{}
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				 & (!xx[i].0.Alive ||!xx[i].0.Unclean)
				 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& (l_line[i+6]>0){foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & ((l_line[i+6]>0) & (l_line[i+6]<255)) {allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
		}
	}else if xx[ii].0.Type==GOBLIN { //Victory conditions for goblin: Kill all non-goblins, then everything else.
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for (i,y) in xx.iter().enumerate() {
			if (y.1!=xx[ii].1)
			 & (y.0.Type != GOBLIN)
			 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {hate=true}else{}
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				 & (xx[i].0.Type != GOBLIN)
				 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & ((l_line[i+6]>0) & (l_line[i+6]<255)) {allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
		}
	}else{                           //victory condition for normal creatures.
		let mut foes_live=false;
		let mut allies_live=false;
		for i in 0..xx.len(){
			if (all_groups[i]!=owng)
			 & ((l_line[i+6]>0) & (l_line[i+6]<255)) {foes_live=true
			}else{};
			if (all_groups[i]==owng)
			       & ((l_line[i+6]>0) & (l_line[i+6]<255)) {allies_live=true
			}else{};			 
		};
		if foes_live || !allies_live {victory=false}else{}
	};					
	victory
}

//The winning team from the winning team's perspective;
//different lifeforms have different victory conditions.
//Simple version.
fn perfect_goal_marker(){}
fn perfect_goal(xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, 		 //participating battles
		ii:usize,						 //self index.
		all_groups:&Vec<usize>)->[Option<u8>;22] {
			
	let ii8= ii as u8;
	let owng= xx[ii].1;
	let mut victory=true;
	let mut output = [None;22];
	if xx.len()>22 {return output;}; //CRASH PROTECTION is too many.
	
	if !xx[ii].0.Alive {         //undead victory condition. (kill all things living).
		let mut alive=false;
		for i in 0..xx.len(){
			if (all_groups[i]!=owng)
			 & xx[i].0.Alive {
				output[i] = Some(0);
			}else{};
		};
		
	}else if xx[ii].0.Type==ANGEL{  //victory conditions for angels. (KIll giants, undeaad and humans first).
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for y in xx.iter(){
			if (y.1!=xx[ii].1)
			 & ((y.0.Type==GIANT)||(y.0.Type==UNDEAD)||(y.0.Type==HUMAN))
			 & (y.0.HP_shade>0.0){hate=true}else{};
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& ((xx[i].0.Type==GIANT)
				||(xx[i].0.Type==UNDEAD)
				||(xx[i].0.Type==HUMAN)) {output[i] = Some(0);};
			};
			
		}else{
			for i in 0..xx.len(){
				if all_groups[i]!=owng {
					output[i] = Some(0);
				}else{
					output[i] = Some(5);
				};
			};
		};
			
	}else if xx[ii].0.Type==BEAST{ //Victory conditions for beasts: Kill all unclean and undead, then everything else.
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for y in xx.iter(){
			if (y.1!=xx[ii].1)
			 & (!y.0.Alive || y.0.Unclean)
			 & (y.0.HP_shade>0.0){hate=true}else{}
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				 & (!xx[i].0.Alive ||!xx[i].0.Unclean) {output[i] = Some(0);};
			};
		}else{
			for i in 0..xx.len(){
				if all_groups[i]!=owng {
					output[i] = Some(0)
				}else{
					output[i] = Some(5)
				};
			};
		}
	}else if xx[ii].0.Type==GOBLIN { //Victory conditions for goblin: Kill all non-goblins, then everything else.
		let mut foes_live=false;
		let mut allies_live=false;
		let mut hate=false;
		for y in xx.iter(){
			if (y.1!=xx[ii].1)
			 & (y.0.Type != GOBLIN)
			 & (y.0.HP_shade>0.0){hate=true}else{}
		};
		if hate {
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				 & (xx[i].0.Type != GOBLIN) {output[i] = Some(0);};
			};
		}else{
			for i in 0..xx.len(){
				if all_groups[i]!=owng {
					output[i] = Some(0)
				}else{
					output[i] = Some(5)
				};		 
			};
		}
	}else{                           //victory condition for normal creatures.
		let mut foes_live=false;
		let mut allies_live=false;
		for i in 0..xx.len(){
			if all_groups[i]!=owng {
				output[i] = Some(0)
			}else{
				output[i] = Some(5)
			};		 
		};
	};					
	output
}

//A vector of entries which are not the same.
fn vvinv<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]!=b{ivec.push(i)}
		else{continue}
	}
	ivec
}

//unnecessary function
//which function for first index (faster).
fn lwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{
			ivec.push(i);
			return ivec}
		else{}
	}
	ivec
}

//unnecessary function
//which function for first index..
//..from the back end...
//..(faster than vvwhich)..
//..(slower than lwhich).. (in theory)
fn rwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	let al=a.len();
	for i in 0..al{
		if a[al-i]==b{
			ivec.push(i);
			return ivec}
		else{continue}
	}
	ivec
}

//does vector contain b? From back.
fn rhas<T:PartialOrd>(a:&Vec<T>, b:&T)->bool{
	let mut ihaz=false;
	for x in a.iter().rev(){
		if x==b{
			ihaz=true;
			return ihaz}
		else{continue}
	}
	ihaz
}


//which function for multiple. For [u8;28].
fn vawhich<T:PartialOrd>(a:&[T;28], b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{ivec.push(i)}
		else{continue}
	}
	ivec
}
//which function for first index (faster). For [u8;28].
fn lawhich<T:PartialOrd>(a:&[T;28], b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{
			ivec.push(i);
			return ivec}
		else{}
	}
	ivec
}
//which function for first index..  For [u8;28].
//..from the back end...
//..(faster than vvwhich)..
//..(slower than lwhich).. (in theory)
fn rawhich<T:PartialOrd>(a:&[T;28], b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	let al=a.len();
	for i in 0..al{
		if a[al-i]==b{
			ivec.push(i);
			return ivec}
		else{continue}
	}
	ivec
}

//does a contain b? For [u8;28].
fn lahas<T:PartialOrd>(a:&[T;28], b:&T)->bool{
	let mut ihaz=false;
	for x in a.iter(){
		if x==b{
			ihaz=true;
			return ihaz}
		else{}
	}
	ihaz
}
//does a contain b? From back. For [u8;28].
fn rahas<T:PartialOrd>(a:&[T;28], b:&T)->bool{
	let mut ihaz=false;
	for x in a.iter().rev(){
		if x==b{
			ihaz=true;
			return ihaz}
		else{continue}
	}
	ihaz
}

//Counts cases of "indv[i]" for each entry of "indv" in "mesv".
fn vcount<T:PartialOrd + Copy>(indv:&Vec<T>, mesv:&Vec<T>)->Vec<f32>{
	let il=indv.len();
	let mut counts:Vec<f32>=vec![0.0;il];
	if il>0{
		for i in 0..indv.len(){
			for x in mesv.into_iter(){
				if x==&indv[i]{counts[i]+=1.0}else{}
			}
		}
	}else{};
	counts
}

//VECTORS MUST HAVE SAME LENGTH!!
fn vf32ratio(num:&Vec<f32>, denom:&Vec<f32>)->Vec<f32>{
	let dnl=denom.len();
	let mut rvec:Vec<f32>= vec!(0.0;dnl);
	for i in 0..dnl{
		if denom[i]>0.0{
			rvec[i]=num[i]/denom[i]
		}else{
			rvec[i]=0.0
		}
	};
	rvec
}

//returns max value of a vector.
fn vnmax<T:PartialOrd+Copy>(a:&Vec<T>)->T{
	let mut max:T=a[0];
	for i in 0..a.len(){
		if max<=a[i]{max=a[i]}
		else{max=max};
	};
	max
}	

//returns index of max value of a vector.
fn vnmaxi<T:PartialOrd+Copy>(a:&Vec<T>)->usize{
	let mut max:T=a[0];
	let mut maxi:usize=0;
	for i in 0..a.len(){
		if max<=a[i]{
			max=a[i];
			maxi=i
		}else{
			max=max
		};
	};
	maxi
}	

//returns index of min value of a vector.
fn vnmini<T:PartialOrd+Copy>(a:&Vec<T>)->usize{
	let mut min:T=a[0];
	let mut mini:usize=0;
	for i in 0..a.len(){
		if min>a[i]{
			min=a[i];
			mini=i
		}else{
			min=min
		};
	};
	mini
}	

pub fn byteru16(x:u16)->[u8;2]{
    let y= unsafe {transmute(x.to_be())};
    y
}

pub fn permit_a(i:usize)->usize{
	let modi= if i<9 {3}else{i-5};
	modi
}

//NB Order and Orders (record and now) must be the same length);
fn order_matcher(now:&Vec<(u8,u8)>,record:&Vec<(u8,u8)>)->bool{
	let mut compliance:usize=0;
	let mut compliant=false;
	let n_l=now.len();
	let r_l=record.len();
	if (n_l==0) || (r_l==0){
		compliant=true;
		return compliant
	}else{};
	let check_n=if n_l<=r_l{n_l}else{r_l};
	for i in 0..check_n{
		if now[i]==record[i]{compliance+=1}else{}
	};
	let threshold=if check_n>3{3}else{check_n};
	if compliance>threshold{compliant=true}else{compliant=false};
	compliant
}

fn state_matcher(&now:&[u8;28],record:&[u8;28],cms:usize,tolerance:usize)->bool{
	let mut compliance:usize=0;
	let mut compliant=false;
	for i in 6..(6+cms){
		if (now[i]+2>record[i]) & (now[i]-2<record[i]){
			compliance+=1}else{}
	};
	if compliance==cms{true}else{false}
}


fn light_m(now_l:i32,observed_l:i32)->bool{
	let nl=now_l as f32;
	let ol=observed_l as f32;
	let upper_lim:f32=nl*1.1+5.0;
	let lower_lim:f32=nl*0.9-5.0;
	if (ol<=upper_lim) & (ol>=lower_lim){true}else{false}
}

fn vmode<T:PartialOrd+Copy> (data:&Vec<T>)->T{
	let uniq_t=uniq(data);
	let count_t=vcount(&uniq_t,&data);
	let index=vnmaxi(&count_t);
	uniq_t[index]
}

//Not used anymore.
fn essential_n (homo:Vec<[i16;23]>)->([i16;23],[f32;23]){
	let mut ess_a=homo[0];
	let mut means:[f32;23]=[0.0;23];
	let l_h=homo.len() as f32;
	if l_h<1.0{println!("l_h.len()==0! We're gonna crash!");};
	for x in homo.into_iter(){
		for i in 0..23{
			if ess_a[i]!=x[i]{ess_a[i]=0}else{};
			means[i]+=x[i] as f32		
		}
	};
	for i in 0..23{
		if ess_a[i]==0{means[i]=means[i]/l_h}else{means[i]=0.0}
	};
	(ess_a,means)
}

fn rms23<T:Num+Copy>(a:[T;23],now:&[u8;28])->f32
where f32: std::convert::From<T>{
	let mut rms:f32=0.0;
	for i in 0..23{
		rms+=((now[i+5] as f32)-(f32::from(a[i]))).powf(2.0)
	};
	rms.sqrt()
}

fn rms23_special<T:Num+Copy>(a:[T;23],now:&[u8;28])->f32
where f32: std::convert::From<T>{
	let mut rms:f32=0.0;
	for i in 0..23{
		let b=f32::from(a[i]);
		rms+=((now[i+5] as f32)-(b)).powf(2.0)
	};
	rms.sqrt()
}

fn rms23_special_b<T:Num+Copy>(a:[T;23],now:&[u8;28])->f32
where f32: std::convert::From<T>{
	let mut rms:f32=0.0;
	for i in 0..23{
		let b=f32::from(a[i]);
		if b>0.0{rms+=((now[i+5] as f32)-(b)).powf(2.0)}else{}
	};
	rms.sqrt()
}

//special mean function
//uses f32 for speed, but may reduce accuracy sometimes.
fn mean23<T:Num+Copy>(battle_block:&Vec<&[T;23]>)->[f32;23]
where f32: std::convert::From<T> {
	let mut output = [0.0;23];
	let mut n = battle_block.len() as f32;
	if n<1.0 {n = 1.0;}; //A little bit of cheating.
	
	for x in battle_block.iter() {
		for i in 0..23 {output[i]+= f32::from(x[i]);};
	}
	
	for i in 0..23 {output[i] = output[i]/n;};
	output
}

//Convert type.
fn f32_fy(input:&[Option<u8>;22])->[Option<f32>;22] {
	let mut output:[Option<f32>;22] = [None;22];
	for (x,y) in input.iter().zip(output.iter_mut()) {
		if x.is_some() {*y = Some(x.unwrap() as f32);};
	}
	output
} 

//A function to use the cause_effect hashmap and my_causes to build
//rms between desired_goal and each effect.
//rms_builder_cause_effect(&mut my_causes_val,&cause_effect_means,&ideal_effect);
fn rms_builder_cause_effect_marker(){}
fn rms_builder_cause_effect(my_ce:&mut Vec<(&[u8],f32)>,
							ce_mean_hash:&HashMap<&[u8],[f32;23]>,
							ideal_effect:&[Option<f32>;22]) {
	
	for y in my_ce.iter_mut() {
		match ce_mean_hash.get(&y.0) {
			Some(effects) => {
				y.1 = rms23_f32_option(ideal_effect,effects);
				println!("y={}",y.1);
			},
			_			  => {},
		};
	}
}

//A function to give you an RMS from two wather unusual types.
fn rms23_f32_option(des:&[Option<f32>;22],comp:&[f32;23])->f32 {
	let mut rms:f32=0.0;
	for i in 0..22{
		if des[i].is_some() {
			rms+=(comp[i+1]-des[i].unwrap()).powf(2.0);
		};
	};
	rms.sqrt()
}

//A function to get the desired effect from desired_goal and now, and
//that is the ideal effect.
fn idealise_marker(){}
fn idealise(now:&[u8;28],desired_goal:&[Option<f32>;22])->[Option<f32>;22] {
	let mut ideal_effect = [None;22];
	
	for (i,(e,g)) in ideal_effect.iter_mut().zip(desired_goal.iter()).enumerate() {
		if g.is_some() & (now[i+6] != 0) {
			*e = Some(now[i+6] as f32-g.unwrap());
		};
	}
	ideal_effect
}

//takes Vec<([u8],f32)> and returns the [u8] from the minimum f32.
fn minimal_cause(source:Vec<(&[u8],f32)>) -> (usize,usize) {
	//Anti crash clause.
	if source.len()>0 {
		let mut out:(usize,usize) = (source[0].0[0] as usize,source[0].0[1] as usize);
		let mut min_val:f32 = source[0].1;
		for &(slice,val) in source.iter() {
			if val<min_val {
				min_val = val;
				out = (slice[0] as usize,slice[1] as usize);
			};
		};
		out
	}else{
		(255,255)
	}
}


// The main function for the AI decision maker.
// part_b2 will go inside here.
fn ai_part_a_marker(){}
pub fn ai_part_a <'a> (x:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						 ii:usize,                             	//ifast from main battle.
						 turn:usize,
						 lore:&Vec<Vec<[u8;28]>>, 							 //(turn,turn,action,idm,ifast,light,x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7],x[8],x[9],x[10],etc)
						 last_lines: &Vec<&[u8;28]>,
						 differences: &Vec<Vec<[i8;23]>>,					 //lore effect substracted.
						 cause_effect: &HashMap<&[u8],Vec<&[i8;23]>>,   //vector of causes and effects
						 cause_effect_means: &HashMap<&[u8],[f32;23]>,
						 all_causes: &Vec<&[u8]>,
						 now:&[u8;28],              //recl essentially.
						 lsum:i32,
						 order:Vec<(u8,u8)>
					   ) -> (usize,usize) {
						   
	println!("Into ai_part_a");
	let mut action=1;
	let mut idm=255;
	let t0=PreciseTime::now();
	let lore_len = lore.len();
	
	let mut best_act_pm:usize=1;	//Primary choice action
	let mut best_tar_pm:usize=255;	//Primary choice target
	
	let mut best_act_v2:usize=1;
	let mut best_tar_v2:usize=255;
	{	
		println!("x.len={}, ii={}",x.len(),ii);	 
		let selfl= x[ii].clone();                              	//Lifeform acting.
		//println!("Got past first indexer");
		let ii8= ii as u8;									//self index as u8.
		let t_u8= byteru16(turn as u16);					//turn as u8.
		let mut chosen_battles:Vec<&Vec<[u8;28]>>= Vec::with_capacity(lore_len);	//vector of battles fulfilling criteria.
		let mut vict_c_battles:Vec<&Vec<[u8;28]>>= Vec::with_capacity(lore_len);	//vector of battles fulfilling criteria which were "won".
		let mut all_groups= Vec::with_capacity(25);											//vector of groups following normal indexing.
		let mut uniq_group= Vec::with_capacity(25);											//vector of unique group listing.
		let mut chosen_stats:Vec<(u8,u8,usize)>= Vec::with_capacity(lore_len);		//vector of action, target and group for chosen battle lines.
		let mut vict_c_stats:Vec<(u8,u8,usize)>= Vec::with_capacity(lore_len);		//vector of action, target and group for victorious battle lines.
		let mut last_c_stat:Vec<[u8;4]>= Vec::with_capacity(lore_len);	
		let mut last_v_stat:Vec<[u8;4]>= Vec::with_capacity(lore_len);	
		let mut ids=Vec::new();
		let mut ids8=Vec::new();
		let cms=x.len();
			
		for y in x.iter(){			 //make by group vector.
			all_groups.push(y.1);
			ids.push(y.0.id);
			ids8.push(y.0.id as u8)
		};           
		uniq_group= uniq(&all_groups);      //make unique group vector,
		println!("All groups by [i]: {:?}\n All ids by [i]: {:?}\n Unique groups: {:?}",all_groups,ids,uniq_group);     
		println!("Owng: {}",x[ii].1);																					
		
		let mut v_count:Vec<usize> = Vec::new();  //victory count at each trigger.
		let mut l_count:Vec<usize> = Vec::new();   //defeat count at each trigger.
		let mut trigs:Vec<[u8;3]> = Vec::new();       				//unique triggers
		trigs.push([255,255,255]);
		l_count.push(100000);
		v_count.push(0);
		
		//println!("got to iterating lore");
		for y in lore.iter(){ 		//create unique paths for everything. Curently pathed out.
			
			let mut lxsum:i32=0;
			for i in 3..y.len(){
				
				lxsum+=(y[i][5] as i32)-128;
				let mut light=true;
				if (y[i][4]==ii8) & (now[6..28]==y[i][6..28]) & (y[i][2]!=0) & (i>=turn) & light {					//Condition.
					let temp_trig:[u8;3]=[y[i][2],y[i][3],y[i][4]];
					let j=i+1;
					if lhas(&trigs,&temp_trig)==false {			//branch
						trigs.push(temp_trig);
						let lng = trigs.len()-1;
						
						if goal(&x,ii,&y[y.len()-1],&all_groups) {
							v_count.push(0);
							v_count[lng]+= 1;
							l_count.push(1)
						}else{
							v_count.push(0);
							l_count.push(1);
							l_count[lng]+= 1;						
						}
					}else{
						if goal(&x,ii,&y[y.len()-1],&all_groups) {
							v_count[vvwhich(&trigs,temp_trig)[0]]+= 1;
						}else{
							l_count[vvwhich(&trigs,temp_trig)[0]]+= 1;
						}
					}
				}else{};
				light=light_m(lsum,lxsum.clone());
			}
		};
		//println!("got past iterating lore");
		
		//generate vectors of unique paths in order to count and assess. Currently commented out as unique paths are not used.
		let trigs_l=trigs.len();


		let mut l_v:Vec<f32>=Vec::new();
		let mut l_v_by_count:Vec<f32> = Vec::new();
		for i in 0..trigs_l{
			l_v_by_count.push((l_count[i] as f32)/((v_count[i]+l_count[i]) as f32));
		};
		
		
		//let all_paths_to_rome = v_paths.iter().fold(0,|acc,ref x| acc + x.len()); Path out.
		let total_victories = v_count.iter().fold(0,|acc,x| acc + x);
		let mut i_best_path:usize=0;    //Primary choice
		let mut i_best_count:usize=0;   //Alternative choice
		
		let mut best_act_sec:usize=1;	//Secondary choices.
		let mut best_tar_sec:usize=255;	//Secondary choice
		println!("total victories = {}",
									  //all_paths_to_rome,
									  total_victories);
		
		if total_victories<10*trigs.len() { //If there are no victories in normal parameters, make a set of less stringent tables.
			
			if !SWITCH_1 {
				println!("Survival strategy failed. Herp (all_paths = {})",total_victories);
				//println!("Got A");
				for number in 0..lore.len(){        //Make chosen_battles battles list.  ((lsum-7)<=xlsum) & (xlsum<=(lsum+7)) &
					let mut xlsum:i32=0;
					for i in 3..lore[number].len(){ // Reminder:(turn,turn,action,idm,ifast,light,x[0],x[1],x[2],etc)
						//Insert xlsum and orders script here.
						//NB this is a modified permissive script.
						if (lore[number][i][4]==ii8) & (lore[number][i][6+ii]==now[6+ii]){// if turn is yours and your state matches your state...				
							chosen_battles.push(&lore[number]);
							chosen_stats.push((lore[number][i][2],lore[number][i][3],all_groups[ii]));
							if goal(&x,ii,&lore[number][lore[number].len()-1],&all_groups) {vict_c_battles.push(&lore[number])}else{};
						}else{}
					};
				};
				//println!("Got B");

				for y in chosen_battles.iter(){
					let y_l=y.len()-1;
					if y[y_l][4]==ii8{
						last_c_stat.push([y[y_l][2],y[y_l][3],y[y_l][6+ii],now[6+(y[y_l][3] as usize)]]);
					}else{};
				};
				//println!("Got C");

				let mut lv_effects:Vec<[i16;23]>=Vec::new();
				if vict_c_battles.len()>0{
					for y in vict_c_battles.iter(){
						let y_l=y.len()-1;
						let mut temp_sub:[i16;23]=[0;23];
						for i in 0..23{temp_sub[i]=(now[5+i] as i16)-(y[y_l][5+i] as i16)};
						lv_effects.push(temp_sub);
						if y[y_l][4]==ii8{
							last_c_stat.push([y[y_l][2],y[y_l][3],y[y_l][6+ii],now[6+(y[y_l][3] as usize)]]);
						}else{};
					};
					lv_effects.remove(0);
					//println!("Got D");
					let mut uniq_cl_stat=uniq(&last_c_stat);
					let mut last_index=vec!([1,255,5,5]);
					//println!("Got Da");
					for y in uniq_cl_stat.into_iter(){
						if (y[2]==0)||(y[3]==0){}else{last_index.push(y)}
					};
					//println!("Got Db");
					let l_c_count:Vec<f32>=vcount(&last_index,&last_c_stat);
					let l_v_count:Vec<f32>=vcount(&last_index,&last_v_stat);
					//println!("Got Dc");
					let last_ratios:Vec<f32>=vf32ratio(&l_v_count,&l_c_count);
					//println!("Got DcII");
					let salvation=last_index[vnmaxi(&last_ratios)];
					//println!("Got Dd");
					best_act_pm=salvation[0] as usize;
					best_tar_pm=salvation[1] as usize;
			
					println!("Last ratios len: {}", last_ratios.len());
					println!("Last valid actions len: {}",last_index.len())
				}else{
					println!("No victories detected.");
					best_act_pm=1;
					best_tar_pm=255;
				};
			  
			//Experimental Engine for telling causes and consequences.
			//will actually be used to replace the above permissing thing.
			}else{
				
				println!("Building library of relevant causes");
				println!("length of all_causes = {}",all_causes.len());
				let mut my_causes_val:Vec<(&[u8],f32)> = Vec::with_capacity(1000);
				for cause in all_causes.iter() {
					if cause[2]==ii8 {my_causes_val.push((cause,0.0));};
				};
				
				let desired_goal:[Option<u8>;22] = perfect_goal(&x,ii,&all_groups);
				
				//Need the difference between now and ideal goal.
				//Then we need the effect which gets closest to this.
				let d_goal32:[Option<f32>;22] = f32_fy(&desired_goal);
				let ideal_effect:[Option<f32>;22] = idealise(&now,&d_goal32);
				
				rms_builder_cause_effect(&mut my_causes_val,&cause_effect_means,&ideal_effect);
				println!("Perfect goal: {:?}",desired_goal);
				println!("Status stat!: {:?}",&now[6..28]);
				for (i,&(x,e)) in my_causes_val.iter().enumerate() {
					println!("{}). Cause: {:?}. RMSs: {}",i,x,e);
				}
				let (action,idm) = minimal_cause(my_causes_val);
				println!("Ideal effect: {:?}",ideal_effect);
				println!("Chosen effect: {:?}",cause_effect_means.get(&[action as u8,idm as u8,ii8][..]));
				best_act_pm = action;
				best_tar_pm = idm;
			}; 
			
					
		}else{
			//i_best_path = vnmini(&l_v);
			i_best_count = vnmini(&l_v_by_count);
			best_act_pm = trigs[i_best_count][0] as usize;
			best_tar_pm = trigs[i_best_count][1] as usize;
		};
		//println!("Herp Derp's i_best_path = {}",i_best_path);
		//println!("Min L/V (path) = {} (number {})",l_v[vnmini(&l_v)],vnmini(&l_v));
		
		if best_tar_pm != 255 {
			println!("Min L/V (count)= {} (number {})",l_v_by_count[vnmini(&l_v_by_count)],vnmini(&l_v_by_count));
			if l_v_by_count[vnmaxi(&l_v_by_count)]==0.0 {
				println!("By count method has failed us. Going by unique paths.");
				best_act_pm = trigs[i_best_path][0] as usize;
				best_tar_pm = trigs[i_best_path][1] as usize;
			};
			//println!("got D");
			
			for i in 0..trigs.len(){println!("{}.) Trigs: {:?}, Ratio(paths,counts): ({})",
																							 i,
																							 &trigs[i],
																							 &l_v_by_count[i])};    
			if best_tar_pm<x.len(){
				println!("Best target: {} aka {} from group {}",best_tar_pm,x[best_tar_pm].0.name,x[best_tar_pm].1) 
			}else{
				println!("Survival strategy failure. Herp derp.")
			};
		};
	};	
	println!("Best action: {}",best_act_pm);                                 
	let t1=PreciseTime::now();	
	println!("Thinking time: {}",t0.to(t1));		                         
	//panic!("I want to pause here. FOREVER.");			
	(best_act_pm,best_tar_pm)                             //returns best action and best target.
}
