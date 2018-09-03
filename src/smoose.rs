#[allow(unreachable_code)]
#[allow(unused_mut)]
///TGWM: module smoose
///
///The smoose module handles NPCs.
///For now it a clone of the functions
///from the terminal version.
///A placeholder.
///
///
///
//mod lmoose;

use std::io;
#[allow(unused_imports)] use lmoose::{Spell,Item,Lifeform,Shade,Place,cureL,cure,cureG,cureH,exorcism,exorcismG,exorcismH,
			 ember,fire,fireball,inferno,spark,lightning,lightningH,crystalliseL,crystallise,crystalliseH,
			 sum_reaper,teleport,teleportG,light,lightH,darkness,darknessH,slow,haste,lifestealer,curse,
			 apocalypse,timestop,world,goblin_dem,goblin_sco,goblin_witch,bandit,bandit_lord,dark_apprentice,
			 necromancer,necromancer_lord,skeleton,skeleton_kn,ghost,ghost_an,white_witch,beast_green,
			 beast_red,beast_great,fallen,titan,warrior,witch,wonderer,alien,loser,beast_serpent,sage_forsaken,
			 white_queen,shortstaff};
			 
#[allow(unused_imports)] use lmoose::{ADVENT,ALBION,ALIEN,ANGEL,BEAST,BONE,BRIDGE,CITY,
		     DEATH,DESERT,ELF,EVIL,FALLEN,FIRE,FOREST,GIANT,GOBLIN,GRASSLAND,
		     HEALING,HIGHLAND,HOLY,HUMAN,ICE,LIGHTNING,MALACHIA,
			 MINDLESS,MOORLAND,MOOSE,RADIANT,RUIN,STEPPE,SPIRIT,
			 TELEPORTATION,TIME,TUNDRA,UNDEAD,VOID,WATER,WITCH,WHITE,NONE};

fn sage_prices<'a>(list:&'a Vec<Spell>,typ:u8,special:Vec<&str>)->Vec<(&'a Spell,usize)>{
	let mut shopping:Vec<(&Spell,usize)>=Vec::new();
	if special.len()==0{
		for x in list.iter(){
			if x.Type==typ{
				shopping.push((x,(x.MP*x.MP) as usize))
			}	
		}
	}else{
		for x in special.into_iter(){
			for y in list.iter(){
				if x==y.name{
					shopping.push((y,(y.MP*y.MP) as usize))
				}
			}
		}
	};
	shopping
}

fn form55(x:&Vec<&str>,ma:usize){
	let maxl:f64=ma as f64;
	let s=vec!["_";ma+9].concat();
	let d=vec!["_";ma+7].concat();
	println!("{}",&s);
	for i in 0..x.len(){
		let j=i+1;
		let xl=x[i].len();
		let xml=maxl-(x[i].len() as f64);
		let spacel:usize=if j<10{(xml/2.0) as usize} else {(xml/2.0) as usize-1};
		let sa=vec!(" ";spacel).concat();
		let sb=if xl%2==1{vec!(" ";spacel).concat()}else{vec!(" ";spacel-1).concat()};
		println!("|[{}]>{}{}{}<[{}]|",j,&sa,&x[i],&sb,j);
		//println!("Spacer length={}",spacel)
		
	};
	println!("|{}|\n",&d);
}

