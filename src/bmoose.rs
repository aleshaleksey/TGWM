///bmoose is the module of TGWM which deals with battle logic.
///Some of this module shares bits with gmoose.
extern crate rand;
extern crate inflector;

use lmoose;
use lmoose::*;
use xmoose::*;
use imoose::{byteru16,permit_a};
use cmoose::*;
use gmoose::arcana_index_from_spell_name;

use inflector::Inflector;
use rand::Rng;
use std::option::{Option};
use std::f32;
use std::sync::mpsc::{Receiver,SyncSender};


//gmoose victory.
fn who_won_marker(){}
fn who_won(alive:&[bool;5])->(bool,Option<usize>){
	let mut live_count:u8 = 0;
	let out = (false,None);
	for i in 0..5 { 
		if alive[i] {live_count+= 1;};
	};
	if live_count==1 {
		for i in 0..5 {
			if alive[i] {return (true,Some(i));};
		};
		out
	}else{
		out
	}
}

fn ai_battle_rand_marker(){}
//Battle takes a vector of tuples of Lifeforms & their group...
//...as output by engenB
//battle_rand() is an automated random battle.
//it takes random_choice instead of dumb choice and has no player turn.
//it does not give exp and does not return new Lifeforms. This is important for implementation.
pub fn battle_rand(mut x:Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
                   y:&Vec<Spell>,
                   z:&Place,
                   ns:&Vec<String>,
                   battled:usize)->(Vec<[u8;28]>){
	//Set up recorder vector.
	let mut record:Vec<[u8;28]>= vec!([0;28],[255;28],[255;28]);
	let cms=x.len();
	for i in 0..cms{
		record[1][6+i]=x[i].0.id as u8;
		record[2][6+i]=x[i].1 as u8
	};

    let mut timer: Vec<f32>=vec!(0.0;cms);
    //let mut party=&x[0..y[0]];
    
    //create timer and initiate first turn.
    for nth in 0..cms{
		let tvar=(rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
        let time=x[nth].0.Speed_shade.clone()+tvar;
        let time=if time<1.0{1.0}else{time};
        timer[nth]=1.0/time
    };
    let mut fast=vnmin(timer.clone());
    let mut ifast:usize=vwhich_or(&timer,fast,0);
    //println!("timer:{:?}\nfast:{}\nifast:{}",timer,fast,ifast);
    let mut ttakes:Vec<usize> = vec![0;cms];
    
    //main battle loop.
    let mut tturns:u16=0;
    let mut omnicide:bool=false;
    while tturns<(cms*6+20) as u16{
		let ttturns=byteru16(tturns);
		//println!("Got minus A");
		let mut recl:[u8;28]=[
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x0
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
		for i in 0..cms{recl[6+i]=state_m(x[i].0.HP_shade,x[i].0.HP)};
		tturns+= 1;
		omnicide=all_dead(&x);
		if omnicide==true{
			record.push(recl.clone());
			break
		}else if state_m(x[ifast].0.HP,x[ifast].0.HP_shade)==0{
			let tvar:f32= (rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
			let time=x[ifast].0.Speed_shade.clone()+tvar;
			timer[ifast]+=1.0/time;
			let time=if time.is_normal()==true{time}else{5.0};
			fast=vnmin(timer.clone());
			ifast=vwhich_or(&timer,fast,ifast);
			record.push(recl);
			continue
		}else{
			let victory=genocide(&x,x[ifast].1);
			if victory==true{
				//record victory here!!
				break
			}else{
				ttakes[ifast]+= 1;
				let mut idm:usize=0;
				let mut dumbI:usize=0;			
				if ifast==0{
					idm = rand::thread_rng().gen_range(0,cms)
				}else{
					let mut liv_vec:Vec<usize> = Vec::with_capacity(25);
					for i in 0..cms{
						if state_m(x[i].0.HP,x[i].0.HP_shade)>0{liv_vec.push(i)}else{}
					};		
					let idm_pick = rand::thread_rng().gen_range(0,liv_vec.len());
					idm = liv_vec[idm_pick]
				};
				recl[3]=idm as u8;
				if x[ifast].0.Spellist.len()>0{
						dumbI=rand_choice(idm,&x[ifast].0,&ns[ifast],&x,&y,ifast);
				}else{
					dumbI = rand::thread_rng().gen_range(0,2)
				};
				
				//Record action choice (dumbI)!!
				if dumbI==1{
					//attack action	
					let attack_result:(f32,usize) =  attack_r(&mut x,
															  ifast,
															  idm);
					x[attack_result.1].0.HP_shade-= attack_result.0;
					if x[attack_result.1].0.HP_shade<=0.0 {
						for x in x.iter_mut() {
							if x.2[0].is_some() {
								if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
							};
						};
					};
                    //record target & battlefield state!!!
                }else if dumbI==0{
					//defence action
					x[ifast].2[0] = Some([idm,ttakes[ifast]]);
				}else if dumbI>1{
					//println!("magic idm:{} dI:{}",&idm,&dumbI);
					let dumbj=dumbI-2;
					if y[dumbj].MP>x[ifast].0.MP_shade {
						dumbI = 1;
						let attack_result:(f32,usize) =  attack_r(&mut x,
															  ifast,
															  idm);
						x[attack_result.1].0.HP_shade-= attack_result.0;
						if x[attack_result.1].0.HP_shade<=0.0 {
							for x in x.iter_mut() {
								if x.2[0].is_some() {
									if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
								};
							};
						};
					}else{
						let to_hit=sp_target_comp(&y[dumbj], &x, ifast, idm);
						let shades=magic_rand_battle(to_hit.clone(),&x,&ns,&y[dumbj],ifast);
						for z in 0..cms{
							let hpo=x[z].0.HP_shade;
							x[z].0.MP_shade+=shades.0[z].MP_shade;
							x[z].0.HP_shade+=shades.0[z].HP_shade;
							x[z].0.Speed_shade+= shades.0[z].S_shade;
							if x[z].0.Speed_shade<5.0{
								x[z].0.Speed_shade=5.0
							}else{};
							x[z].0.Attack_shade+=shades.0[z].A_shade;
							x[z].0.Defence_shade+=shades.0[z].D_shade;
							x[z].0.WM_shade+=shades.0[z].WM_shade;
							x[z].0.BM_shade+=shades.0[z].BM_shade;
							timer[z]+=shades.0[z].Timestop;
							if shades.0[z].Death==true{
								x[z].0.HP_shade=0.0
							}else{};
							if shades.0[z].Teleport==true{
								//record teleport!!!
								timer[z]=1000000.0
							}else{
							//record action choice & target+battlefield state!
							};
							//record target & battlefield state!!
							if x[z].0.HP_shade<=0.0 {
								for x in x.iter_mut() {
									if x.2[0].is_some() {
										if x.2[0].unwrap()[0]==z {x.2[0] = None;};
									};
								};
							};
						}
						recl[5] = (recl[5] as i32 + shades.1) as u8;
					};
				
				}else{};
				recl[2] = dumbI as u8;
				record.push(recl);
			};
		};
		
		//modulate special attacks
		if x[ifast].2[0].is_some() {
			if x[ifast].2[0].unwrap()[1]+1 < ttakes[x[ifast].2[0].unwrap()[0]] {x[ifast].2[0] = None;};
		};
		if x[ifast].2[1].is_some() {
			if x[ifast].2[1].unwrap()[1]+1 < ttakes[ifast] {x[ifast].2[1] = None;};
		};
		
		let tvar:f32= (rand::thread_rng().gen_range(1,10)+rand::thread_rng().gen_range(1,10)) as f32;
        let time=x[ifast].0.Speed_shade.clone()+tvar;
        let time=if time.is_normal()==true{time}else{5.0};
        timer[ifast]+=1.0/time;
//        for i in 0..cms{
//			if timer[i].is_normal()==true{
//				timer[i]=vnmax(timer.clone());
//			}else{};
//		};
        fast=vnmin(timer.clone());
        ifast=vwhich_or(&timer,fast,ifast);
	};	
    record
}

fn ai_battle_turn_marker(){}
//gmoose function for the AI turn in battle.
pub fn ai_battle_turn<'a,'b> ( mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						ns:&Vec<String>,
						p_loc: &Place,
						y: &Vec<Spell>,
						cms: usize,
						mut battle_fast:&mut f32,
						mut battle_ifast:&mut usize,
						mut battle_ttakes:&mut Vec<usize>,
						mut battle_tturns:&mut u16,
						mut battle_orders:&mut Vec<(u8,u8)>,
						mut battle_timer:&mut Vec<f32>,
						mut in_battle_record:&mut Vec<[u8;28]>,
						timer: usize,
						mut freeze_timer:&mut usize,
						mut comm_text: &mut String,
						mut pause: &mut bool,
						mut shaking_timer: &mut usize,
						mut shaking_dam: &mut [bool;25],
						mut ai_turn_started: &mut bool,
						mut ai_started_thinking: &mut bool,
						mut thought_sender_to_brain: &mut SyncSender<(usize,usize,[u8;28],i32,Vec<(u8,u8)>,bool)>,
						mut thought_receiver_to_body: &mut Receiver<(usize,usize)>,
						mut sprite_boxer: &mut GraphicsBox,
						sprite_pos: &mut [[f64;2];25],
						targets:&mut Vec<usize>
					  ){
	//println!("Monster number {} to go.",battle_ifast);
	
	let mut recl:[u8;28] = [0;28];
	if !*ai_turn_started {
		//if ai turn has not truly started, inititate recl and other important variables.
		*shaking_dam = [false;25];
		*targets = Vec::with_capacity(25);
		let ttturns=byteru16(*battle_tturns);
		//println!("Got minus A");
		recl = [
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			*battle_ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x0
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
			
		for i in 0..cms{recl[6+i]=state_m(encounter[i].0.HP_shade,encounter[i].0.HP)};
		*battle_tturns+= 1;
		in_battle_record.push(recl.clone());
		//NB genocide+omnicide loop moved outside of function.
		if encounter[*battle_ifast].0.HP_shade<=0.0 {
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+=1.0/time;
			let fast = vnmin(battle_timer.clone());
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			return
		}else{
			//start ai turn if ai is not dead.
			*ai_turn_started = true;
		};
	};
			
	if *ai_turn_started & !*ai_started_thinking {
		println!("{} from group {} is about to take action!",ns[*battle_ifast],encounter[*battle_ifast].1);
		let turning = *battle_tturns as usize;
		let mut lsum:i32=0;
		let mut battle_orders:Vec<(u8,u8)> = Vec::new();
		if *battle_tturns>1{
			println!("a:pa(t+1)={},(t+2)={},ibr.len()={}",permit_a(turning+1),turning+2,in_battle_record.len());					
			for j in permit_a(turning+1)..(turning+2){battle_orders.push((in_battle_record[j][4],0))};
			//println!("b");
			for i in 3..in_battle_record.len(){lsum+=in_battle_record[i][5] as i32 -128}
			//println!("c");
		}else{
			battle_orders=vec![(0,0)]
		};

		let bif:usize = *battle_ifast;
		let trng:usize = turning.clone();
		let rcl:[u8;28] = recl.clone();
		let lsm:i32 = lsum.clone();
		let borders:Vec<(u8,u8)> = battle_orders.clone();
		if (encounter[*battle_ifast].0.Type != MINDLESS)
		& (encounter[*battle_ifast].0.SubType != MINDLESS) {
			thought_sender_to_brain.clone().send((
								bif,
								trng,
								rcl,
								lsm,
								borders,
								true
			));
			println!("AI's thoughts sent");

		};
		*ai_started_thinking = true;
		return
	};
	if *ai_turn_started & *ai_started_thinking {
		
		if timer%5 != 0 {return}; //give the computer more time to think.
		
		//try to receive the answer, if failed go to next frame and try again, else continue turn.
		//(best_act_pm,best_tar_pm)
		let mut choice:(usize,usize,bool);
		
		if (encounter[*battle_ifast].0.Type != MINDLESS)
		 & (encounter[*battle_ifast].0.SubType != MINDLESS) {
			match thought_receiver_to_body.try_recv() {
				Ok(answer) => {
					let (choice_a,choice_b) = answer;
					choice = (choice_a,choice_b,true);
				},
				Err(_) => { choice = (0,0,false)},
			};
		}else{
			choice = (255,255,true);
		};
								
		if !choice.2 {return};
		println!("Answer: {:?}",choice);						
		let mut idm=choice.1;
		let mut exI=choice.0;
		
		let libr:usize = in_battle_record.len()-1;
		
		//AI SCRIPT FAILURE COMBINATION<<<<<<<<<<<
		if (idm>=encounter.len()) | (exI>y.len()+1) {
			
			println!("AI script failed, idm={}, exI={}.",idm,exI);			
			idm = dumb_target_chooser(idm,cms,encounter,*battle_ifast);
			
			if idm>= encounter.len() {
				println!("{} Panics!\n",&ns[*battle_ifast]);
				idm = bankai(*battle_ifast,&encounter)
			}else{};
			state_match(encounter[*battle_ifast].0.HP,
						encounter[*battle_ifast].0.HP_shade,
						&ns[*battle_ifast],
						encounter[*battle_ifast].1);
			let mut dumbI:usize=99999;		
			if encounter[*battle_ifast].0.Spellist.len()>0 {
				dumbI=dumb_choice(idm,*battle_ifast,&encounter[*battle_ifast].0,&ns[*battle_ifast],&encounter,&y);
				exI=dumbI;
				println!("{}",dumbI)
			}else{
				dumbI = rand::thread_rng().gen_range(0,2);
				exI = dumbI;	
				println!("{}!",dumbI)
			};				
			in_battle_record[libr][3]=idm as u8;
			in_battle_record[libr][2]=exI as u8;
			
		}else{};	
		if exI==1 {			
			//*comm_text = format!("{} from group {} attacks {} from group {}.", &ns[*battle_ifast],encounter[*battle_ifast].1, &ns[idm], &encounter[idm].1);	
			
			let attack_result:(f32,usize) =  attack(encounter,
												*battle_ifast,
												idm,
												&ns,
												comm_text);
			//damage and shaking.									
			if attack_result.0 > 0.0 {
				encounter[attack_result.1].0.HP_shade-= attack_result.0;
				if encounter[attack_result.1].0.HP_shade>0.0 {
					shaking_dam[attack_result.1] = true;
					//*shaking_timer = timer;
				};
			};
					
			if encounter[attack_result.1].0.HP_shade<=0.0 {
				for x in encounter.iter_mut() {
					if x.2[0].is_some() {
						if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
					};
				};
			};
			
			
			*sprite_boxer = GraphicsBox::Attack
								(SpriteBox::new
									(timer,
									&encounter[*battle_ifast],
									*battle_ifast,
									&encounter[idm],
									idm,
									&sprite_pos[*battle_ifast],
									&sprite_pos[idm],
									shaking_dam[attack_result.1])
							);
			shaking_dam[attack_result.1] = true;
												
		}else if exI==0 {
			*comm_text = format!("{} from group {} takes a stance between {} from group {} and the world!", &ns[*battle_ifast],encounter[*battle_ifast].1, &ns[idm], &encounter[idm].1);			
			encounter[idm].2[0] = Some([*battle_ifast,battle_ttakes[*battle_ifast]]);
		}else if exI >1 {
			//println!("magic idm:{} dI:{}",&idm,&dumbI);
			let exj=exI-2;
			if *battle_ifast==0{
				*comm_text = format!("You prepare to cast {} at {} from group {}.", &y[exj].name, &ns[idm], &encounter[idm].1)
			}else{
				*comm_text = format!("{} prepares to cast {} at {} from group {}.",&ns[*battle_ifast], &y[exj].name, &ns[idm], &encounter[idm].1)
			};
			if y[exj].MP > encounter[*battle_ifast].0.MP_shade {
				println!("\nenemy ran out of mana\n");
				in_battle_record[libr][2]=1;
				let spare_text:String = format!("{}\n...But doesn't have enough mana!", comm_text);		
				let attack_result:(f32,usize) =  attack(encounter,
													*battle_ifast,
													idm,
													&ns,
													comm_text);
				//damage and shaking.									
				if attack_result.0 > 0.0 {
				encounter[attack_result.1].0.HP_shade-= attack_result.0;
				if encounter[attack_result.1].0.HP_shade>0.0 {
					shaking_dam[attack_result.1] = true;
					*shaking_timer = timer;
				};
			};
				if encounter[attack_result.1].0.HP_shade<=0.0 {
					for x in encounter.iter_mut() {
						if x.2[0].is_some() {
							if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
						};
					};
				};
				*comm_text = format!("{}\n{}",spare_text,comm_text);
			}else{
				let to_hit=sp_target_comp(&y[exj], &encounter, *battle_ifast, idm);
				let shades=magic(to_hit.clone(),&encounter,&ns,&y[exj],*battle_ifast,comm_text);
				for z in 0..cms{
					let hpo=encounter[z].0.HP_shade;
					encounter[z].0.MP_shade+=shades.0[z].MP_shade;
					
					encounter[z].0.HP_shade+=shades.0[z].HP_shade;
					//Damage and avatar shaking.
					if (shades.0[z].HP_shade<0.0) & (encounter[z].0.HP_shade>0.0) {
						shaking_dam[z] = true;
						*shaking_timer = timer;
					};			
					encounter[z].0.Speed_shade+= shades.0[z].S_shade;
					if encounter[z].0.Speed_shade<5.0{
							encounter[z].0.Speed_shade=5.0
					}else{};
					encounter[z].0.Attack_shade+=shades.0[z].A_shade;
					encounter[z].0.Defence_shade+=shades.0[z].D_shade;
					encounter[z].0.WM_shade+=shades.0[z].WM_shade;
					encounter[z].0.BM_shade+=shades.0[z].BM_shade;
					battle_timer[z]+=shades.0[z].Timestop;
					if shades.0[z].Death==true{
						encounter[z].0.HP_shade=0.0
					}else{};
					if shades.0[z].Teleport==true{
						*comm_text = format!("{}\n...{} is warped off the battlefield.",comm_text,encounter[z].0.name);
						battle_timer[z]=1000000.0;
					};
					if (encounter[z].0.HP_shade<=0.0) & (hpo>0.0){
						*comm_text = format!("{}\n{} from group {} is slain by the spell.",
											 comm_text,&ns[z],encounter[z].1);
					};
					if encounter[z].0.HP_shade<=0.0 {
						for x in encounter.iter_mut() {
							if x.2[0].is_some() {
								if x.2[0].unwrap()[0]==z {x.2[0] = None;};
							};
						};
					};
				};
				
				//Fills the graphics box with the appropriate information,
				//to launch the sprites into tomorrow.
				sprite_box_filler(&y[exj],sprite_boxer,
					 &encounter[*battle_ifast],
					 *battle_ifast,
					 &to_hit,
					 targets,
					 &sprite_pos,
					 shaking_dam);
				
				in_battle_record[libr][5] = (in_battle_record[libr][5] as i32 + shades.1) as u8;
			};
		}else{};
		in_battle_record[libr][2]=exI as u8;
		println!("battle record turn end length ibr.len()={},turn={}\n",in_battle_record.len(),battle_tturns);	
	
	
		if encounter[*battle_ifast].2[0].is_some() {
			println!("x[ifast].2[0].is_some(), so shouldn't crash when unwrapping");
			if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[encounter[*battle_ifast].2[0].unwrap()[0]] {encounter[*battle_ifast].2[0] = None;};
		};
		if encounter[*battle_ifast].2[1].is_some() {
			if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
		};
		
		//modulate special attacks
			//switch of specials if more than 2 turns have passed.
		if encounter[*battle_ifast].2[0]!= None {
			if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[0] = None;};
		}else if encounter[*battle_ifast].2[1]!= None {
			if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
		};
		
		battle_ttakes[*battle_ifast]+= 1;
			
		let tvar:f32=
			(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
		let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
		battle_timer[*battle_ifast]+=1.0/time;
		let bt2 = battle_timer.to_owned();
		let fast = vnmin(bt2);
		*battle_ifast = vwhich(battle_timer,fast).unwrap_or(*battle_ifast);
		*pause = true;
		*comm_text = format!("{}\n***Press Enter to Continue***",comm_text);
		*ai_turn_started = false;
		*ai_started_thinking = false;
	};
}

fn player_battle_turn_marker(){}
//gmoose function for the player's turn in battle.
//NB do not switch n_s_l_q_f[4] off here. This is now fully regulated
//by the game_over() function.
pub fn player_battle_turn (mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						ns:&Vec<String>,
						p_loc: &Place,
						y: &Vec<Spell>,
						cms: usize,
						mut battle_fast:&mut f32,
						mut battle_ifast:&mut usize,
						mut battle_ttakes:&mut Vec<usize>,
						mut battle_tturns:&mut u16,
						mut battle_orders:&mut Vec<(u8,u8)>,
						mut battle_timer:&mut Vec<f32>,
						mut in_battle_record:&mut Vec<[u8;28]>,
						timer: usize,
						mut freeze_timer:&mut usize,
						mut yt_adcwpe_bw: &mut [bool;9],
						mut n_s_l_q_f: &mut [bool;7],
						mut recl: &mut [u8;28],
						mut comm_text: &mut String,
						mut sel_targets:&mut Vec<usize>,
						mut to_cast: &mut String,
						mut to_hit: &mut Vec<(bool,bool)>,
						mut pause: &mut bool,
						mut escape: &mut bool,
						mut shaking_timer: &mut usize,
						mut shaking_dam: &mut [bool;25],
						mut sprite_boxer: &mut GraphicsBox,
						sprite_pos: &mut [[f64;2];25],
						targets:&mut Vec<usize>) {

	let mut turn_over = false;
	if *yt_adcwpe_bw==[false,false,false,false,false,false,false,false,false] {
		*sel_targets = Vec::with_capacity(25);
		*targets = Vec::with_capacity(25);
		println!("Monster number {} to go.",battle_ifast);
		//If button may not be pressed and nothing pressed, 						
		let ttturns=byteru16(*battle_tturns);
		*shaking_dam = [false;25];
		//println!("Got minus A");
		*recl = [
			ttturns[0],									//0#turns
			ttturns[1],									//1#turns
			0,											//2#action
			0,											//3#idm
			*battle_ifast as u8,								//4#ifast
			128,										//5#light
			255,										//6#x[0]
			255,										//7#x[1]
			255,										//8#x[2]
			255,										//9#x[3]									
			255,										//10#x[4]
			255,										//11#x[5]
			255,										//12#x[6]
			255,										//13#x[7]
			255,										//14#x[8]
			255,										//15#x[9]
			255,										//16#x[10]
			255,										//17#x[11]
			255,										//18#x[12]
			255,										//19#x[13]
			255,										//20#x[14]
			255,										//21#x[15]
			255,										//22#x[16]
			255,										//23#x[17]
			255,										//24#x[18]
			255,										//25#x[19]
			255,										//26#x[20]
			255,										//27#x[21]
		];
		for i in 0..cms{recl[6+i]=state_m(encounter[i].0.HP_shade,encounter[i].0.HP)};
		*battle_tturns+= 1;
		if encounter[*battle_ifast].0.HP_shade<=0.0{
			//If player is dead, initiate the next turn.
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+= 1.0/time;
			let bt2 = battle_timer.to_owned();
			let fast = vnmin(bt2);
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			in_battle_record.push(recl.clone());
			return
		}else{
			//if player is not dead, enable button mashing.
			yt_adcwpe_bw[0]=true;
		};
	}else if *yt_adcwpe_bw==[true,false,false,false,false,false,false,false,false] {
		//If player may press button but no button pressed, set comm-text to remind player.
		*comm_text = format!("{}, it is time to act!\n(You are {})",&ns[*battle_ifast],sm_rets(&encounter[*battle_ifast].0));
	}else{
		//If button may be pressed and something may be pressed, calculate the consequences and prepare next turn.
		if yt_adcwpe_bw[1] {
			if sel_targets.len()==0 {
				*comm_text = format!("Who will you attack?\n");
			}else{
				let p_t:usize = sel_targets[0];
				println!("p_t: {}",p_t);
				
				*sel_targets = Vec::with_capacity(25);
				let attack_result:(f32,usize) =  attack(encounter,
												*battle_ifast,
												p_t,
												&ns,
												comm_text);
				if attack_result.0 > 0.0 {
					encounter[attack_result.1].0.HP_shade-= attack_result.0;
					if encounter[attack_result.1].0.HP_shade>0.0 {
						shaking_dam[attack_result.1] = true;
						//*shaking_timer = timer;
					};
				};
					
				//in_battle_record.push(recl.clone());
				turn_over = true;
				if encounter[attack_result.1].0.HP_shade<=0.0 {
					for x in encounter.iter_mut() {
						if x.2[0].is_some() {
							if x.2[0].unwrap()[0]==attack_result.1 {x.2[0] = None;};
						};
					};
				};
				
				*sprite_boxer = GraphicsBox::Attack
									(SpriteBox::new
										(timer,
										&encounter[*battle_ifast],
										*battle_ifast,
										&encounter[p_t],
										p_t,
										&sprite_pos[*battle_ifast],
										&sprite_pos[p_t],
										shaking_dam[attack_result.1]
				));
				shaking_dam[attack_result.1] = false;
				
			};
		}else if yt_adcwpe_bw[2] {
			if sel_targets.len()==0 {
				*comm_text = format!("Defence? How valiant. But whom will you defend?");
			}else{
				encounter[sel_targets[0]].2[0] = Some([*battle_ifast,battle_ttakes[*battle_ifast]]);
				*comm_text = format!("You place yourself between {} and the hostile world...",ns[sel_targets[0]]); 
				//in_battle_record.push(recl.clone());
				turn_over = true;
			};
		}else if yt_adcwpe_bw[4] { //make a wait script
			//in_battle_record.push(recl.clone());
			turn_over = true;
		}else if yt_adcwpe_bw[3] {
			if *to_cast==String::new() {
				*comm_text = "Which spell will you cast?".to_owned();
				yt_adcwpe_bw[7] = false;
				yt_adcwpe_bw[8] = false;
			}else{	
				//spell casting...This can go very wrong if strings in spell list do not equal spell names.
				let to_cast_ind:usize = arcana_index_from_spell_name(y,&to_cast).unwrap();
				if untarget_boon(&y[to_cast_ind]) {
					yt_adcwpe_bw[7] = true;
				};
				if untarget_woe(&y[to_cast_ind]) {
					yt_adcwpe_bw[8] = true;
				};
				if !yt_adcwpe_bw[7] {
					*comm_text = if y[to_cast_ind].target_woe==SAME {
						"Who will your spell target?".to_owned()
					}else{
						"Who will your receive your spell?".to_owned()
					};
					if sel_targets.len() > 0 {
						sp_targets_boon(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets);
						yt_adcwpe_bw[7] = true;
						if y[to_cast_ind].target_woe != SAME{
							*sel_targets = Vec::with_capacity(25);
						}else{
							sp_targets_woe(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets, *battle_ifast);
							*sel_targets = Vec::with_capacity(25);						
						};
					};
				}else if !yt_adcwpe_bw[8] {
						*comm_text = "Who will your spell target?".to_owned();
					if sel_targets.len() > 0 {
						sp_targets_woe(&y[to_cast_ind], &encounter, comm_text, to_hit, sel_targets, *battle_ifast);
						yt_adcwpe_bw[8] = true;
					};
				};
				
				if yt_adcwpe_bw[7] & yt_adcwpe_bw[8] {
					println!("{:?}",to_hit);
					sp_targets(&y[to_cast_ind], &encounter, &ns, *battle_ifast,to_hit);
					println!("{:?}",to_hit);
					let shades=magic(to_hit.clone(),&encounter,&ns,&y[to_cast_ind],*battle_ifast,comm_text);
					let hpo:f32=0.0;
					for z in 0..cms{
						let hpo=encounter[z].0.HP_shade;
						encounter[z].0.MP_shade+=shades.0[z].MP_shade;
						
						encounter[z].0.HP_shade+=shades.0[z].HP_shade;
						//Damage and avatar shaking.
						if (shades.0[z].HP_shade<0.0) & (encounter[z].0.HP_shade>0.0) {
							shaking_dam[z] = true;
							*shaking_timer = timer;
						};	
						encounter[z].0.Speed_shade+= shades.0[z].S_shade;
						if encounter[z].0.Speed_shade<5.0{
							encounter[z].0.Speed_shade=5.0
						}else{};
						encounter[z].0.Attack_shade+=shades.0[z].A_shade;
						encounter[z].0.Defence_shade+=shades.0[z].D_shade;
						encounter[z].0.WM_shade+=shades.0[z].WM_shade;
						encounter[z].0.BM_shade+=shades.0[z].BM_shade;
						battle_timer[z]+=shades.0[z].Timestop;
						if shades.0[z].Death==true{
							encounter[z].0.HP_shade=0.0
						}else{};
						if shades.0[z].Teleport==true{
						*comm_text = format!("{}\n...{} is warped off the battlefield.",comm_text,encounter[z].0.name);
						battle_timer[z]=1000000.0
						}else{};
						if encounter[z].0.HP_shade<=0.0 {
							for x in encounter.iter_mut() {
								if x.2[0].is_some() {
									if x.2[0].unwrap()[0]==z {x.2[0] = None;};
								};
							};
						};
						if (encounter[z].0.HP_shade<=0.0)
						 & (hpo>0.0){
							*comm_text = format!("{}\n{} from group {} is slain by the spell.",
									 comm_text,&ns[z],encounter[z].1)
						}else{};	
					};
					println!("Effects of spellcasting over");

					//Fills the graphics box with the appropriate information,
					//to launch the sprites into tomorrow.
					if y[to_cast_ind].MP<=encounter[*battle_ifast].0.MP_shade {
						sprite_box_filler(&y[to_cast_ind],sprite_boxer,
							 &encounter[*battle_ifast],
							 *battle_ifast,
							 &to_hit,
							 targets,
							 &sprite_pos,
							 shaking_dam
						);
					};
										
					//*sel_targets = Vec::with_capacity(25);
					*to_hit = vec![(false,false);cms];	
					yt_adcwpe_bw[3] = false;
					yt_adcwpe_bw[7] = false;
					yt_adcwpe_bw[8] = false;
					*to_cast = String::new();	
					
					recl[5] = (recl[5] as i32 + shades.1) as u8;
					//in_battle_record.push(recl.clone());
					turn_over = true;
					println!("Effects of spellcasting over");
				};
			}
		}else if yt_adcwpe_bw[6] {
			yt_adcwpe_bw[6] = false;
			//write an escape script.
			let mut toy_timer:Vec<f32> = vec![0.0;cms];
			let mut a = 0.0;
			let mut b = 0.0;
			for nth in 0..cms{
				a = (rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
				b = encounter[nth].0.Speed_shade.clone()+a;
				toy_timer[nth]=1.0/b;
			};
			let centrifuge = vnmin(toy_timer.clone());
			let escapee:usize = vwhich(&toy_timer,centrifuge).unwrap_or(0);
			if encounter[escapee].1==encounter[*battle_ifast].1 {
				*escape = true;
				println!("You have managed to flee from battle");
				*comm_text = "You have managed to flee from battle".to_owned();
				turn_over = false;
				*freeze_timer = timer-10;
				battle_ttakes[*battle_ifast]+=1;
			}else{
				println!("You tried to escape... But alas!");
				*comm_text = "You tried to escape... But alas!".to_owned();
				turn_over = true;
				println!("cms:{}, to_hitl:{}, battle_timerl:{}, encounterl:{}.",cms,to_hit.len(),battle_timer.len(),encounter.len());
				//in_battle_record.push(recl.clone());
			};
		}else{
			//in_battle_record.push(recl.clone());
			turn_over = true;
		};
		if turn_over {
			in_battle_record.push(recl.clone());
			*freeze_timer = timer-10;
			
			//switch off specials if more than 2 turns have passed.
			if encounter[*battle_ifast].2[0]!= None {
				println!("Got to defend switch offing: battle ttakes ifast: {}",battle_ttakes[*battle_ifast]);
				if encounter[*battle_ifast].2[0].unwrap()[1]+1 < battle_ttakes[encounter[*battle_ifast].2[0].unwrap()[0]] {encounter[*battle_ifast].2[0] = None;};
			}else if encounter[*battle_ifast].2[1]!= None {
				println!("Got to special switch offing");
				if encounter[*battle_ifast].2[1].unwrap()[1]+1 < battle_ttakes[*battle_ifast] {encounter[*battle_ifast].2[1] = None;};
			};
			
			battle_ttakes[*battle_ifast]+=1;
				
			let tvar:f32=
				(rand::thread_rng().gen_range(-10,11)-rand::thread_rng().gen_range(-10,11)) as f32;
			let time = encounter[*battle_ifast].0.Speed_shade.clone()+tvar;
			battle_timer[*battle_ifast]+= 1.0/time;
			let fast = vnmin(battle_timer.clone());
			*battle_ifast = vwhich(&battle_timer,fast).unwrap_or(*battle_ifast);
			*sel_targets = Vec::with_capacity(25);
			*to_hit = vec![(false,false);cms];	
			*yt_adcwpe_bw=[false,false,false,false,false,false,false,false,false];
			*pause = true;
			*comm_text = format!("{}\n***Press Enter to Continue***",comm_text);
		};
	};	
	//println!("Got to the end of player turn, next monster: {}",battle_ifast);		
}

fn game_over_marker(){}
//Function for pronouncing the end of the battle.
pub fn game_over(mut encounter: &mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				mut enemies:  &mut Vec<(Lifeform,usize)>,
				mut party: &mut Vec<(Lifeform,usize)>,
				dungeons: &mut Vec<Dungeon>,
				mut n_s_l_q_f:&mut [bool;7],
				mut tt_e_c_i_ll:&mut [bool;8],
				mut exp_players:&mut [f32; 5],
				mut battle_gold_pot:&mut usize,
				mut comm_text:&mut String,
				timer:usize,
				mut freeze_timer:&mut usize,
				battle_tturns:&mut u16,
				idungeon: &mut Option<usize>,
				dungeon_pointer: &mut usize,
				escape: bool) {
					
	//println!("entering game over");
	let max_group = if encounter.len()==0 {0}else{encounter[encounter.len()-1].1};
	let p_len = party.len();
	let mut alive = [false;5];
	let mut defeat = [true;5];				
	let mut omnicide = true;
	
	for x in encounter.iter() {
		if x.0.HP_shade>0.0 {
			defeat[x.1] = false;
			alive[x.1] = true;
			omnicide = false;
		};
	};
	
	let mut victory:(bool,Option<usize>)= if !omnicide | (*battle_tturns>600) | escape {who_won(&alive)}else{(false,None)};
	//println!("b2");
	if escape {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if !alive[0] {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "Your party has fallen in battle. Moose over.".to_owned();
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		for mut x in party.iter_mut() {
			x.0.Gold = 0;
			x.0.Exp = if x.0.Exp > x.0.ExpUsed + 10.0 {
				x.0.Exp-9.0
			 }else{
				x.0.ExpUsed
			}; 	
		};
	}else if omnicide {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "In the wake of this battle there are no survivors. Moose over.".to_owned();
		dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if *battle_tturns>600 {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		*comm_text = "The sun sets on your anger... And you live to rage another day.".to_owned();
		dungeon_navigator_a(true,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
	}else if victory.0 {
		n_s_l_q_f[4] = false;
		*freeze_timer = timer;
		let winner:String = match victory.1 {
			Some(x) => format!("Group {}",x),
			_ => "No one".to_owned(),
		};
		*comm_text = format!("{} has emerged victorious.",winner);
		if victory.1.unwrap()==0 {
			for (i,mut x) in party.iter_mut().enumerate() {
				x.0.Exp+= exp_players[i];
				x.0.Gold = *battle_gold_pot/p_len;
				println!("{}: has {} exp ({} unused)",x.0.name,x.0.Exp,x.0.ExpUsed); 	
			};
			dungeon_navigator_a(true,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		}else{
			for mut x in party.iter_mut() {
				x.0.Gold = 0;
				x.0.Exp = if x.0.Exp > x.0.ExpUsed + 10.0 {
					x.0.Exp-9.0
				 }else{
					x.0.ExpUsed
				}; 	
			};
			dungeon_navigator_a(false,idungeon,dungeon_pointer,dungeons,tt_e_c_i_ll);
		}; 
	};	
}

//makes level up quesiton menu appear on main menu.
fn lvlq_marker(){}
pub fn lvlq(party:&Vec<(Lifeform,usize)>,
			p_names:&Vec<String>,
			mut tt_e_c_i_ll: &mut [bool;8]) {
	for q in party.iter(){			
		if (q.0.Exp-q.0.ExpUsed)>10.0{
			tt_e_c_i_ll[5] = true;
			tt_e_c_i_ll[6] = true;
			return;
		};
	};
}


//function to produce battle ending bool (draw).
fn all_dead_marker(){}
fn all_dead(a:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)->bool{
	let mut omnicide:bool=true;
	for i in 0..a.len(){
		if a[i].0.HP_shade>0.0{omnicide=false
		}else{omnicide=omnicide};
	};
	omnicide
}

//function to produce battle ending bool (win).
fn genocide_marker(){}
fn genocide(a:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,b:usize)->bool{
	let mut victory:bool=true;
	for i in 0..a.len(){
		if (a[i].0.HP_shade>0.0) & (a[i].1!=b){victory=false
		}else{victory=victory};
	};
	victory
}

//dungeon navigator for advancing post battle
fn dungeon_navigator_a_marker(){}
fn dungeon_navigator_a (victory:bool,
						mut idungeon: &mut Option<usize>,
						mut dungeon_pointer: &mut usize,
						dungeons: &mut Vec<Dungeon>,
						mut tt_e_c_i_ll: &mut [bool;8]) {
	if idungeon.is_some() {
		if victory {
			if *dungeon_pointer < dungeons[idungeon.unwrap()].scenes.len()+3 {
				*dungeon_pointer+= 1;
			}else{
				*dungeon_pointer = 0;
				*idungeon = None;
			};
		}else{
			*idungeon = None;
			*dungeon_pointer = 0;
			tt_e_c_i_ll[2] = false;
			tt_e_c_i_ll[0] = true;
		};
	};
	//println!("dpoint: {}\nidun: {:?}",dungeon_pointer,idungeon);
}

//Standard attack function.
fn attack_marker(){}
fn attack(encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
			ifast:usize,
			p_t:usize,
			ns:&Vec<String>,
			mut comm_text:&mut String)->(f32,usize){
				
	let corpse:String = if encounter[p_t].0.HP_shade>0.0 {String::new()}else{"the corpse of ".to_owned()};
	if encounter[ifast].1==0 {
		*comm_text = format!("You prepare to attack {}{} from group {}.", corpse, &ns[p_t], &encounter[p_t].1)
	}else{
		*comm_text = format!("{} from group {} attacks {}{} from group {}.", &ns[ifast],encounter[ifast].1, corpse, &ns[p_t], &encounter[p_t].1)
	};
	let mut p_t:usize = p_t;
	
	//substitute victim for victim's defender.
	if encounter[p_t].2[0].is_some() {
		p_t = encounter[p_t].2[0].unwrap()[0];
		*comm_text = format!("\n{}...But {} from group {} stands in their path!",comm_text, &ns[p_t],encounter[p_t].1);
	};		
		
	
	//calculate if attack hits & whatnot.	  
	let aa=if encounter[ifast].0.Attack_shade <=2.0 {2} else {encounter[ifast].0.Attack_shade as isize};
	let ab=if encounter[ifast].0.Attack_shade <=0.0 {0.0} else {encounter[ifast].0.Attack_shade};
	let dd=if encounter[p_t].0.Defence_shade <=2.0 {2} else {encounter[p_t].0.Defence_shade as isize};
	let a=rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa);
	let b=rand::thread_rng().gen_range(-2,dd)+rand::thread_rng().gen_range(-2,dd);
	let mut HP_loss:f32 = 0.0;
	if (a<b) & (p_t != ifast) {
		*comm_text = format!("{}\n...and misses.",comm_text);
	}else{
		*comm_text = format!("{}\n...and the attack finds its mark.",comm_text);
		let att_pow:f32 = 1.5*ab*(1.0+encounter[ifast].0.Exp/1000.0);
				HP_loss = (rand::thread_rng().gen_range(-10,11)+rand::thread_rng().gen_range(-8,9)) as f32;
				HP_loss+= att_pow-encounter[p_t].0.Defence_shade/2.0;
				if p_t != ifast {HP_loss-= encounter[p_t].0.WM_shade/2.0;};
				HP_loss = if HP_loss<0.0 {
					println!("The strike is ineffective!");
					0.0
				}else{
					println!("Damage done: {}",&HP_loss);
					HP_loss
				};
	}
	dam_match(encounter[p_t].0.HP,
						  HP_loss,
						  &ns[ifast],
						  &ns[p_t],
						  encounter[p_t].1,
						  comm_text);
	if (encounter[p_t].0.HP_shade-HP_loss<=0.0) & !(encounter[p_t].0.HP_shade>0.0) {
		*comm_text = format!("{}\n{} falls to the attack.\n",comm_text, &encounter[p_t].0.name);
	};
	(HP_loss,p_t)	
}

//Automated attack function modernised.
fn attack_r_marker(){}
fn attack_r(encounter:&mut Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
			ifast:usize,
			p_t: usize)->(f32,usize){
				
	let mut p_t:usize = p_t;
	
	//substitute victim for victim's defender.
	if encounter[p_t].2[0].is_some() {
			p_t = encounter[p_t].2[0].unwrap()[0];
	};		  
	
	//calculate if attack hits & whatnot.
	let aa:isize = if encounter[ifast].0.Attack_shade <=2.0 {2} else {encounter[ifast].0.Attack_shade as isize};
	let ab:f32 = if encounter[ifast].0.Attack_shade <=0.0 {0.0} else {encounter[ifast].0.Attack_shade};
	let dd:isize = if encounter[p_t].0.Defence_shade <=2.0 {2} else {encounter[p_t].0.Defence_shade as isize};
	let a:isize = rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa)+rand::thread_rng().gen_range(-2,aa);
	let b:isize = rand::thread_rng().gen_range(-2,dd)+rand::thread_rng().gen_range(-2,dd);
	let mut HP_loss:f32 = 0.0;
	if a>=b {
		let att_pow:f32 = 1.5*ab*(1.0+encounter[ifast].0.Exp/1000.0);
				HP_loss = (rand::thread_rng().gen_range(-10,11)+rand::thread_rng().gen_range(-8,9)) as f32;
				HP_loss+= att_pow-encounter[p_t].0.Defence_shade/2.0-encounter[p_t].0.WM_shade/2.0;
				HP_loss = if HP_loss<0.0 {0.0}else{HP_loss};
	};
	(HP_loss,p_t)	
}


//Matches damage inflicted with correct statement...
//...And will eventually output answers for machine learning.
fn state_match(HP_tot:f32, HP_now:f32, d_name:&String, group:usize){
let stater:f32=HP_now/HP_tot;
let state:i32=
    if stater>=1.0{5}
	else if (stater<1.0) & (stater>=0.75){4}
	else if (stater<0.75) & (stater>=0.5){3}
	else if (stater<0.5) & (stater>=0.25){2}
	else if(stater<0.25) & (stater>0.0){1}
	else{0};
let damned:&str= match state{
	5=>"uninjured",
	4=>"lightly wounded",
	3=>"wounded",
	2=>"gravely wounded",
	1=>"on death's door",
	0=>"dead",
	_=>"on another plane of existence"};	
println!("{} from group {} is {}.\n",
         &d_name,
         group,
         damned);
}


//This is the magic function. It will get complicated
//It takes the target list.
fn magic_marker(){}
fn magic(tl:Vec<(bool,bool)>,xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ns:&Vec<String>,spell:&Spell, ifast:usize, mut comm_text:&mut String)
->(Vec<Shade>,i32){
	
	//Tell us what's happening, or chant special spell.
	if spell.name=="Inferno"{
		*comm_text = "O Pyros! O primal flame...\nWherein the world was forged...\nBurn up Everything!\n".to_owned();
	}else if spell.name=="Jovian Lightning"{
		*comm_text = "O spear of Theos! O spark of Life!\nTouch the earth!\nRevoke your gift!\n".to_owned();
	}else if spell.name=="True Crystallise"{
		*comm_text = "O star of the North!\n O cross of the south!\nUnite...\n...And pierce the world with stillness!\n".to_owned();
	}else if spell.name=="Sword of Perseus"{
		*comm_text = "O spirit of the hero,\n Come forth to soulless steel!\n...Make right the gods' mistakes!\n".to_owned();
	}else{
		*comm_text = if xx[ifast].1==0 {
			format!("You cast {}...",spell.name)
		}else{
			let a = match xx[ifast].1 {
				1 => "the north",
				2 => "the east",
				3 => "the south",
				_ => "the north",
			};
			format!("{} from {} casts {}...",xx[ifast].0.name,a,spell.name)
		};
	};
	
	//initiate output and other vectors.
	let tll=tl.len();
	let shay=Shade::new();
	let mut shades:Vec<Shade>=vec!(shay.clone();tll);
	let mut escaped_reaper:usize=0;
	let mut trapped_reaper:usize=0;
	
	//determine strength of caster effects.
	if (spell.MP>xx[ifast].0.MP_shade) & (xx[ifast].1==0){
		*comm_text = format!("{}\n...You don't have enough mana!",comm_text);
		return (shades,0)
	}else{};
	
	let caff=if (xx[ifast].0.SubType==spell.Type)
			 || (xx[ifast].0.Affinity==spell.Type){1.8
			 }else{1.3};
	
	let CM= if spell.Light==true {
		(xx[ifast].0.WM_shade+spell.Power-10.0)*caff
	}else{
		(xx[ifast].0.BM_shade+spell.Power-10.0)*caff
	};
	let BM= xx[ifast].0.BM_shade as i64;
	let BM=if BM<1 {1}else{BM};
	
	
	if spell.Death==true{
		*comm_text = "This forbidden magic summons a reaper...".to_owned();
	}else{};
	
	for i in 0..tll{
		let WMi=xx[i].0.WM as i64;
		let WMi=if WMi<1 {1} else {WMi};
		let mut extra_dam:f32=0.0;
		if spell.Health>0.0{
			extra_dam=CM-xx[i].0.WM*0.8;
			extra_dam=if extra_dam<0.0{0.0}else{extra_dam};
		}else{};
		
		//Harming script.
		if (tl[i].1==true) & ((spell.Light==false)||
		((xx[i].0.Unclean==true)||(xx[i].0.Alive==false))){
			let PW=spow(spell.Power,CM,&xx[i].0,&spell);
			let PW=if PW<=0.0{0.0}else{PW};
			//harming script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade-= if spell.Light==false{PW*spell.Health*0.06+extra_dam
								 }else if (spell.Light==true)
								      & ((xx[i].0.Unclean==true)
								      || (xx[i].0.Alive==false)){PW*spell.Health*0.08+extra_dam
								 }else{0.0};
			shades[i].S_shade-=PW*0.005*spell.Speed;
			shades[i].A_shade-=PW*0.02*spell.Attack;
			shades[i].D_shade-=PW*0.02*spell.Defence;
			shades[i].WM_shade-=PW*0.02*spell.WM;
			shades[i].BM_shade= if xx[i].0.Unclean==true{
				                    0.0+PW*0.02*spell.BM
				                }else{0.0-PW*0.02*spell.BM};
			shades[i].Inv_shade= Vec::new();
			if spell.Death==true{
			let a=rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM);
			let b=rand::thread_rng().gen_range(-1,WMi)+rand::thread_rng().gen_range(-1,WMi);
			*comm_text = format!("{}\n...The summoned reaper reaches for {}!",comm_text,ns[i]);
				if (a>=b) & (xx[i].0.Type!=UNDEAD) {
					trapped_reaper+=1;
					shades[i].Death=true;
					*comm_text = format!("{}\n...And takes its soul.",comm_text)
				}else{
					escaped_reaper+=1;
					*comm_text = format!("{}\n...but {} repels it.",comm_text,ns[i])
				};
			}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			shades[i].Timestop=
				if (spell.Timestop==true) &
					((i==ifast)||(i==xx[i].1)){
					xx[ifast].0.WM_shade.powf(1.0+spell.Power/10.0)*0.001
					}else if spell.Timestop==true {PW*0.001
				}else{0.0};
			if shades[i].Timestop<0.0{
			shades[ifast].Timestop-=shades[i].Timestop;
			shades[i].Timestop=0.0}else{};
		};
		
		//Healing script.
		if (tl[i].0==true) & (((spell.Light==true) &
		(xx[i].0.Alive==true) )|| ((spell.name=="Lifestealer")
		|| (trapped_reaper>0))){
			let PW:f32=spow(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=spowl(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=if LPW<0.0{0.0}else{LPW};
			let PW:f32=if PW<0.0{0.0}else{PW};
			//healing script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade+= if ((xx[i].0.HP_shade>0.0)
			                      & (xx[i].0.Alive==true)
			                      & (xx[i].0.HP>xx[i].0.HP_shade))
			                     || (spell.name=="Lifestealer"){
				spell.Health*CM*(10.0+spell.Power)*0.01
			}else if (xx[i].0.HP_shade>0.0)
			       & (xx[i].0.Alive==false)
			       & (spell.name!="Lifestealer"){
				0.0-PW*spell.Health*0.05
			}else{0.0};
			shades[i].S_shade+= PW*0.004*spell.Speed;
			if shades[i].S_shade<0.0{shades[i].S_shade=0.0}else{};
			shades[i].A_shade+= PW*0.01*spell.Attack;
			shades[i].D_shade+= PW*0.01*spell.Defence;
			shades[i].WM_shade+= PW*0.005*spell.WM;
			shades[i].BM_shade+= if xx[i].0.Unclean==true{
				                    -PW*0.005*spell.BM
				                }else{CM*0.005*spell.BM};
			
			if (spell.Death==true) & (trapped_reaper>0)
			 & (xx[i].0.HP_shade<=0.0){
				println!("The reaper returns what it took...");
				shades[i].HP_shade+=xx[i].0.HP-xx[i].0.HP_shade;
				trapped_reaper-=1}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			}else{};
			
		//Print Messages		
		if shades[i].HP_shade<0.0{
			let affected:String= if xx[i].0.HP_shade<=0.0 {
				format!("{}'s remains",ns[i])
			}else{
				ns[i].to_owned()
			};
			*comm_text = format!("{}\n{} {}",
					 comm_text,
			         convert_mag_type(spell.Type).to_title_case(),
			         spd(affected,shades[i].HP_shade));
			println!("Damage to {}: {}.",ns[i],0.0-shades[i].HP_shade);
		}else if shades[i].HP_shade>0.0{
			*comm_text = format!("{}\n{} is healed by the spell.",comm_text,ns[i]);
		}else{};
		if (shades[i].S_shade<0.0)||
		   (shades[i].A_shade<0.0)||
		   (shades[i].D_shade<0.0)||
		   (shades[i].WM_shade<0.0)||
		   (shades[i].BM_shade<0.0){
			*comm_text = format!("{}\nThe spell ails {}.",comm_text,ns[i]);
		}else if (shades[i].S_shade>0.0)||
		   (shades[i].A_shade>0.0)||
		   (shades[i].D_shade>0.0)||
		   (shades[i].WM_shade>0.0)||
		   (shades[i].BM_shade>0.0){
			*comm_text = format!("{}\nThe spell strengthens {}.",comm_text,ns[i]);
		}else{};
		shades[i].WM_shade+=(spell.Illumination as f32)*0.01*CM;
		shades[i].BM_shade-=(spell.Illumination as f32)*0.01*CM;			
		if shades[i].Timestop>0.0{
			*comm_text = format!("{}\n{}'s time stops.",comm_text,ns[i])
		}else{};
//		println!("{}: {},{},{},{},{},{},{},{},{}.",
//				 xx[i].0.name,
//				 shades[i].HP_shade,
//				 shades[i].S_shade,
//				 shades[i].A_shade,
//		 		 shades[i].D_shade,
//				 shades[i].WM_shade,
//				 shades[i].BM_shade,
//				 shades[i].Death,
//				 shades[i].Teleport,
//				 shades[i].Timestop);
				 
	};
	//loop finished.
	
	shades[ifast].MP_shade=0.0-spell.MP;
	if spell.Illumination>20{
		*comm_text = format!("{}\nA divine light descends onto the battlefield.",comm_text)
	}else if spell.Illumination>10{
		*comm_text = format!("{}\nThe spell illuminates the battlefield with a brilliant light.",comm_text)
	}else if spell.Illumination>0{
		*comm_text = format!("{}\nThe spell illuminates the battlefield.",comm_text)
	}else if spell.Illumination<0{
		*comm_text = format!("{}\nThe spell darkens the battlefield.",comm_text);
	}else if spell.Illumination<(-9){
		*comm_text = format!("{}\nThe spell casts menacing shadows on the battlefield.",comm_text);
	}else if spell.Illumination<(-19){
		*comm_text = format!("{}\nThe battlefield is plunged into Abyssal darkness.",comm_text);
    }else{};
	if escaped_reaper>0{
		*comm_text = format!("{}\nThe turned reaper turns on its master and steals its soul.",comm_text);
		shades[ifast].Death=true
	}else{};
	
	for i in 0..xx.len() {
		println!("SPF on {}: HP: {}. MP: {}. Speed: {}. Attack {}. Defence {}. WM: {}. BM {}.",
				 xx[i].0.name,shades[i].HP_shade,
							shades[i].MP_shade,
							shades[i].S_shade,
							shades[i].A_shade,
							shades[i].D_shade,
							shades[i].WM_shade,
							shades[i].BM_shade)
	};
		
	(shades,spell.Illumination)
}

fn magic_rand_marker(){}
fn magic_rand_battle(tl:Vec<(bool,bool)>,xx:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,ns:&Vec<String>,spell:&Spell, ifast:usize)
->(Vec<Shade>,i32){	
	let tll=tl.len();
	let mut shay= Shade::new();
	
	let mut shades:Vec<Shade>=vec!(shay.clone();tll);
	let mut escaped_reaper:usize=0;
	let mut trapped_reaper:usize=0;
	
	if (spell.MP>xx[ifast].0.MP_shade) & (ifast==0){
		//record mana out!;
		return (shades,0)
	}else{};
	
	let caff=if (xx[ifast].0.SubType==spell.Type)
			 || (xx[ifast].0.Affinity==spell.Type){1.8
			 }else{1.3};
	
	let CM= if spell.Light==true {
		(xx[ifast].0.WM_shade+spell.Power-10.0)*caff
	}else{
		(xx[ifast].0.BM_shade+spell.Power-10.0)*caff
	};
	let BM= xx[ifast].0.BM_shade as i64;
	let BM=if BM<1 {1} else {BM};
	
	if spell.Death==true{
	}else{};
	
	for i in 0..tll{
		let WMi=xx[i].0.WM as i64;
		let WMi=if WMi<1 {1} else {WMi};
		let mut extra_dam:f32=0.0;
		if spell.Health>0.0{
			extra_dam=CM-xx[i].0.WM*0.8;
			extra_dam=if extra_dam<0.0{0.0}else{extra_dam};
		}else{};
		
		//Harming script.
		if (tl[i].1==true) & ((spell.Light==false)||
		((xx[i].0.Unclean==true)||(xx[i].0.Alive==false))){
			let PW=spow(spell.Power,CM,&xx[i].0,&spell);
			let PW=if PW<=0.0{0.0}else{PW};
			//harming script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade-= if spell.Light==false{PW*spell.Health*0.06+extra_dam
								 }else if (spell.Light==true)
								      & ((xx[i].0.Unclean==true)
								      || (xx[i].0.Alive==false)){PW*spell.Health*0.08+extra_dam
								 }else{0.0};
			shades[i].S_shade-=PW*0.005*spell.Speed;
			shades[i].A_shade-=PW*0.02*spell.Attack;
			shades[i].D_shade-=PW*0.02*spell.Defence;
			shades[i].WM_shade-=PW*0.02*spell.WM;
			shades[i].BM_shade= if xx[i].0.Unclean==true{
				                    0.0+PW*0.02*spell.BM
				                }else{0.0-PW*0.02*spell.BM};
			shades[i].Inv_shade= Vec::new();
			if spell.Death==true{
			let a=rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM)+rand::thread_rng().gen_range(-1,BM);
			let b=rand::thread_rng().gen_range(-1,WMi)+rand::thread_rng().gen_range(-1,WMi);
				if (a>=b) & (xx[i].0.Type!=UNDEAD) {
					trapped_reaper+=1;
					shades[i].Death=true;
				}else{
					escaped_reaper+=1;
				};
			}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true
					}else{false};
			shades[i].Timestop=
				if (spell.Timestop==true) &
					((i==ifast)||(i==xx[i].1)){
					xx[ifast].0.WM_shade.powf(1.0+spell.Power/10.0)*0.001
					}else if spell.Timestop==true {PW*0.001
				}else{0.0};
			if shades[i].Timestop<0.0{
			shades[ifast].Timestop-=shades[i].Timestop;
			shades[i].Timestop=0.0}else{};
		};
		
		//Healing script.
		if (tl[i].0==true) & (((spell.Light==true) &
		(xx[i].0.Alive==true) )|| ((spell.name=="Lifestealer")
		|| (trapped_reaper>0))){
			let PW:f32=spow(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=spowl(spell.Power,CM,&xx[i].0,&spell);
			let LPW:f32=if LPW<0.0{0.0}else{LPW};
			let PW:f32=if PW<0.0{0.0}else{PW};
			//healing script.
			shades[i].MP_shade= 0.0;
			shades[i].HP_shade+= if ((xx[i].0.HP_shade>0.0)
			                      & (xx[i].0.Alive==true)
			                      & (xx[i].0.HP>xx[i].0.HP_shade))
			                     || (spell.name=="Lifestealer"){
				spell.Health*CM*(10.0+spell.Power)*0.01
			}else if (xx[i].0.HP_shade>0.0)
			       & (xx[i].0.Alive==false)
			       & (spell.name!="Lifestealer"){
				0.0-PW*spell.Health*0.05
			}else{0.0};
			shades[i].S_shade+= PW*0.004*spell.Speed;
			if shades[i].S_shade<0.0{shades[i].S_shade=0.0}else{};
			shades[i].A_shade+= PW*0.01*spell.Attack;
			shades[i].D_shade+= PW*0.01*spell.Defence;
			shades[i].WM_shade+= PW*0.005*spell.WM;
			shades[i].BM_shade+= if xx[i].0.Unclean==true{
				                    -PW*0.005*spell.BM
				                }else{CM*0.005*spell.BM};
			
			if (spell.Death==true) & (trapped_reaper>0)
			 & (xx[i].0.HP_shade<=0.0){
				shades[i].HP_shade+=xx[i].0.HP-xx[i].0.HP_shade;
				trapped_reaper-=1}else{};
			shades[i].Teleport=
				if (spell.Teleport==true) &
				   (((i==ifast)||(i==xx[i].1))||PW>0.0){true}else{false};
		}else{};
			
		//Print Messages	
		shades[i].WM_shade+=(spell.Illumination as f32)*0.01*CM;
		shades[i].BM_shade-=(spell.Illumination as f32)*0.01*CM;	
		
//		println!("{}: {},{},{},{},{},{},{},{},{}.",
//				 xx[i].0.name,
//				 shades[i].HP_shade,
//				 shades[i].S_shade,
//				 shades[i].A_shade,
//		 		 shades[i].D_shade,
//				 shades[i].WM_shade,
//				 shades[i].BM_shade,
//				 shades[i].Death,
//				 shades[i].Teleport,
//				 shades[i].Timestop);
				 
	};
	//loop finished.
	shades[ifast].MP_shade=0.0-spell.MP;
	if escaped_reaper>0{shades[ifast].Death=true}else{};
		
	(shades,spell.Illumination)
}

//
//
//Spellcasting shell which determined which effects affect whom...
//...and in what order.
//
//Spell targeting part A.1
fn untarget_boon(spell:&Spell)-> bool {
	match spell.target_boon {
		SELF => true,
		BOB  => true,
		ALL  => true,
		PARTY => true,
		ANY => true,
		_      => false,
	}
}

//Spell targeting part A.2
fn untarget_woe(spell:&Spell)-> bool {
	match spell.target_woe {
		SELF => true,
		SAME => true,
		BOB  => true,
		ALL => true,
		ANY  => true,
		PARTY=> true,
		_      => false,
	}
}


//transfers targets to the to_hit vector (boon).
fn sp_targets_boon(spell: &Spell,
				  encounter: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				  mut comm_text: &mut String,
				  mut to_hit: &mut Vec<(bool,bool)>,
				  tl: &Vec<usize>) {
	
	match spell.target_boon {
		TARGET|GROUP|SINGLE => {to_hit[tl[0]].0 = true},
		GROUPS|ANY => 		{ let group = encounter[tl[0]].1;
							  for i in 0..encounter.len() {
								if encounter[i].1==group {
									to_hit[i].0 = true
								}; 
							  };
							},
		_ => {},
	};
}

//transfers targets to the to_hit vector (woe). MUST BE USED AFTER BOON.
fn sp_targets_woe(spell: &Spell,
				  encounter: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				  mut comm_text: &mut String,
				  mut to_hit: &mut Vec<(bool,bool)>,
				  tl: &Vec<usize>,
				  ifast:usize) {
	
	match spell.target_woe {
		TARGET|SINGLE => {to_hit[tl[0]].1 = true},
		GROUP => { let group = encounter[tl[0]].1;
							  for i in 0..encounter.len() {
								if encounter[i].1==group {
									to_hit[i].1 = true
								}; 
							  };
							},
		ANY|GROUPS => {	for i in 0..encounter.len() {
						if encounter[i].1!=encounter[ifast].1 {
							to_hit[i].1 = true
						};
					};
				 },
		SAME => {	for x in to_hit.iter_mut() {x.1 = x.0};
				  },
		_ => {},
	};
}

fn sp_targets(spell:&Spell, t_l: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				ns:&Vec<String>,
				ifast:usize,
				targets:&mut Vec<(bool,bool)>){
	
	//Set up variables.
	let tll=t_l.len() as usize;
	let mut target_boon:char='x';
	let mut target_woe:char='x';
	
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).

	if  spell.target_boon==NON{
		for x in targets.iter_mut() {x.0 = false};
	}else if spell.target_boon==SELF{
		for x in targets.iter_mut() {x.0 = false};
		targets[ifast].0=true;
		println!("Your spell aids only you ourself.");
	}else if spell.target_boon==ALL{
		for i in 0..tll{
			targets[i].0=true
		};
		println!("This spell is received by all.");
	}else if spell.target_boon==PARTY{
		for x in targets.iter_mut() {x.0 = false};
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].0=true
			}else{};
		};
		println!("This spell is received by all.");
	}else{
	};
	
//create target lists (BM).
//create target lists (BM).
//create target lists (BM).
    
	if spell.target_woe==NON{
		for x in targets.iter_mut() {x.1 = false};
	}else if spell.target_woe==SAME{
		for i in 0..targets.len() {targets[i].1 = targets[i].0};
	}else if spell.target_woe==SELF{
		for x in targets.iter_mut() {x.1 = false};
		targets[ifast].1 = true;
		println!("This spell hurts only yourself");
	}else if spell.target_woe==ALL {
		for x in targets.iter_mut() {x.1 = true};
		println!("This spell targets all combatants.");
	}else if (spell.target_woe==ANY) | (spell.target_woe==GROUPS) {
		for i in 0..targets.len() {
			if t_l[i].1 != 0 {targets[i].1 = true}
		};
		println!("This spell targets all combatants.");
	}else{
	};
}

