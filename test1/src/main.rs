
use jmge::*;
use std::rc::Rc;
use rand::Rng;


fn oops(e: Error) -> !
{
	println!("Fatal error: {}", e.to_string());
	std::process::exit(1);
}


#[derive(Component)]
struct RotSpeed (f32);


struct RotSys
{
	f: f32,
}

impl System for RotSys
{
	fn run(&mut self, world: &World)
	{
		self.f += 1.0;

		for (e, mut r) in world.iter_mut::<Renderable>()
		{
			if let Some(rs) = world.try_get::<RotSpeed>(&e)
			{
				r.angle = self.f*rs.0/100.0;
			}
		}
	}
}


fn run() -> Result<(), Error>
{
	let mut wnd = Window::new()?;
	let mut rend = Renderer::new()?;

	let _cnv1 = Canvas::from_memory_file(include_bytes!("../../kanade.png"))?;
	let cnv2 = Canvas::from_memory_file(include_bytes!("../../kanade2.png"))?;
	let cnv3 = Canvas::from_memory_file(include_bytes!("../../adventurer.png"))?;

	let mut atlas = TextureAtlas::new(2048, false);
	let tex = Rc::new(atlas.add(cnv2)?);


	let ss = Rc::new(SpriteSheet::from_file("adventurer.json", &cnv3, &mut atlas)?);


	let audio = Audio::new()?;

	let sound = Sound::from_file("what.wav")?;
	let music = Sound::from_file("Battleship.ogg")?;
	let mut sink = None;


	let mut world = World::new();
	world.register::<Renderable>();
	world.register::<RotSpeed>();
	world.register::<Sprite>();


	let (tw, th) = tex.size();

	let mut ents = Vec::new();

	let mut rng = rand::thread_rng();
	for _ in 0..100
	{
		let ent = world.new_entity();

		let mut r = Renderable::new(&tex, rng.gen_range(0, 1920), rng.gen_range(0, 1080));
		r.x_origin = tw as i32/2;
		r.y_origin = th as i32/2;

		world.set(&ent, r);

		world.set(&ent, RotSpeed(rng.gen_range(0.5, 4.0)));

		ents.push(ent);
	}

	//let mut f = 0.0;

	let rotater = RotSys { f: 0.0 };
	world.add_system("rotater", rotater);


	let sp = Sprite::new(&ss, "idle");
	let mut r = Renderable::new(&sp.get_texture(), 500, 400);
	r.x_scale = 4.0;
	r.y_scale = 4.0;
	
	let adv = world.new_entity();
	world.set(&adv, r);
	world.set(&adv, sp);


	world.add_system("sprite", SpriteSystem::new());


	while !wnd.should_close()
	{
		//world.run_once(&mut rotater);

		//world.run("rotater");
		world.run_all();

		wnd.poll_events();
		
		let kbd = wnd.keyboard();
		if kbd.key_pressed(Key::Escape)
		{
			break;
		}

		if kbd.key_pressed(Key::Q)
		{
			sink = None;
		}

		if kbd.key_pressed(Key::W)
		{
			let s = audio.play(&music);
			s.set_volume(0.1);

			sink = Some(s);
		}

		if kbd.key_pressed(Key::Space) || wnd.mouse().but_pressed(0)
		{
			audio.play_detached(&sound);
		}

		if kbd.key_pressed(Key::A)
		{
			world.set_active("rotater", false);
		}

		if kbd.key_pressed(Key::S)
		{
			world.set_active("rotater", true);
		}

		if kbd.key_pressed(Key::D)
		{
			world.remove_system("rotater");
		}

		if kbd.key_pressed(Key::Z)
		{
			let mut sp = world.get_mut::<Sprite>(&adv);
			sp.set_tag("attack1");
			sp.set_next_tag("idle");
			audio.play_detached(&sound);
		}

		if kbd.key_pressed(Key::V)
		{
			wnd.set_vsync(!wnd.vsync());
		}

		
		rend.add_world(&world);

		wnd.clear(Color::rgb(0.3, 0.5, 1.0));

		rend.render(wnd.projection_matrix());

		wnd.swap();
	}

	std::mem::drop(sink);

	Ok(())
}


fn main()
{
	run().unwrap_or_else(|e| oops(e));
}



