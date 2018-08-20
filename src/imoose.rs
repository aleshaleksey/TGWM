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
use num::Num;
use std::mem::transmute;
use std::thread;
use std::sync::mpsc::Receiver;
use time::PreciseTime;

// Generates the difference tables used in part_b2
// To be done in the initial part of the battle.
// cause_effect is a complex tuple. The second half is a collection of 
// all possible outcomes of an action.
fn ai_part_b1_marker(){}
pub fn ai_part_b1(lore: &Vec<Vec<[u8;28]>>,
				  differences: &mut Vec<Vec<[i8;23]>>,
				  cause_effect: &mut Vec<([u8;3],Vec<&[[i8;23]]>)>) {
	
	//Defensive: Reset differences.
	*differences = Vec::with_capacity(lore.len());
	
	//Fill differences using lore.
	for x in lore.iter() {
		let mut diff_n:Vec<[i8;23]> = Vec::with_capacity(x.len());
		
		for j in 0..(x.len()-1) {
			let mut slice = [0;23];
			for i in 5..28 {
				slice[i-5] = (x[j][i] as i16 - x[j+1][i] as i16) as i8
			};
			diff_n.push(slice);
		};
		diff_n.push([0;23]);
		differences.push(diff_n);
	}
	
	
	
	
}

// The main function for the AI decision maker.
// part_b2 will go inside here.
fn ai_part_a_marker(){}
pub fn ai_part_a <'a> (x:&Vec<(Lifeform,usize,[Option<[usize;2]>;2])>,
						 ii:usize,                             	//ifast from main battle.
						 turn:usize,
						 lore:&Vec<Vec<[u8;28]>>, 							 //(turn,turn,action,idm,ifast,light,x[0],x[1],x[2],x[3],x[4],x[5],x[6],x[7],x[8],x[9],x[10],etc)
						 differences: &Vec<Vec<[i8;23]>>,					 //lore effect substracted.
						 cause_effect: &Vec<([u8;3],Vec<&'a[[i8;23]]>)>,   //vector of causes and effects.
						 now:&[u8;28],              //recl essentially.
						 lsum:i32,
						 order:Vec<(u8,u8)>
					   ) -> (usize,usize) {
	let mut action=1;
	let mut idm=255;
	let t0=PreciseTime::now();
	let lore_len = lore.len();
	
	let mut best_act_pm:usize=1;	//Primary choice action
	let mut best_tar_pm:usize=255;	//Primary choice target
	{		 
		let selfl= x[ii].clone();                                 	//Lifeform acting.
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
			
		for y in x.iter(){									 //make by group vector.
			all_groups.push(y.1);
			ids.push(y.0.id);
			ids8.push(y.0.id as u8)
		};           
		uniq_group= uniq(&all_groups);               //make unique group vector,
		println!("All groups by [i]: {:?}\n All ids by [i]: {:?}\n Unique groups: {:?}",all_groups,ids,uniq_group);     
		println!("Owng: {}",x[ii].1);																					
		
		let mut d_battles:Vec<Vec<[i16;28]>>=Vec::new();
		let mut act_consequences:Vec<[i16;26]>=Vec::new();


		//let (v_lore,l_lore):(Vec<&Vec<[u8;28]>>,Vec<&Vec<[u8;28]>>)=lore.into_iter().partition(|&q| goal(&x,ii,&q[q.len()-1],&all_groups)==true);
		//println!("V-lore length: {}\nL-Lore length:{}",v_lore.len(),l_lore.len());
		//for y in v_lore[1].iter(){println!("{:?}",y)};
		//panic!("I want to pause here. FOREVER.");
		let mut v_paths:Vec<Vec<Vec<[u8;3]>>> = Vec::new();  //unique paths to victory from trigger point.
		let mut l_paths:Vec<Vec<Vec<[u8;3]>>> = Vec::new();   //unique paths to defeat from trigger point.
		let mut v_count:Vec<usize> = Vec::new();  //victory count at each trigger.
		let mut l_count:Vec<usize> = Vec::new();   //defeat count at each trigger.
		let mut trigs:Vec<[u8;3]> = Vec::new();       				//unique triggers
		trigs.push([255,255,255]);
		l_paths.push(vec!(vec!([255,255,255]);100000));
		v_paths.push(vec!(vec!([255,255,255])));     //NB this is a complex structure.	
		l_count.push(100000);
		v_count.push(0);
		
		//println!("got A");
		for y in lore.iter(){ 		//create unique paths for everything. Curently pathed out.
			//let mut temp_pather:Vec<[u8;3]>=Vec::new();
			//for z in y.iter(){temp_pather.push([z[2],z[3],z[4]])};
			//temp_pather.remove(0);
			//temp_pather.remove(0);
			//temp_pather.remove(0);
			let mut lxsum:i32=0;
			for i in 3..y.len(){
	//			let mut orders:Vec<(u8,u8)>=Vec::new();
	//			if i>3{
	//				for j in permit_a(i-1)..(i){orders.push((y[j][4],0))}
	//			}else{orders=vec![(0,0)]};
	//			let orders_fulfilled=order_matcher(&orders,&order);
				//temp_pather.remove(0);
	//			let now_m=state_matcher(&now,&y[i],cms,2);
				lxsum+=(y[i][5] as i32)-128;
				let mut light=true;
				if (y[i][4]==ii8) & (now[6..28]==y[i][6..28]) & (y[i][2]!=0) & (i>=turn) & light {					//Condition.
					let temp_trig:[u8;3]=[y[i][2],y[i][3],y[i][4]];
					let j=i+1;
					//let mut temp_path=temp_pather.clone();
					if lhas(&trigs,&temp_trig)==false{			//branch
						trigs.push(temp_trig);
						if goal(&x,ii,&y[y.len()-1],&all_groups) {
							//v_paths.push(vec![vec![[255,255,255]]]);
							v_count.push(0);
							let lng=v_paths.len()-1;
							//v_paths[lng].push(temp_path.clone());
							v_count[lng]+= 1;
							//l_paths.push(vec![vec![[255,255,255]]]);
							l_count.push(1)
						}else{
							//v_paths.push(vec![vec![[255,255,255]]]);
							v_count.push(0);
							//l_paths.push(vec![vec![[255,255,255]]]);
							l_count.push(1);
							let lng=v_paths.len()-1;
							//l_paths[lng].push(temp_path.clone());
							l_count[lng]+= 1;						
						}
					}else{
						if goal(&x,ii,&y[y.len()-1],&all_groups) {
							//v_paths[vvwhich(&trigs,temp_trig)[0]].push(temp_path);
							v_count[vvwhich(&trigs,temp_trig)[0]]+= 1;
						}else{
							//l_paths[vvwhich(&trigs,temp_trig)[0]].push(temp_path);
							l_count[vvwhich(&trigs,temp_trig)[0]]+= 1;
						}
					}
				}else{};
				light=light_m(lsum,lxsum.clone());
			}
		};
		
		//generate vectors of unique paths in order to count and assess. Currently commented out as unique paths are not used.
		let trigs_l=trigs.len();
		//let l_p_thread=thread::spawn(move||{
		//	for i in 1..trigs_l{l_paths[i]=uniq_m(l_paths[i].clone())};
		//		l_paths
		//	});
		//let v_p_thread=thread::spawn(move||{
		//	for i in 1..trigs_l{v_paths[i]=uniq_m(v_paths[i].clone())};
		//		v_paths
		//	});
		//l_paths=l_p_thread.join().expect("L-path crashed");
		//v_paths=v_p_thread.join().expect("V-path crashed");


		let mut l_v:Vec<f64>=Vec::new();
		let mut l_v_by_count:Vec<f64> = Vec::new();
		for i in 0..trigs_l{
			l_v_by_count.push((l_count[i] as f64)/((v_count[i]+l_count[i]) as f64));
			//if (v_paths[i].len()>0) & (l_paths[i].len()>0){ Path out.
			//	let lp=l_paths[i].len() as f64;
			//	let vp=v_paths[i].len() as f64;
			//	l_v.push(lp/vp)			
			//}else{
			//	l_v.push(std::f64::MAX);
			//}
		};
		
		
		//let all_paths_to_rome = v_paths.iter().fold(0,|acc,ref x| acc + x.len()); Path out.
		let total_victories = v_count.iter().fold(0,|acc,x| acc + x);
		let mut i_best_path:usize=0;    //Primary choice
		let mut i_best_count:usize=0;   //Alternative choice
		
		let mut best_act_sec:usize=1;	//Secondary choices.
		let mut best_tar_sec:usize=255;	//Secondary choice
		println!("Paths to rome = {}",
									  //all_paths_to_rome,
									  total_victories);
		
		if total_victories<1 {
			println!("Survival strategy failed. Herp (all_paths = {}). Derp (v_paths = {})",
																							total_victories,
																							//all_paths_to_rome,
																							v_paths.len());
			
			println!("Got A");
			for number in 0..lore.len(){        //Make chosen_battles battles list.  ((lsum-7)<=xlsum) & (xlsum<=(lsum+7)) &
				let mut xlsum:i32=0;
				for i in 3..lore[number].len(){
					//Insert xlsum and orders script here.
					//NB this is a modified permissive script.
					if (lore[number][i][4]==ii8) & (lore[number][i][6+ii]==now[6+ii]){				
						chosen_battles.push(&lore[number]);
						chosen_stats.push((lore[number][i][2],lore[number][i][3],all_groups[ii]));
						if goal(&x,ii,&lore[number][lore[number].len()-1],&all_groups) {vict_c_battles.push(&lore[number])}else{};
					}else{}
				};
			};
			println!("Got B");

	//		for y in chosen_battles.iter(){
	//			let mut temp_index=Vec::new();		
	//			for z in y.iter().rev(){	
	//				temp_index.push(z[4]);
	//				if z[4]==ii8{	
	//					if lhas(&temp_index,&ii8)==true{
	//						last_c_stat.push([z[2],z[3],z[6+ii],now[6+(z[3] as usize)]]);
	//						break
	//					}else{}
	//				}else{};
	//			}
	//		};

			for y in chosen_battles.iter(){
				let y_l=y.len()-1;
				if y[y_l][4]==ii8{
					last_c_stat.push([y[y_l][2],y[y_l][3],y[y_l][6+ii],now[6+(y[y_l][3] as usize)]]);
				}else{};
			};
			println!("Got C");
	//		for y in vict_c_battles.iter(){
	//			let mut temp_index=Vec::new();
	//			let mut lasty=y[y.len()-1];
	//			for z in y.iter().rev(){	
	//				temp_index.push(z[4]);
	//				if z[4]==ii8{	
	//					if lhas(&temp_index,&ii8)==true{
	//						last_v_stat.push([z[2],z[3],z[6+ii],now[6+(z[3] as usize)]]);
	//						break
	//					}else{}
	//				}else{};
	//			}
	//		};

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
				println!("Got D");
				let mut uniq_cl_stat=uniq(&last_c_stat);
				let mut last_index=vec!([1,255,5,5]);
				println!("Got Da");
				for y in uniq_cl_stat.into_iter(){
					if (y[2]==0)||(y[3]==0){}else{last_index.push(y)}
				};
				println!("Got Db");
				let l_c_count:Vec<f64>=vcount(&last_index,&last_c_stat);
				let l_v_count:Vec<f64>=vcount(&last_index,&last_v_stat);
				println!("Got Dc");
				let last_ratios:Vec<f64>=vf64ratio(&l_v_count,&l_c_count);
				println!("Got DcII");
				let salvation=last_index[vnmaxi(&last_ratios)];
				println!("Got Dd");
				best_act_pm=salvation[0] as usize;
				best_tar_pm=salvation[1] as usize;
		
				println!("Last ratios len: {}", last_ratios.len());
				println!("Last valid actions len: {}",last_index.len())
			}else{
				println!("No victories detected.");
				best_act_pm=1;
				best_tar_pm=255;
			};
			   //SCOPING OUT
			  //SCOPING OUT
			 //SCOPING OUT
			//SCOPING OUT 
			//Experimental Engine for telling causes and consequences.
			let not_now = false;
			if not_now {
			//This needs to be shifted out of this function and into the main function.
			
			//println!("Delta(battles): {}\nact_consequences: {}",d_battles.len(),act_consequences.len());
			//let mut temp_b=[0;23];	
			//let temp_a=[act_consequences[0][0],act_consequences[0][1],act_consequences[0][2]];
			//for i in 3..26{temp_b[i-3]=act_consequences[0][i]};	
			//cause_effect.0.push(temp_a);
			//cause_effect.1.push(vec!(temp_b));
				
			//for x in act_consequences.iter(){		
				//let mut temp_b=[0;23];	
				//let temp_a=[x[0],x[1],x[2]];
				//for i in 3..26{temp_b[i-3]=x[i]};
				//if lhas(&cause_effect.0,&temp_a) {			
					//cause_effect.1[vvwhich(&cause_effect.0,temp_a)[0]].push(temp_b)
				//}else{
					//cause_effect.0.push(temp_a);
					//cause_effect.1.push(vec!(temp_b))
				//}
			//};
			
			//let mut cause_meffect:(Vec<[i16;3]>,Vec<[i16;23]>)=(Vec::new(),Vec::new());
			//for i in 0..cause_effect.0.len(){
				//cause_meffect.0.push(cause_effect.0[i]);
				//cause_meffect.1.push(vmode(&cause_effect.1[i]))		
			//};
			//println!("Causes length:{}\nEffects length:{}",cause_effect.0.len(),cause_effect.1.len());
			//println!("Cause[1]: {:?}\nMode Effect[1]:{:?}",cause_meffect.0[0],cause_meffect.1[0]);
			
			//println!("lv_effects.len()=={}",lv_effects.len());
			
			////give up if you're going to crash.
			//if lv_effects.len()==0 {
				//println!("lv_effects.len()==0, so we're giving up and doing things the dumb way. Herp derp.");
				//std::mem::drop(chosen_battles);
				 //best_act_pm = 255;
				 //best_tar_pm = 255;
			//}else{
				//let (good,bad):([i16;23],[f64;23])=essential_n(lv_effects);
				//println!("Good: {:?}\n Bad: {:?}",good,bad);
				//let to_good=rms23(good,now);
				//let to_bad=rms23_special(bad,now);
				//println!("Good: {:?}\n Bad: {:?}",to_good,to_bad);
			//};
			
			
			
			//println!("Causes:\n{:?}",cause_effect.0);
			}; //SCOPING OUT OVER
			  //SCOPING OUT OVER
			 //SCOPING OUT OVER
			//SCOPING OUT OVER
			
					
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
			
			//println!("L-paths length: {}\nV-paths length:{}\n",l_paths.len(),v_paths.len());
			//println!("L-paths[1] length: {}\nV-paths[1] length:{}",l_paths[0].len(),v_paths[0].len());
			for i in 0..trigs.len(){println!("{}.) Trigs: {:?}, Ratio(paths,counts): ({})",
																							 i,
																							 &trigs[i],
																							 //&l_v[i],
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

//The winning team from the winning team's perspective;
//different lifeforms have different victory conditions.
//Simple version.
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
			 & (l_line[i+6]>0) 
			 & xx[i].0.Alive {
				alive=true
			}else{};
		};
		if alive {victory=false}else{}
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
				& ((xx[i].0.Type==GIANT)||(xx[i].0.Type==UNDEAD)||(xx[i].0.Type==HUMAN))
				& (l_line[i+6]>0){foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& (l_line[i+6]>0){foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & (l_line[i+6]>0){allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
		}
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
				 & (!xx[i].0.Alive ||!xx[i].0.Unclean)
				 & (l_line[i+6]>0){foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& (l_line[i+6]>0){foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & (l_line[i+6]>0){allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
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
				 & (xx[i].0.Type != GOBLIN)
				 & (l_line[i+6]>0){foes_live=true}else{}
			};
			if foes_live {victory=false}else{}	
		}else{
			for i in 0..xx.len(){
				if (all_groups[i]!=owng)
				& (l_line[i+6]>0){foes_live=true
				}else{};
				if (all_groups[i]==owng)
			     & (l_line[i+6]>0){allies_live=true
				}else{};			 
			};
			if foes_live || !allies_live {victory=false}else{}
		}
	}else{                           //victory condition for normal creatures.
		let mut foes_live=false;
		let mut allies_live=false;
		for i in 0..xx.len(){
			if (all_groups[i]!=owng)
			 & (l_line[i+6]>0){foes_live=true
			}else{};
			if (all_groups[i]==owng)
			       & (l_line[i+6]>0){allies_live=true
			}else{};			 
		};
		if foes_live || !allies_live {victory=false}else{}
	};					
	victory
}


//which function for multiple.
fn vvwhich<T:PartialOrd>(a:&Vec<T>, b:T)->Vec<usize>{
	let mut ivec:Vec<usize>=Vec::new();
	for i in 0..a.len(){
		if a[i]==b{ivec.push(i)}
		else{continue}
	}
	ivec
}
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

//does vector contain b?
fn lhas<T:PartialOrd>(a:&Vec<T>, b:&T)->bool{
	let mut ihaz=false;
	for x in a.iter(){
		if x==b{
			ihaz=true;
			return ihaz}
		else{continue}
	}
	ihaz
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





//machine learning version of state_match()
fn state_m(HP_tot:f64, HP_now:f64)->u8{
let stater:f64=HP_now/HP_tot;
let state:u8=
    if stater>=1.0{5}
	else if (stater<1.0) & (stater>=0.75){4}
	else if (stater<0.75) & (stater>=0.5){3}
	else if (stater<0.5) & (stater>=0.25){2}
	else if(stater<0.25) & (stater>0.0){1}
	else{0};
state
}

fn uniq<T:Copy + PartialOrd>(mvec:&Vec<T>)->Vec<T>{
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

fn uniq_m<T:PartialOrd>(mvec:Vec<T>)->Vec<T>{
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

//Counts cases of "indv[i]" for each entry of "indv" in "mesv".
fn vcount<T:PartialOrd + Copy>(indv:&Vec<T>, mesv:&Vec<T>)->Vec<f64>{
	let il=indv.len();
	let mut counts:Vec<f64>=vec![0.0;il];
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
fn vf64ratio(num:&Vec<f64>, denom:&Vec<f64>)->Vec<f64>{
	let dnl=denom.len();
	let mut rvec:Vec<f64>= vec!(0.0;dnl);
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

fn byteru16(x:u16)->[u8;2]{
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
	let nl=now_l as f64;
	let ol=observed_l as f64;
	let upper_lim:f64=nl*1.1+5.0;
	let lower_lim:f64=nl*0.9-5.0;
	if (ol<=upper_lim) & (ol>=lower_lim){true}else{false}
}

fn vmode<T:PartialOrd+Copy> (data:&Vec<T>)->T{
	let uniq_t=uniq(data);
	let count_t=vcount(&uniq_t,&data);
	let index=vnmaxi(&count_t);
	uniq_t[index]
}


fn essential_n (homo:Vec<[i16;23]>)->([i16;23],[f64;23]){
	let mut ess_a=homo[0];
	let mut means:[f64;23]=[0.0;23];
	let l_h=homo.len() as f64;
	if l_h<1.0{println!("l_h.len()==0! We're gonna crash!");};
	for x in homo.into_iter(){
		for i in 0..23{
			if ess_a[i]!=x[i]{ess_a[i]=0}else{};
			means[i]+=x[i] as f64		
		}
	};
	for i in 0..23{
		if ess_a[i]==0{means[i]=means[i]/l_h}else{means[i]=0.0}
	};
	(ess_a,means)
}

fn rms23<T:Num+Copy>(a:[T;23],now:&[u8;28])->f64
where f64: std::convert::From<T>{
	let mut rms:f64=0.0;
	for i in 0..23{
		rms+=((now[i+5] as f64)-(f64::from(a[i]))).powf(2.0)
	};
	rms.sqrt()
}

fn rms23_special<T:Num+Copy>(a:[T;23],now:&[u8;28])->f64
where f64: std::convert::From<T>{
	let mut rms:f64=0.0;
	for i in 0..23{
		let b=f64::from(a[i]);
		rms+=((now[i+5] as f64)-(b)).powf(2.0)
	};
	rms.sqrt()
}

fn rms23_special_b<T:Num+Copy>(a:[T;23],now:&[u8;28])->f64
where f64: std::convert::From<T>{
	let mut rms:f64=0.0;
	for i in 0..23{
		let b=f64::from(a[i]);
		if b>0.0{rms+=((now[i+5] as f64)-(b)).powf(2.0)}else{}
	};
	rms.sqrt()
}