//
//
//
//Spell targeting for computer.
fn sp_target_comp(spell:&Spell, t_l: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, ifast:usize,idm:usize)->Vec<(bool,bool)>{
	
	//Set up variables.
	let tll=t_l.len() as usize;
	let mut targets: Vec<(bool,bool)>=vec![(false,false);tll];
	
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
//create target lists (WM).
	let mut boon_chosen=false;
	while boon_chosen==false{
	if spell.target_boon==NON{
		targets=targets;
		boon_chosen=true
	}else if spell.target_boon==SELF{
		targets[ifast].0=true;
		boon_chosen=true;
	}else if spell.target_boon==ALL||(spell.target_boon==ANY){
		for i in 0..tll{
			targets[i].0=true
		};
		boon_chosen=true
	}else if spell.target_boon==PARTY{
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].0=true
			}else{};
		};
		boon_chosen=true
	}else if (spell.target_boon==SINGLE)|(spell.target_boon==TARGET){
		targets[idm].0=true;
        boon_chosen=true;
	}else if (spell.target_boon==GROUP)||(spell.target_boon==GROUPS){
		let group=t_l[ifast].1;
		for i in 0..tll{
			if t_l[i].1==group{
				targets[i].0=true
			}else{};
		};
		boon_chosen=true
	}else{boon_chosen=true};
	}