fn form55_string(x:&Vec<String>,ma:usize){
	let maxl:f64=ma as f64;
	let s=vec!["_";ma+9].concat();
	let d=vec!["_";ma+7].concat();
	println!("{}",&s);
	for i in 0..x.len(){
		let j=i+1;
		let xl=x[i].len();
		let xml=maxl-(x[i].len() as f64);
		let spacel:usize=if j<10{(xml/2.0) as usize} else {(xml/2.0) as usize-1};
		let sa=vec!(" ";spacel).concat();
		let sb=if xl%2==1{vec!(" ";spacel).concat()}else{vec!(" ";spacel-1).concat()};
		println!("|[{}]>{}{}{}<[{}]|",j,&sa,&x[i],&sb,j);
		//println!("Spacer length={}",spacel)
		
	};
	println!("|{}|\n",&d);
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

#[allow(unused_must_use)]
fn any_key(){
	println!("[Press any key to continue]\n");
	let mut steserifu=String::new();
	io::stdin().read_line(&mut steserifu);
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_fire(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["It's cold here","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about fire spells.",
	             "Tell me why you sages are always so fickle.",
	             "Tell me about the Desert.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,FIRE,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	


	println!("You light a fire in the desert night and realise that you are not alone.
The form of a nomad sits by the fire, adoring it like a dear child...
\n...\"I am the Sage of Fire\" it states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}? Or did you just want to stay warm?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see. Don't burn out\n...\nThe sage is gone and you are left alone in the {} again.\n",&loc.name);
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Fire magic is the light born of destruction that burns up everything.
Fire magic is the most destructive of magics, but it also carries the seed of rebirth.
Inferno is the greatest fire magic. It is the fire that burns up everything.\n");
					any_key();
					continue},
					Ok(2)=>{println!("Fickle? It is simply that without an arcane wisdom equaling our own, you cannot understand the logic that binds us.\n"
					);
					any_key();
					continue},
					Ok(3)=>{println!("The desert is where the unnecessary aspects of self are burned away, and the soul is reborn in its true form.
The ashes of that which no longer belongs are mixed into the sands...
...Nations...
...Cultures...
...Gods.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor extinguishes the last fire...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Fire teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}


#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_lightning(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about lightning spells.",
	             "Tell me what you wished for when you became a sage.",
	             "Tell me about the Stone Maze.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,LIGHTNING,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("A single spark in the stony labyrinth...\n...Calls down lightning from a clear sky.
The flash momentarily blinds you and you realise that this lightning is something else.
\n...\"I am the Sage of Lighning\" it states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see, and may you see too {}",party[0].0.name);
			println!("...\n...\nThe lightning fades and the Sage of Lightning is gone as if they never were...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Lightning magic is the anger of the gods.
It destroys and disempowers. Only the truly righteous can withstand it.
Jovian Lightning is the greatest lightning magic. It is the spear of burning gold borrowed from the heavens.\n");
					any_key();
					continue},
					Ok(2)=>{println!("Each of us had their own wish. But they could only be fulfilled by learning that which is forbidden to mortals.\n"
					);
					any_key();
					continue},
					Ok(3)=>{println!("This maze is all that remains of the nation that challenged the gods...
...Flora...
...Fauna...
...Lands...
...Waters...
...Sky...
...All purified by Jove's lightning.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor punishes the gods for their arrogance...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Lighning teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_ice(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about ice spells.",
	             "Tell me what you sages are anyway.",
	             "Tell me about the Frozen Wastes.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,ICE,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("You try to make the artic wastes a little colder...\n...And the air around you begins to coalesce.
Before you stands the White Queen.
\n...\"I am the Sage of Ice\" it states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("...\n...\nThe Sage of Ice was just another jagged block of ice.....");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Ice magic is the destruction of fire.
It is the stillness that waits at the end of the world.
There is a spell that brings peace and stillness: True Crystalise is that gift.\n");
					any_key();
					continue},
					Ok(2)=>{println!("We are what happens to mere mortals when you surpass the limits of mortal knowledge. We have become inseperate from the sacred laws that have created the world.\n");
					any_key();
					continue},
					Ok(3)=>{println!("These icy plains and peaks are exactly what they seem.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor's heart crystalises...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Ice teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_light(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about light spells.",
	             "Tell me about white magic.",
	             "Tell me about the White Sea.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,NONE,
		vec!["Light","Sacred Light","Exorcism","Greater Exorcism","Holy Exorcism"]);
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("As you cast your spell, a shining figure approaches you from the mists.
Serene, it walks over the water. As it approaches your ship, it says:
\n...\"I am the Sage of Light\" it states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("If there is something you need from me, I here.\n...\nThe Sage of Light turns and melts into the mists...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Light spells bring light to dark places, be they physical or metaphysical.
White magic is strengthened and black magic is weakened in the light.
The more potent light spells banish unclean creatures directly.
The Sacred spells are the most potent spells available to a white mage.\n");
					any_key();
					continue},
					Ok(2)=>{println!("White magic is born of goodwill. Even without using spells, it protects against both physical harm and dark sorceries.
When projected as a spell, white magic preserves and restores things to the way they ought to be.\n");
					any_key();
					continue},
					Ok(3)=>{println!("The White Sea... With the light fading, it is no longer the sacred place it used to be.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor can no longer see the light...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Light teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_darkness(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about darkness spells.",
	             "Tell me about black magic.",
	             "Tell me about the Black Obelisk.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,NONE,
		vec!["Darkness","Abyssal Darkness","Curse"]);
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("As darkness descends upon the Black Obelisk, you feel a presence besides you.
Standing at your side, darker than black, it utters:
\n...\"I am the Sage of Darkness\".\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("Then may you see only darkness on your quest.\n...\nYour spell lifts and you are alone in the land of the Black Obelisk...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("My magic gives birth to darkness, extinguishing the light that gives life its meaning. Just as the rain extingushes a small flame, the darkness extinguishes the hope that fuels white magic.
The more potent spells of darkness transcend the unphysical and weaken order upon which physical matter is based.
Abbysal spells, despite their potence, are not the most powerful dark magics in this world.\n");
					any_key();
					continue},
					Ok(2)=>{println!("Black magic is born of despair. Your despair will not help you unless you project it as a spell.
When projected as a spell, black magic will to restore all things to the nothingness they were before the world began.
\n");
					any_key();
					continue},
					Ok(3)=>{println!("The Black Obelisk is my masterpiece: It is the crystalisation of the cumulative negative energy of all living things.
You can see all that is wrong with yourself if you look for long enough into this stone.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor curses all the lands, sea and sky, and the void engulfs all things in its blackness.\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Darkness teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_life(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about healing spells.",
	             "Tell me why I keep coming back to life.",
	             "Tell me about the White Island.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,HEALING,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("You cast your spell amongst the harsh winds of the White Island.
Someone stands before you.
\n...\"I am the Sage of Light\" it utters.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("\"Then stay alive till we next meet\".\n...\nThe wind picks up, and you can no longer remember whether an old man or a young child stood before you...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("My magic gives birth to darkness, extinguishing the light that gives life its meaning. Just as the rain extingushes a small flame, the darkness extinguishes the hope that fuels white magic.
The more potent spells of darkness transcend the unphysical and weaken order upon which physical matter is based.
Abbysal spells, despite their potence, are not the most powerful dark magics in this world.\n");
					any_key();
					continue},
					Ok(2)=>{println!("You keep coming back to life because I want you to.\n");
					any_key();
					continue},
					Ok(3)=>{println!("The White Island is a haunted place.
It is the last place where those who have overstayed their welcome upon this world are permitted to exist before they go on their way.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor leaves the White Island for the last time...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Life teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_death(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["...","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about death spells.",
	             "Tell me why I keep coming back to life.",
	             "Tell me about the City of the Dead.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,DEATH,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("You finish exorcising the crypts and the dead lie still as they should. Then they rise again.
The corpses bow as one and speak as a raspy chorus:
\n...\"I am the Sage of Death\"\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWhy did you call me, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("\"Then stay close to the grave\".\n...\nThe corpses crumple like puppets, just as they should have to start of with...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("My magic gives birth to darkness, extinguishing the light that gives life its meaning. Just as the rain extingushes a small flame, the darkness extinguishes the hope that fuels white magic.
The more potent spells of darkness transcend the unphysical and weaken order upon which physical matter is based.
Abbysal spells, despite their potence, are not the most powerful dark magics in this world.\n");
					any_key();
					continue},
					Ok(2)=>{println!("You keep coming back to life because the world has not yet died.\n");
					any_key();
					continue},
					Ok(3)=>{println!("The City of the Dead is a place of rest. Just as the desert is the grave of oceans...
This is the grave of civilisations.
Now that the world itelf is tired, it has become crowded in my beloved city."
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world will end when Bremnor can rest no longer...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Death teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_albion(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["Oops","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about time magic.",
	             "Tell me, were your wishes granted.",
	             "Tell me about Albion.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,TIME,Vec::new());
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("O Albion! Is this your former splendour?..
You illuminate the empty temples with your spell and see a young man stride towards you.
\n...\"I am the Sage of Albion\" he states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nWelcome to the Albion of my youth, {}. My apologies for this poor hospitality.",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("\"I see.\"");
			any_key();
			println!("\"I see.\"");
			println!("The sage freezes motionless and flickers out of existence...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Chronomancy is the power to manipulate your vector in a non-spacial dimension.
It can change the course of history...
...Timestop is the most powerful time magic. Alas, there is no way to truly turn back the clocks...\n");
					any_key();
					continue},
					Ok(2)=>{println!("Our wishes were granted, but it would have been better had they been left unfulfilled...\n");
					any_key();
					continue},

					Ok(3)=>{println!("Albion is but a ruin now. Once upon a time a city of light by the name of Albion stood here...
...It embodied, peace...
...Fairness...
...Prosperity...
...The future that our wishes devoured.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("The world has already ended, Bremnor just hasn't realised it yet...\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Albion teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(unused_mut)]
pub fn sage_malachia(mut party:Vec<(Lifeform,usize)>, loc:&Place, summon:&str,spl:&Vec<Spell>)->Vec<(Lifeform,usize)>{
	let mut sage_loop=true;
	let q_0=vec!["Oops","I wanted to ask a question","I wanted to learn a spell"];
	let q_1=vec!["Tell me about Apocaplypse.",
	             "Tell me, what did your wishes do to this world?",
	             "Tell me about Malachia.",
	             "Tell me when the world will end.",
	             "Don't tell me anything."];
	let mut pricelist:Vec<(&Spell,usize)>=sage_prices(spl,NONE,vec!["Apocalypse"]);
	let mut on_offer:Vec<String>=Vec::new();
	for x in pricelist.iter(){
		let owned:String=x.0.name.to_owned();
		let off_string=owned+" ("+&(x.1.to_string())+"gp)";
		on_offer.push(off_string)
	};
	on_offer.push("Maybe not...".to_owned());
	

	println!("You summon death incarnate at the square of Malachia...
The reaper stands motionless, observing the sky, then it turns to face you.
\n...\"I am the Sage of Malachia\" it states.\n");
	any_key();

	loop{
		let mut sage_loop2=true;
		let mut sage_loop3=true;
		println!("\nHas the time finally come, {}?",party[0].0.name);
		let mut ans_0=String::new();
		form55(&q_0,60);
		io::stdin().read_line(&mut ans_0);
		let ans_0= match ans_0.trim().parse(){
			Ok(1)=>1,
			Ok(2)=>2,
			Ok(3)=>3,
			Ok(_)=>4,
			Err(_)=>4,
		};
		if ans_0==4{
			continue
		}else if ans_0==1{
			println!("I see.");
			println!("\"We all reaped as we sowed\".\n...\n\"You too will not reap but what you sowed\"\n...And the reaper fades away...");
			any_key();
			return party
		}else if ans_0==2{
			while sage_loop2==true{
				println!("What would you ask, {}?\n",party[0].0.name);
				form55(&q_1,60);
				let mut ans_1=String::new();
				io::stdin().read_line(&mut ans_1);
				let ans_1= match ans_1.trim().parse(){
					Ok(1)=>{println!("Apocalypse is a magic that can wrong all that is wrong..
...And the breathing corpse that is this world, will be no more upon its casting...
...And a living world, which has yet to give up, can be born anew.\n");
					any_key();
					continue},
					Ok(2)=>{println!("We became sages, part of the logic of this world..
...and all our wishes were granted, and all our hopes were fulfilled.
But our hopes had become this world's hopes, and with our hopes fulfilled...
...The world had nothing left to hope for.\n");
					any_key();
					println!("\nSo this hopeless world died.\n");
					any_key();
					continue},

					Ok(3)=>{println!("Malachia is the green city. More precisely that is what it used to be. Perhaps next time it will not end like this.\n"
					);
					any_key();
					continue},
					Ok(4)=>{println!("That... That is up to you.\n"
					);
					any_key();
					continue},
					Ok(5)=>{sage_loop2=false;
					println!("Then I shall keep my peace.\n");
					any_key()},
					Ok(_)=>continue,
					Err(_)=>continue,
				};
			}
		}else if ans_0==3{
			while sage_loop3==true{
				println!("\nWhich spell would you learn?\n");
				form55_string(&on_offer,60);
				let mut ans_2=String::new();
				io::stdin().read_line(&mut ans_2);
				let mut ans_2:usize= match ans_2.trim().parse(){
				Ok(num)=>num,
				Err(_)=>continue,	
				};
				ans_2-=1;
				if (ans_2+1)<on_offer.len(){
					if (pricelist[ans_2].1<=party[0].0.Gold) & (lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==false){	
						party[0].0.Spellist.push(pricelist[ans_2].0.id);
						println!("Sage of Malachia teaches you {}.",&pricelist[ans_2].0.name);
						any_key();
						sage_loop3=true
					}else if lhas(&party[0].0.Spellist,&pricelist[ans_2].0.id)==true{
						println!("You already know {}.",&pricelist[ans_2].0.name);
						any_key()
					}else{
						println!("Your pockets are not deep enough to pay the sage's ridiculous asking price.\n");
						any_key();
					}
				}else{
					println!("Some other time...\n");
					any_key();
					sage_loop3=false
				}
			}		
		}else{}
	};	
	party
}
