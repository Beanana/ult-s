#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

pub static mut FIGHTER_MANAGER: usize = 0;

mod util;
mod common;

mod bayonetta;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod cloud;
mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;
mod edge;
mod element;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod gaogaen;
mod gekkouga;
mod ike;
mod inkling;
mod jack;
mod kamui;
mod kirby;
mod koopa;
mod koopajr;
mod krool;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod murabito;
mod packun;
mod pacman;
mod palutena;
mod peach;
mod pichu;
mod pikachu;
mod pikmin;
mod pit;
mod pitb;
mod popo;
mod ptrainer;
mod purin;
mod reflet;
mod richter;
mod ridley;
mod robot;
mod rosetta;
mod roy;
mod ryu;
mod samus;
mod samusd;
mod sheik;
mod shizue;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod tantan;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
mod wolf;
mod younglink;
mod zelda;

#[skyline::main(name = "ult_s")]
pub fn main() {
	//Common
	util::install();
	common::install();
	
	//Fighters
	bayonetta::install();
	brave::install();
	buddy::install();
	
	captain::install();
	chrom::install();
	cloud::install();
	
	daisy::install();
	dedede::install();
	demon::install();
	diddy::install();
	dolly::install();
	donkey::install();
	duckhunt::install();
	
	edge::install();
	element::install();
	
	falco::install();
	fox::install();
	
	gamewatch::install();
	ganon::install();
	gaogaen::install();
	gekkouga::install();
	
	ike::install();
	inkling::install();
	
	jack::install();
	
	kamui::install();
	kirby::install();
	koopa::install();
	koopajr::install();
	krool::install();
	
	link::install();
	littlemac::install();
	lucario::install();
	lucas::install();
	lucina::install();
	luigi::install();
	
	mario::install();
	mariod::install();
	marth::install();
	master::install();
	metaknight::install();
	mewtwo::install();
	miifighter::install();
	miigunner::install();
	miiswordsman::install();
	murabito::install();
	
	packun::install();
	pacman::install();
	palutena::install();
	peach::install();
	pichu::install();
	pikachu::install();
	pikmin::install();
	pit::install();
	pitb::install();
	popo::install();
	ptrainer::install();
	purin::install();
	
	reflet::install();
	richter::install();
	ridley::install();
	robot::install();
	rosetta::install();
	roy::install();
	ryu::install();
	
	samus::install();
	samusd::install();
	sheik::install();
	shizue::install();
	simon::install();
	snake::install();
	sonic::install();
	szerosuit::install();
	
	tantan::install();
	toonlink::install();
	trail::install();
	
	wario::install();
	wiifit::install();
	wolf::install();
	
	younglink::install();
	
	zelda::install();
}