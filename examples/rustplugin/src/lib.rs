use std::mem::ManuallyDrop;

use bakkesmod::console;
use bakkesmod::game;
use bakkesmod::prelude::*;
use bakkesmod::wrappers::Actor;
use bakkesmod::wrappers::Car;
use bakkesmod::wrappers::VehicleInputs;

#[plugin_init(RustPlugin)]
pub fn on_load() {
	let cvar = console::register_cvar("rust_cvar");
	cvar.on_changed(|old_value, cvar| {
		log_console!("old value = {old_value}, new value = {}", cvar.get::<i32>());
	});
	log_console!("cvar name = {}", cvar.get_name());
	cvar.set(42);
	cvar.notify();
	cvar.set(69);

	let cvar = console::get_cvar("rust_cvar").unwrap();
	log_console!("f32 val = {}", cvar.get::<f32>());
	log_console!("remove = {:?}", console::remove_cvar("rust_cvar"));
	log_console!("after remove f32 val = {}", cvar.get::<f32>());

	console::register_notifier("rust_copy", move |args| {
		log_console!("{}", args.join(" "));
	});

	console::register_notifier("rust_spawn", |_| match game::current_state() {
		None => log_console!("no server"),
		Some(server) => server.spawn_car("doobus")
	});

	game::hook_event("Function TAGame.Car_TA.OnHitBall", |car: &Car, params| {
		// let ball = unsafe { params.get_actor::<Actor>(0) };
		// log_console!("player hit ball @ pos {} (hf {})", car.position(), car.has_flip());
		let server = game::current_state().unwrap();
		log_console!("cars: {}", server.cars().len());
		for car in server.cars() {
			let Some(pri) = car.pri() else {
				continue;
			};
			log_console!("car ({}) {} - boost {}", pri.index(), pri.player_name(), car.boost().amount());
		}
	});

	game::hook_event("Function TAGame.Car_TA.SetVehicleInput", |car: &Car, params| {
		let Some(pri) = car.pri() else {
			return;
		};
		log_console!(
			"car {}/{} ({}, {:?}) - demo {}, on ground {}, has jump {}, flip {}",
			pri.index(),
			pri.team().index(),
			car.position(),
			car.rotation(),
			car.is_hidden(),
			car.is_on_ground(),
			car.can_jump(),
			car.has_flip()
		);
		// let p = unsafe { params.get_mut::<VehicleInputs>(0) };
		// p.set(VehicleInputs::BOOST);
		// if p.flags != 0 {
		// 	log_console!("flags: {:08b}", p.flags);
		// }
	});

	console::register_notifier("rust_demolish", |args| {
		match game::local_car() {
			Some(car) => car.demolish(),
			None => log_console!("Car is NULL")
		};
	});

	// console::register_notifier("rust_notifier", Box::new(normal_function_callback));

	// console::register_notifier(
	// 	"rust_set_loc",
	// 	Box::new(move |_: Vec<String>| {
	// 		match game::get_local_car() {
	// 			Some(car) => {
	// 				let origin = vec3!(0.0, 0.0, 0.0);
	// 				let new_loc = origin + vec3!(200.0, 1000.0, 50.0);
	// 				car.set_location(new_loc);
	// 			}
	// 			None => log_console!("Car is NULL")
	// 		};
	// 	})
	// );

	// console::register_notifier(
	// 	"rust_set_gravity",
	// 	Box::new(move |params: Vec<String>| {
	// 		if params.len() < 2 {
	// 			log_console!("not enough parameters!");
	// 			return;
	// 		}

	// 		let new_value_str = &params[1];
	// 		let new_value: f32 = match new_value_str.parse::<f32>() {
	// 			Ok(val) => val,
	// 			Err(_) => {
	// 				log_console!("invalid input!");
	// 				return;
	// 			}
	// 		};

	// 		match console::get_cvar("sv_soccar_gravity") {
	// 			Some(cvar) => {
	// 				log_console!("current value: {}", cvar.get_float_value());
	// 				log_console!("setting to {}", new_value);
	// 				cvar.set_float_value(new_value);
	// 			}
	// 			None => log_console!("cvar 'sv_soccar_gravity' not found")
	// 		};
	// 	})
	// );

	// console::add_on_value_changed(
	// 	&cvar,
	// 	Box::new(move |_: String, cvar: CVar| {
	// 		log_console!("cvar {} has a new value: {}", cvar.get_name(), cvar.get_string_value());
	// 	})
	// );

	// console::register_cvar("rust_ticker");

	// let counter_base = Rc::new(RefCell::new(0));
	// let counter_ref = Rc::clone(&counter_base);

	// game::hook_event(
	// 	"Function Engine.GameViewportClient.Tick",
	// 	Box::new(move || {
	// 		let ticker = console::get_cvar("rust_ticker").unwrap();
	// 		if !ticker.get_bool_value() {
	// 			return;
	// 		}

	// 		let mut counter = counter_ref.borrow_mut();
	// 		*counter += 1;
	// 		if (*counter % 240) == 0 {
	// 			// log_console!("viewport client tick");

	// 			match game::get_local_car() {
	// 				Some(car) => {
	// 					let location = car.get_location();
	// 					log_console!("{}", location);

	// 					let vehicle_sim = car.get_vehicle_sim().unwrap();
	// 					let wheels = vehicle_sim.get_wheels();
	// 					let wheel0 = wheels.get(0);
	// 					log_console!("wheel 0 spin speed: {}", wheel0.get_spin_speed());
	// 					let wheel3 = wheels.get(3);
	// 					log_console!("wheel 3 spin speed: {}", wheel3.get_spin_speed());
	// 				}
	// 				None => log_console!("Car is NULL")
	// 			};
	// 		}
	// 	})
	// );

	// game::register_drawable(Box::new(move |canvas: Canvas| {
	// 	let top_left = vec2!(100, 100);
	// 	let width = vec2!(250, 0);
	// 	let height = vec2!(0, 150);
	// 	canvas.draw_line(top_left, top_left + width);
	// 	canvas.draw_line(top_left + width, top_left + width + height);
	// 	canvas.draw_line(top_left, top_left + height);
	// 	canvas.draw_line(top_left + height, top_left + width + height);
	// }));

	// game::hook_event_with_caller_post(
	// 	"Function TAGame.Car_TA.ApplyBallImpactForces",
	// 	Box::new(move |car: Box<CarWrapper>| {
	// 		log_console!("Ball hit!");
	// 		log_console!("car location: {}", car.get_location());
	// 	})
	// );

	// console::register_notifier(
	// 	"rust_set_timer",
	// 	Box::new(move |params: Vec<String>| {
	// 		if params.len() < 2 {
	// 			log_console!("not enough parameters!");
	// 		} else {
	// 			let time_param = &params[1];
	// 			let time: f32 = match time_param.parse::<f32>() {
	// 				Ok(secs) => secs,
	// 				Err(_) => {
	// 					log_console!("invalid input!");
	// 					return;
	// 				}
	// 			};
	// 			game::set_timeout(
	// 				Box::new(move || {
	// 					log_console!("{} secs have passed!", time);
	// 				}),
	// 				time
	// 			);
	// 		}
	// 	})
	// );

	// console::register_notifier(
	// 	"rust_get_ball_info",
	// 	Box::new(move |_: Vec<String>| {
	// 		let game = match game::get_game_event_as_server() {
	// 			Some(g) => g,
	// 			None => {
	// 				log_console!("game is null!");
	// 				return;
	// 			}
	// 		};

	// 		match game.get_ball() {
	// 			Some(ball) => log_console!("{}", ball.get_location()),
	// 			None => log_console!("ball is NULL")
	// 		};
	// 	})
	// );

	// console::register_notifier("rust_spawn_car", Box::new(spawn_car_callback));
}

// fn normal_function_callback(params: Vec<String>) {
// 	log_console!("this is the callback for rust_notifier!");
// 	log_console!("params = {:x?}", params);
// }

// fn spawn_car_callback(_: Vec<String>) {
// 	let game = match game::get_game_event_as_server() {
// 		Some(g) => g,
// 		None => {
// 			log_console!("game is null!");
// 			return;
// 		}
// 	};

// 	game.spawn_bot(22, "Bors");
// }