//create target lists (BM).
//create target lists (BM).
//create target lists (BM).
	let mut woe_chosen=false;	
    while woe_chosen==false{
	if spell.target_woe==NON {
		targets=targets;
		woe_chosen=true
	}else if spell.target_woe==SELF {
		targets[ifast].1=true;
		woe_chosen=true
	}else if spell.target_woe==ALL {
		for i in 0..tll{
			targets[i].1=true
		};
		woe_chosen=true
	}else if spell.target_woe==PARTY {
		for i in 0..tll{
			if t_l[i].1==t_l[ifast].1{
			targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else if (spell.target_woe==SINGLE)|(spell.target_woe==TARGET) {
		targets[idm].1=true;
        woe_chosen=true;
	}else if spell.target_woe==GROUP {
		let group=t_l[idm].1;
		for i in 0..tll{
			if t_l[i].1==group{
				targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else if (spell.target_woe==GROUPS)||(spell.target_woe==ANY){
		let group=t_l[ifast].1;
		for i in 0..tll{
			if t_l[i].1!=group{
				targets[i].1=true
			}else{};
		};
		woe_chosen=true
	}else{woe_chosen=true};
	}

targets
}

//legacy algorithm for choosing a target (ie, attack the weakest enemy).
fn dumb_choice_a_marker(){}
fn dumb_target_chooser_legacy(idm:usize,				//target id.
							  cms:usize,				//encounter len.
							  encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,	//encounter
							  battle_ifast:usize) -> usize {
								  
	let mut dngrrs: Vec<f32>=vec!(0.0;cms);
	for d in 0..cms{dngrrs[d]=dangerous(&encounter[d].0,&encounter[battle_ifast].0)};
	let mut dngm:f32=0.0;
	let mut min:f32=f32::MAX;	
	for j in 0..dngrrs.len(){
		if (min>=dngrrs[j])
		 & (encounter[battle_ifast].1!=encounter[j].1)
		 & (encounter[j].0.HP_shade>0.0) {
			 min = dngrrs[j];
		 };
	};
	dngm=min;
	vwhich(&dngrrs,dngm).unwrap_or(idm)
}

//Better target picking algorithm:
//Find most dangerous group for YOU and attack quishiest member.
//NB currently silly as does not compare BM vs WM, attack vs defence.
fn dumb_choice_b_marker(){}
fn dumb_target_chooser (idm:usize,
						cms:usize,
						encounter:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						battle_ifast:usize) -> usize {
	
	//initiate vectors.
	let mut squish: Vec<f32> =  Vec::with_capacity(cms);
	let mut dngr_grs: [f32;5] = [0.0;5];
	
	//assign danger ratings and quish
	for x in encounter.iter() {
		dngr_grs[x.1]+= dangerous(&x.0,&encounter[battle_ifast].0);
		squish.push(x.0.Defence_shade+x.0.WM_shade);
	};
	
	//assign group danger ratings.
	let mut target_gr:usize = 0;
	let mut highest_danger:f32 = 0.0;
	
	for (i,x) in dngr_grs.iter().enumerate() {
		if (*x>highest_danger) & (i != encounter[battle_ifast].1) {
			highest_danger = *x;
			target_gr = i;
		};
	};
	
	//pick squishiest from dangerous group.
	let mut min:f32 = f32::MAX;	
	for (def,mon) in squish.iter().zip(encounter.iter()){
		if (min>=*def)
		 & (encounter[battle_ifast].1 != mon.1)
		 & (mon.1==target_gr)
		 & (mon.0.HP_shade>0.0) {
			 min = *def
		 };
	};
	
	vwhich(&squish,min).unwrap_or(idm)
}

//choosing an action for the computer using a "simple" algorithm.
fn dumb_choice (idm:usize,
				ifast:usize,
				caster:&Lifeform,
				c_name:&String,
				pots: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
				spells:&Vec<Spell>)-> usize {
					
	let mut attackq = true;
	let mut spn:Vec<i8> = Vec::new();
	let mut permitted:Vec<usize> = Vec::new();
	let mut permitted_arcana:Vec<&Spell> = Vec::new();
	let mut choice:usize=1;
	for i in 0..spells.len(){
		spn.push(spells[i].id)
	};
	//println!("Point A");
	//println!("{}",spn.len());
	for i in 0..spn.len(){
		if lmoose::lhas(&caster.Spellist,&spn[i]) & (spells[i].MP<=caster.MP_shade) {
			attackq=false;
			permitted.push(i);
			permitted_arcana.push(&spells[i]);
		};
	};
    //println!("Point B");
	if attackq==true{
		if pots[idm].1==pots[ifast].1 {0}else{1};
	}else{
		if caster.HP_shade/caster.HP<0.33 {
			for i in 0..permitted_arcana.len() {
				if permitted_arcana[i].Type==HEALING {
					return permitted[i]+2
				}
			};
		};
		
		//compare what'll do more damage, spell or attack.
		let ab:f32 = if pots[ifast].0.Attack_shade <=0.0 {0.0} else {pots[ifast].0.Attack_shade};
		let mut mean_pot_dam = 1.5*ab*(1.0+pots[ifast].0.Exp/1000.0); - pots[idm].0.Defence_shade/2.0 - pots[idm].0.WM_shade/2.0;
		
		for i in 0..permitted_arcana.len() {
			let caff=if (pots[ifast].0.SubType==permitted_arcana[i].Type)
				 || (pots[ifast].0.Affinity==permitted_arcana[i].Type){1.8
				 }else{1.3};
		
			let CM= if permitted_arcana[i].Light {
						pots[ifast].0.WM_shade*caff
					}else{
						pots[ifast].0.BM_shade*caff
					};
			let mut BM = pots[ifast].0.BM_shade as i64;
			BM = if BM<1 {1} else {BM};
		
			let mut extra_dam:f32 = CM-pots[ifast].0.WM*0.8;
			extra_dam = if extra_dam<0.0{0.0}else{extra_dam};
			
			let mut PW = spow(permitted_arcana[i].Power,CM,&pots[idm].0,&permitted_arcana[i]);
			PW = if PW<=0.0{0.0}else{PW};
			
			let spell_damage =  if !permitted_arcana[i].Light {
									PW*permitted_arcana[i].Health*0.06+extra_dam
								}else if permitted_arcana[i].Light & (pots[idm].0.Unclean | !pots[idm].0.Alive) {
									PW*permitted_arcana[i].Health*0.08+extra_dam
								}else{0.0};
								
			if spell_damage > mean_pot_dam {
				mean_pot_dam = spell_damage;
				choice = permitted[i]+2;
			};
		};

	};
	choice
}

//needs to be improved to take into account of waiting and panicking
//dumb chocie for random battle, output: 0= defend, 1 = attack. 1<magic,
//1=attack. 2...n=cast spell listed [n-2]th in ifast's spellist.
fn rand_choice(idm:usize ,caster:&Lifeform,c_name:&String, pots: &Vec<(Lifeform,usize,[Option<[usize;2]>;2])>, spells:&Vec<Spell>,ifast:usize)->usize{
	let lenny:usize=2+caster.Spellist.len();
	let mut choice:usize=0;
	if ifast==0{
		choice=rand::thread_rng().gen_range(0,lenny);
	}else{
		choice=rand::thread_rng().gen_range(0,lenny);
	};
	let spelln=if choice>1{caster.Spellist[choice-2]}else{-1};
	if choice>1{
		let mut spellcheck:bool = false;
		for i in 0..spells.len(){
			if spelln==spells[i].id{
				choice=i+2;
				spellcheck = true;
			}else{};
		};
		if !spellcheck {choice = rand::thread_rng().gen_range(0,2);};
	};
	choice
}

fn spow(power:f32, CM:f32, t:&Lifeform,spell:&Spell)->f32{ 
	
	let taff = if (t.SubType==spell.Type)
			   || (t.Affinity==spell.Type){1.5}else{1.0};
			 
	let mut pd = CM-t.WM_shade*taff;
	pd = if pd<0.0 {0.0}else{pd};
	let damage = (power+10.0)*(power+10.0)/400.0*pd;
	
	damage
}

fn spowl(power:f32, CM:f32, t:&Lifeform,spell:&Spell)->f32{ 
	
	let taff = if (t.SubType==spell.Type)
			   || (t.Affinity==spell.Type) {1.5}else{1.0};
			 
	let pd = CM*taff;
	let pd = if pd<0.0 {0.0}else{pd};
	let heal = (power+10.0)*(power+10.0)/800.0*pd;
	
	heal
}

fn bankai (ifast:usize, all:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>)->usize{
	let mut good_choice=false;
	let mut kimegao:usize=0;
	while good_choice==false{
		kimegao=rand::thread_rng().gen_range(0,all.len());
		good_choice=if all[ifast].1!=all[kimegao].1{true}else{false}
	};
	kimegao
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
	let damned:String = match state{
		5=>"uninjured".to_owned(),
		4=>"lightly wounded".to_owned(),
		3=>"wounded".to_owned(),
		2=>"gravely wounded".to_owned(),
		1=>"on death's door".to_owned(),
		0=>"dead".to_owned(),
		_=>"absent".to_owned()
	};
	damned
}

//machine learning version of state_match()
fn state_m(HP_tot:f32, HP_now:f32)->u8{
let stater:f32=HP_now/HP_tot;
let state:u8=
    if stater>=1.0{5}
	else if (stater<1.0) & (stater>=0.75){4}
	else if (stater<0.75) & (stater>=0.5){3}
	else if (stater<0.5) & (stater>=0.25){2}
	else if(stater<0.25) & (stater>0.0){1}
	else{0};
state
}

//Matches %HP with correct statement...
//...And will eventually output answers for machine learning.
fn dam_match_match(){}
fn dam_match(HP_tot:f32, HP_loss:f32, a_name:&String, d_name:&String, group:usize,
			 mut comm_text:&mut String){
	let stater:f32=HP_loss/HP_tot;
	let damn:i32=
		if stater>1.2{5}
		else if (stater<=1.2) & (stater>=0.6){4}
		else if (stater<0.6) & (stater>=0.3){3}
		else if (stater<0.3) & (stater>=0.1){2}
		else if(stater<0.1) & (stater>0.0){1}
		else{0};
	let damned:&str= match damn{
		5=>"physically obliterates",
		4=>"inflicts massive damage upon",
		3=>"inflicts grave wounds upon",
		2=>"lightly wounds",
		1=>"barely scratches",
		_=>"inflicts no harm upon",
	};
		
	*comm_text = format!("{}\n{} {} {} from group {}.",
		 comm_text,
         &a_name,
         damned,
         &d_name,
         group);
}

//calculates relative threat level of one lifeform vs another.
fn dangerous(L:&Lifeform,S:&Lifeform)->f32{
	let sp_val_L=L.Spellist.len() as f32;
	let lf_eval_L=L.HP_shade*(L.WM_shade+L.Defence_shade)
	           +(L.MP_shade+10.0*sp_val_L)*(L.WM_shade+L.BM_shade)
	           +L.Speed_shade*(L.WM_shade+L.BM_shade+L.Attack_shade)*10.0
	           +sp_val_L;
	let sp_val_S=S.Spellist.len() as f32;
	let lf_eval_S=S.HP_shade*(S.WM_shade+S.Defence_shade)
	           +(S.MP_shade+10.0*sp_val_S)*(S.WM_shade+S.BM_shade)
	           +S.Speed_shade*(S.WM_shade+S.BM_shade+S.Attack_shade)*10.0
	           +sp_val_S;
    lf_eval_L-lf_eval_S
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
	expgain= if expgain<0.0 {0.0}else{expgain};
	expgain
}

//This translate spell damage number to a string.
fn spd(being:String, damage:f32)->String{
	if damage > -100.0 {	  format!("touches {}.",being)
	}else if damage > -200.0 {format!("impacts {}.",being)
	}else if damage > -400.0 {format!("rains down upon {}.",being)
	}else if damage > -800.0 {format!("ravages {}.",being)
	}else{ 					  format!("envelops {} in utter destruction.",being)
	}
}


//Basic functions which will be put elsewhere afterwards:
//A bunch of unnecessary functions from I didn't know what I was doing.
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
