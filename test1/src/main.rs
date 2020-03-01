
use jmge::*;
//use jmge::gui::*;
use std::rc::Rc;
use rand::Rng;
use std::any::Any;


fn oops(e: Error) -> !
{
	println!("Fatal error: {}", e.to_string());
	std::process::exit(1);
}


struct Sprite
{
	x: i32,
	y: i32,
	angle: f32,
	tex: Rc<Texture>,
}


fn load_canvases() -> (Canvas, Canvas)
{
	let c1 = Canvas::from_memory_file(include_bytes!("../../kanade.png")).unwrap_or_else(|e| oops(e));
	let c2 = Canvas::from_memory_file(include_bytes!("../../kanade2.png")).unwrap_or_else(|e| oops(e));

	(c1, c2)
}

pub struct Position
{
	pub x: f32,
	pub y: f32,
}

impl ecs::Component for Position
{
	fn as_any(&self) -> &dyn Any				{ self }
	fn as_any_mut(&mut self) -> &mut dyn Any	{ self }
}

pub struct Transform
{
	pub angle: f32,
	pub sx: f32,
	pub sy: f32,
}

impl ecs::Component for Transform
{
	fn as_any(&self) -> &dyn Any				{ self }
	fn as_any_mut(&mut self) -> &mut dyn Any	{ self }
}


fn main()
{
	let mut wnd = Window::new().unwrap_or_else(|e| oops(e));
	let mut rend = Renderer::new().unwrap_or_else(|e| oops(e));

	let (_cnv1, cnv2) = load_canvases();

	let mut atlas = TextureAtlas::new(2048);
	let tex = Rc::new(atlas.add(cnv2).unwrap_or_else(|e| oops(e)));




	let world = ecs::World::new();
	world.register_component::<Position>();
	world.register_component::<Transform>();


	let e = world.new_entity();

	let p = Position { x: 100.0, y: 200.0 };
	e.set_component(p);

	let pos = e.get_component::<Position>();
	println!("{},{}", pos.x, pos.y);
	std::mem::drop(pos);

	e.get_component_mut::<Position>().x = 300.0;


	world.iter_with(|pos: &Position| println!("Pos: {}, {}", pos.x, pos.y));


	let mut sprites = Vec::new();

	let mut rng = rand::thread_rng();
	for _ in 0..100
	{
		let sp = Sprite
			{
				x: rng.gen_range(0, 1920),
				y: rng.gen_range(0, 1080),
				angle: 0.0,
				tex: Rc::clone(&tex),
			};

		sprites.push(sp);
	}

	let mut f = 0.0;

	while !wnd.should_close()
	{
		f += 1.0;

		wnd.poll_events();
		
		let kbd = wnd.keyboard();
		if kbd.key_pressed(Key::Escape)
		{
			break;
		}

		for sp in sprites.iter_mut()
		{
			sp.angle = f / 100.0;
		}

		for sp in sprites.iter()
		{
			let (w, h) = sp.tex.size();

			let quad = Quad::new(&sp.tex)
				.with_pos(sp.x as f32, sp.y as f32)
				.with_angle(sp.angle)
				.with_origin(w as f32/2.0, h as f32/2.0);

			rend.add_quad(quad);
		}

		wnd.clear(Color::rgb(0.3, 0.5, 1.0));
		wnd.set_projection();

		rend.render();

		wnd.swap();
	}
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


