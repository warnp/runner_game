use sprite::Sprite;
use shader_manager::{Shaders, ShaderCouple};
use sprite_manager::SpriteManager;

pub struct modules_manager;

impl modules_manager {
    pub fn init(sprite_list: Vec<Sprite>, shaders: Shaders) {

        let screen_height = 768.0;
        let screen_width = 1024.0;
        let mut show_fps = false;
        let display = glium::glutin::WindowBuilder::new()
                          .with_vsync()
                          .with_dimensions(screen_width as u32, screen_height as u32)
                          .build_glium()
                          .unwrap();



        let mut vert = vec![Sprite::new("hero", -0.8, 0.0, [1.0, 0.0, 0.0, 1.0], 3, (0.1, 0.1), 0),
                            Sprite::new("mover0",
                                        0.8,
                                        -0.8,
                                        [1.0, 0.0, 0.0, 1.0],
                                        1,
                                        (0.2, 0.1),
                                        1),
                            Sprite::new("still",
                                        0.0,
                                        -1.0,
                                        [1.0, 0.0, 0.0, 1.0],
                                        2,
                                        (2.0, 1.0),
                                        2)];



        let mut shaders = Shaders::new(vec![&include_bytes!("../content/VFKM2.png")[..],
                                            &include_bytes!("../content/11532.png")[..],
                                            &include_bytes!("../content/NatureForests.png")[..],
                                            &include_bytes!("../content/hero.png")[..]]);
        shaders.compile_shaders(&display);

        let program = shaders.get_compiled_shader("simple_shader");

        let text_manager = TextWriter::new(0,
                                           (256, 256),
                                           (16, 16),
                                           0.050,
                                           (0.0, 0.0),
                                           "toto",
                                           true);
        let print_fps = TextWriter::new(0, (256, 256), (16, 16), 0.05, (0.0, 0.950), "fps", true);

        let mut sprite_manager = SpriteManager::new(vert, &display);
        let texture = shaders.get_texture_array(&display);


        let mut go_up = false;
        let jump_height = 0.5;
        let mut jump = false;
        let mut touch_ground = false;
        let mut loose = false;


        let mut engine_helper = EngineHelper::new();
        let mut buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
        let mut move_object = false;

        loop {

            // HUD
            sprite_manager.delete_sprite("toto");

            let fps_counter = engine_helper.get_fps();

            if engine_helper.get_iterator() % 10.0 == 0.0 {
                sprite_manager.delete_sprite("fps");
                for x in print_fps.get_string(&format!("{}fps", fps_counter.0)[..]) {
                    sprite_manager.add_sprite(x.clone());
                }
            }



            for x in text_manager.get_string(&format!("{}pts", engine_helper.get_iterator())[..]) {
                sprite_manager.add_sprite(x.clone());
            }


            // GAME LOGIC
            if loose {
                println!("LOOSER!!!");
            }


            {
                if move_object {
                    {
                        buffers = sprite_manager.delete_sprite("mover0");
                    }
                    {
                        buffers = sprite_manager.add_sprite(Sprite::new("mover0",
                                                                        1.5,
                                                                        -0.8,
                                                                        [1.0, 0.0, 0.0, 1.0],
                                                                        1,
                                                                        (0.2, 0.1),
                                                                        1));
                    }
                    move_object = false;
                }
            }

            {
                let sprite_mover = sprite_manager.get_sprite("mover0");
                if sprite_mover.vertices[1].position[0] >= -1.0 {
                    buffers = sprite_manager.move_sprite("mover0",
                                                         -0.1 * fps_counter.1 -
                                                         engine_helper.get_iterator() * 0.000001,
                                                         0.0);

                } else {
                    move_object = true;
                }
            }

            if !touch_ground {
                buffers = sprite_manager.move_sprite("hero", 0.0, -0.15 * fps_counter.1);
            }

            {
                let sprite_hero = sprite_manager.get_sprite("hero");
                for sp in &sprite_manager.get_sprite_list()
                                         .into_iter()
                                         .filter(|&x| x.name != "hero")
                                         .collect::<Vec<Sprite>>() {
                    let aa_bb = sp.get_aa_bb();
                    if sprite_hero.detect_collide(aa_bb.0, aa_bb.1) {

                        match sp.name {
                            "mover0" => {
                                loose = true;
                                break;
                            }
                            "still" => {
                                touch_ground = true;
                                break;
                            }
                            _ => {}
                        }

                    } else {

                        touch_ground = false;
                    }
                }
            }

            if jump && touch_ground {
                buffers = sprite_manager.move_sprite("hero", 0.0, 0.3);
                jump = false;

            }

            draw(&display,
                 sprite_manager.set_buffers(),
                 &program,
                 &texture,
                 screen_height,
                 screen_width);

            for ev in display.poll_events() {
                match ev {
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Space)) => jump = true,
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Space)) => jump = false,
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,_,Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                        _,
                                                        Some(glium::glutin::VirtualKeyCode::A)) => {
                        buffers = sprite_manager.add_sprite(Sprite::new("mover1",
                                                                        1.5,
                                                                        -0.8,
                                                                        [1.0, 0.0, 0.0, 1.0],
                                                                        1,
                                                                        (0.2, 0.1),
                                                                        2))
                    }
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                        _,
                                                        Some(glium::glutin::VirtualKeyCode::D)) => {
                        buffers = sprite_manager.delete_sprite("mover1")
                    }
                    glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                        _,
                                                        Some(glium::glutin::VirtualKeyCode::P)) => {
                        show_fps = true
                    }
                    glium::glutin::Event::Closed => return,
                    _ => (),
                }
            }


        }
    }
}
