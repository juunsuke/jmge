
use jmge::*;
use std::rc::Rc;
use rand::Rng;


fn oops(e: Error) -> !
{
	println!("Fatal error: {}", e.to_string());
	std::process::exit(1);
}


fn load_canvases() -> (Canvas, Canvas)
{
	let c1 = Canvas::from_memory_file(include_bytes!("../../kanade.png")).unwrap_or_else(|e| oops(e));
	let c2 = Canvas::from_memory_file(include_bytes!("../../kanade2.png")).unwrap_or_else(|e| oops(e));

	(c1, c2)
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
			let rs = world.get::<RotSpeed>(&e).0;
			r.angle = self.f*rs/100.0;
		}
	}
}


fn run() -> Result<(), Error>
{
	let mut wnd = Window::new()?;
	let mut rend = Renderer::new()?;

	let (_cnv1, cnv2) = load_canvases();

	let mut atlas = TextureAtlas::new(2048);
	let tex = Rc::new(atlas.add(cnv2)?);


	let audio = Audio::new()?;

	let sound = Sound::from_file("what.ogg")?;
	let music = Sound::from_file("Battleship.ogg")?;
	let mut sink = None;


	let mut world = World::new();
	world.register::<Renderable>();
	world.register::<RotSpeed>();


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

		
		rend.add_world(&world);

		wnd.clear(Color::rgb(0.3, 0.5, 1.0));
		wnd.set_projection();

		rend.render();

		wnd.swap();
	}

	std::mem::drop(sink);

	Ok(())
}


fn main()
{
	run().unwrap_or_else(|e| oops(e));
}



/*
fn main()
{
	let mut rng = rand::thread_rng();

	let mut wnd = Window::new().unwrap_or_else(|e| oops(e));


	let mut atlas = TextureAtlas::new(2048);

	//let kanade = Canvas::from_file("kanade2.png").unwrap();

	//let tex = Rc::new(atlas.add(Canvas::from_file("kanade2.png").unwrap_or_else(|e| oops(e))).unwrap_or_else(|e| oops(e)));
	let tex = Rc::new(atlas.add(Canvas::from_memory_file(include_bytes!("../../kanade2.png")).unwrap_or_else(|e| oops(e))).unwrap_or_else(|e| oops(e)));
	let tex2 = Rc::new(atlas.add(Canvas::from_file("kanade.png").unwrap_or_else(|e| oops(e))).unwrap_or_else(|e| oops(e)));

	let mut cnv = Canvas::new(500, 200, Color::rgb(1.0, 0.6, 0.4));

	//let _fnt = Font::new(24).unwrap_or_else(|e| oops(e));
	let fnt = Font::from_canvas(&Canvas::from_file("bmpfont8x16.png").unwrap_or_else(|e| oops(e))).unwrap_or_else(|e| oops(e));

	cnv.draw_text(&fnt, 20, 5, Color::rgba(0.2, 0.5, 1.0, 1.0), "Hello, world!");

	let tex3 = Rc::new(atlas.add(cnv).unwrap_or_else(|e| oops(e)));

	atlas.resize(4096).unwrap_or_else(|e| oops(e));

	//let tex = Rc::new(Texture::Raw(RawTexture::from_file("kanade2.png").unwrap_or_else(|e| oops(e))));
	//let tex2 = Rc::new(Texture::Raw(RawTexture::from_file("kanade.png").unwrap_or_else(|e| oops(e))));

	let mut sprites = Vec::new();
	let mut sb = SpriteBatch::new();

	let (tw, th) = tex.size();

	for _ in 0..100
	{
		let mut sp = Sprite::new(&Rc::clone(&tex));
		sp.set_pos(rng.gen_range(0, 1920), rng.gen_range(0, 1080));
		sp.set_origin(tw as i32/2, th as i32/2);

		sprites.push(sb.add(sp));
	}

	let (tw, th) = tex2.size();

	sprites[10].borrow_mut().set_texture(&Rc::clone(&tex2));
	sprites[90].borrow_mut().set_texture(&Rc::clone(&tex2));
	
	sprites[10].borrow_mut().set_origin(tw as i32/2, th as i32/2);
	sprites[90].borrow_mut().set_origin(tw as i32/2, th as i32/2);

	sprites[90].borrow_mut().set_color(Color::rgba(0.6, 1.0, 0.8, 0.7));


	sprites[99].borrow_mut().set_texture(&Rc::clone(&tex3));
	sprites[99].borrow_mut().set_origin(0, 0);


	let mut tcnv = Canvas::new(512, 384, Color::rgb(0.2, 0.2, 0.2));
	let ttex = Rc::new(Texture::from_canvas(&tcnv));
	let mut sp = Sprite::new(&Rc::clone(&ttex));
	sp.set_pos(100, 10);

	let _sp = sb.add(sp);


	let mut f: u32 = 0;

	while !wnd.should_close()
	{
		f += 1;

		for sp in sprites.iter()
		{
			sp.borrow_mut().set_angle(f as f32 / 100.0);
		}

		sprites[99].borrow_mut().set_angle(0.0);

		wnd.poll_events();

		let mouse = wnd.mouse();
		if mouse.moved()
		{
			let (x, y) = mouse.pos();

			sprites[99].borrow_mut().set_pos(x, y);
		}

	
		let kbd = wnd.keyboard();
		if kbd.key_pressed(Key::Escape)
		{
			break;
		}

		//tcnv.clear(Color::rgb(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)));
		tcnv.clear(Color::hsva(f as f32/1000.0, 1.0, 1.0, 0.5));
		ttex.update(&tcnv);


		wnd.clear(Color::rgb(0.3, 0.5, 1.0));

		wnd.set_projection();
		sb.draw();

		wnd.swap();
	}
}
*/


