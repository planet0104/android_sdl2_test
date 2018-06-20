struct Sprite{
	texture: *mut SDL_Texture,
	w: Uint16,
	h: Uint16
}

/* Adapted from SDL's testspriteminimal.c */
unsafe fn load_sprite(file: &str, renderer: *mut SDL_Renderer) -> Sprite{
	let mut sprite = Sprite{
        texture: ptr::null_mut(),
        w: 0,
        h: 0
    };
	
    let mut surface: *mut SDL_Surface = ptr::null_mut();

    /* Load the sprite image */
    surface = (SDL2.SDL_LoadBMP_RW)((SDL2.SDL_RWFromFile)(CString::new("rust.bmp").unwrap().as_ptr(), CString::new("rb").unwrap().as_ptr()), 1);
    if surface.is_null(){
        panic!("Couldn't load {}: {}\n", file, get_sdl_error());
    }
    sprite.w = (*surface).w as u16;
    sprite.h = (*surface).h as u16;

    /* Create texture from the image */
    sprite.texture = (SDL2.SDL_CreateTextureFromSurface)(renderer, surface);
    println!("a10");
    if sprite.texture.is_null() {
        println!("a11");
        println!("Couldn't create texture: {}\n", get_sdl_error());
        (SDL2.SDL_FreeSurface)(surface);
        return sprite;
    }
    (SDL2.SDL_FreeSurface)(surface);

    sprite
}

unsafe fn draw(lib: &LibSDL2, window: *mut SDL_Window, renderer: *mut SDL_Renderer, sprite:&Sprite){
	let (mut w, mut h)=(0, 0);
	(lib.SDL_GetWindowSize)(window, &mut w, &mut h);
	let dest_rect = SDL_Rect{x: w/2 - sprite.w as i32/2, y: h/2 - sprite.h as i32/2, w: sprite.w as i32, h: sprite.h as i32};
	/* Blit the sprite onto the screen */
	(lib.SDL_RenderCopy)(renderer, sprite.texture, ptr::null_mut(), &dest_rect);
}



// let mut window: *mut SDL_Window = ptr::null_mut();
        // let mut renderer: *mut SDL_Renderer = ptr::null_mut();

        // if (lib.SDL_CreateWindowAndRenderer)(0, 0, 0, &mut window, &mut renderer) < 0 {
        //     //panic!("SDL_CreateWindowAndRenderer Error!");
        //     return -100;
        // }
        
        // let sprite = load_sprite(&lib, "rust.bmp", renderer);
        // if sprite.texture.is_null() {
        //     //panic!("图片加载失败!");
        //     return -101;
        // }


        return 200;

        /* Main render loop */
        // let mut done = false;
        // let mut event = mem::uninitialized();
        // while !done {
        //     /* Check for events */
        //     while (lib.SDL_PollEvent)(&mut event) == 1 {
        //         if event.type_ == SDL_EventType::SDL_QUIT as u32 || event.type_ == SDL_EventType::SDL_KEYDOWN as u32 || event.type_ == SDL_EventType::SDL_FINGERDOWN as u32 {
        //             done = true;
        //         }
        //     }
                        
        //     /* Draw a gray background */
        //     (lib.SDL_SetRenderDrawColor)(renderer, 0xA0, 0xA0, 0xA0, 0xFF);
        //     (lib.SDL_RenderClear)(renderer);
            
        //     draw(&lib, window, renderer, &sprite);
        
        //     /* Update the screen! */
        //     (lib.SDL_RenderPresent)(renderer);
            
        //     (lib.SDL_Delay)(10);
        // }