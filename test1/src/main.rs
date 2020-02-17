
use jmge::*;
use jmge::gui::*;
use std::rc::Rc;
use rand::Rng;


fn oops(e: Error) -> !
{
	println!("Fatal error: {}", e.to_string());
	std::process::exit(1);
}

fn main()
{
	let mut wnd = Window::new().unwrap_or_else(|e| oops(e));

	let mut gui = Gui::new();

	while !wnd.should_close()
	{
		wnd.poll_events();
		gui.process_input(wnd.input());
		
		let kbd = wnd.keyboard();
		if kbd.key_pressed(Key::Escape)
		{
			break;
		}

		wnd.clear(Color::rgb(0.3, 0.5, 1.0));
		wnd.set_projection();
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

