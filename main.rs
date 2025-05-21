use druid::{
    Data, Lens, Env,
    AppLauncher, PlatformError, WindowDesc, 
    Widget, WidgetExt,
    FontDescriptor, FontFamily, Color, 
    RenderContext, PaintCtx, EventCtx
};

use druid::widget::{
    Label, Flex, Painter
};

use rand::Rng;

#[derive(Clone, Data, Lens)]
struct Board {
    cells: [[i8; 3]; 3],
    turn: i8,
    won: i8,
    theme_no: usize
}

#[derive(Clone)]
struct Thematic {
    main_bg: Color,
    neutral: Color,
    player_a: Color,
    player_b: Color,
    sym_neutral: char,
    sym_a: char,
    sym_b: char,
    sym_progress: char
}

// Thematic value
const THEMATICS: [Thematic; 5] = [
    Thematic {
        //main_bg: Color::rgb8(207, 206, 202),
        main_bg: Color::rgb8(255, 255, 255),
        neutral: Color::rgb8(0, 126, 150),
        player_a: Color::rgb8(104, 99, 93),
        player_b: Color::rgb8(138, 135, 129),
        sym_neutral: '●',
        sym_a: '■',
        sym_b: '▼',
        sym_progress: '⌛'
    },
    
    Thematic {
        main_bg: Color::rgb8(255, 255, 255),
        neutral: Color::rgb8(188, 121, 42),
        player_a: Color::rgb8(38, 75, 91),
        player_b: Color::rgb8(194, 210, 209),
        sym_neutral: '●',
        sym_a: '■',
        sym_b: '▼',
        sym_progress: '⌛'
    },

    Thematic {
        main_bg: Color::rgb8(255, 255, 255), 
        neutral: Color::rgb8(151, 157, 166), 
        player_a: Color::rgb8(104, 120, 140), 
        player_b: Color::rgb8(242, 226, 5),
        sym_neutral: '●',
        sym_a: '■',
        sym_b: '▼',
        sym_progress: '⌛'
    },
    
    Thematic {
        main_bg: Color::rgb8(255, 255, 255), 
        neutral: Color::rgb8(163, 217, 217), 
        player_a: Color::rgb8(73, 191, 170), 
        player_b: Color::rgb8(217, 67, 80),
        sym_neutral: '●',
        sym_a: '■',
        sym_b: '▼',
        sym_progress: '⌛'
    },
    
    Thematic {
        main_bg: Color::rgb8(255, 255, 255), 
        neutral: Color::rgb8(143, 223, 136), 
        player_a: Color::rgb8(0, 88, 91), 
        player_b: Color::rgb8(45, 166, 108),
        sym_neutral: '●',
        sym_a: '■',
        sym_b: '▼',
        sym_progress: '⌛'
    }];

const SIZE: f64 = 100.0;
const SPACE: f64 = 20.0;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(framer())
        .title("Tic - Tac - Toe")
        .window_size((1500.0, 750.0));

    let starter = Board {
        cells: [[0; 3]; 3],
        turn: 1,
        won: 0,
        theme_no: rand::thread_rng().gen_range(0..THEMATICS.len())
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(starter)
}

fn framer() -> impl Widget<Board> {
    let mont: FontDescriptor = FontDescriptor::new(
        FontFamily::new_unchecked("Montserrat")
    ).with_size(50.0);

    let mut board_cl = Flex::column()
        .with_flex_spacer(SPACE);

    for i in 0..3 {
        let mut r = Flex::row();

        for j in 0..3 {
            // Copies for tracking
            let i_bg = i;
            let j_bg = j;

            let i_ev = i;
            let j_ev = j;

            let i_lb = i;
            let j_lb = j;

            r.add_child(
                Flex::row()
                    .with_flex_spacer(SPACE/4.0)
                    .with_child(
                        Flex::column()
                            .with_flex_spacer(SPACE/8.0)
                            .with_child(
                                Label::new(
                                    move |data: &Board, _env: &Env| {
                                        let symbol: char;
                                        match data.cells[i_lb][j_lb] {
                                            1 => { symbol = THEMATICS[data.theme_no].sym_a; }
                                            -1 => { symbol = THEMATICS[data.theme_no].sym_b; }
                                            _ => { symbol = THEMATICS[data.theme_no].sym_neutral; }
                                        };
                                        format!("{}", symbol.to_string())
                                    }
                                )
                                .with_font(mont.clone())
                            )
                            .with_flex_spacer(SPACE/4.0)
                            .fix_height(SIZE)
                    )
                    .with_flex_spacer(SPACE/4.0)
                    .fix_width(SIZE)
                    .background(
                        Painter::new(
                            move |ctx: &mut PaintCtx, data: &Board, _env: &Env| {
                                let rnd = ctx.size().to_rounded_rect(SIZE/4.0);
                                let clr: &Color;
                                match data.cells[i_bg][j_bg] {
                                    1 => { clr = &THEMATICS[data.theme_no].player_a; }
                                    -1 => { clr = &THEMATICS[data.theme_no].player_b; }
                                    _ => { clr = &THEMATICS[data.theme_no].neutral; }
                                };
                                ctx.fill(rnd, clr);
                            }
                        )
                    )
                    .on_click(
                        move |ctx: &mut EventCtx, data: &mut Board, _env: &Env| {
                            let claim = data.cells[i_ev][j_ev];
                            if claim == 1 || claim == -1 || data.won != 0 {}
                            else if claim == 0 {
                                data.cells[i_ev][j_ev] = data.turn;
                                data.turn *= -1;

                                ctx.request_paint();

                                let w = winner(&data.cells);
                                if w != 0 {
                                    data.won = w;
                                }
                            }
                        }
                    )            
            );

            r.add_spacer(SPACE);
        }

        board_cl.add_child(r);
        if i < 3 - 1 { board_cl.add_spacer(SPACE); }
    }

    let score = Flex::row()
        .with_flex_spacer(SPACE/4.0)
        .with_child(
            Flex::column()
                .with_flex_spacer(SPACE/2.0)
                .with_child(
                    Label::new(
                        |data: &Board, _env: &Env| {
                            let symbol: char;
                            match data.won {
                                1 => { symbol = THEMATICS[data.theme_no].sym_a; }
                                -1 => { symbol = THEMATICS[data.theme_no].sym_b; }
                                -2 => { symbol = THEMATICS[data.theme_no].sym_neutral; }
                                _ => { symbol = THEMATICS[data.theme_no].sym_progress; }
                            };
                            format!("{}", symbol.to_string())
                        }
                    )
                    .with_font(mont.clone().with_size(250.0))
                )
                .with_flex_spacer(SPACE)
                .expand_height()
        )
        .with_flex_spacer(SPACE/4.0)
        .fix_height(SIZE * 4.0 + SPACE * 3.0)
        .fix_width(SIZE * 3.0 + SPACE * 2.0)
        .background(
            Painter::new(
                move |ctx: &mut PaintCtx, data: &Board, _env: &Env| {
                    let rnd = ctx.size().to_rounded_rect(SIZE/4.0);
                    let clr: &Color;
                    match data.won {
                        1 => { clr = &THEMATICS[data.theme_no].player_a; }
                        -1 => { clr = &THEMATICS[data.theme_no].player_b; }
                        _ => { clr = &THEMATICS[data.theme_no].neutral; }
                    };
                    ctx.fill(rnd, clr);
                }
            )
        );

    let mut theme_switch = Flex::column()
        .with_flex_spacer(SPACE);

    let theme_max = THEMATICS.len();
    for i in 0..theme_max {
        let i_bg = i;
        let i_st = i;

        theme_switch.add_child(
            Flex::row()
                .with_flex_spacer(SPACE/4.0)
                .with_child(
                    Flex::column()
                        .with_flex_spacer(SPACE/8.0)
                        .with_child(
                            Label::new("")
                                .with_font(mont.clone())
                        )
                        .with_flex_spacer(SPACE/4.0)
                        .fix_height(SIZE/2.0)
                )
                .with_flex_spacer(SPACE/4.0)
                .fix_width(SIZE/2.0)
                .background(
                    Painter::new(
                        move |ctx: &mut PaintCtx, _data: &Board, _env: &Env| {
                            let rnd = ctx.size().to_rounded_rect(SIZE/4.0);
                            let clr = &THEMATICS[i_bg].neutral;
                            ctx.fill(rnd, clr);
                        }
                    )
                )
                .on_click(
                    move |_ctx: &mut EventCtx, data: &mut Board, _env: &Env| {
                        data.theme_no = i_st;
                    }
                )            
            );

        if i < theme_max - 1 { theme_switch.add_spacer(SPACE); }
    }

        Flex::row()
            .with_flex_spacer(SPACE)
            .with_child(
                Flex::row()
                    .with_flex_spacer(SPACE)
                    .with_child(
                        board_cl
                            .with_flex_spacer(SPACE)
                            .fix_height(SIZE * 3.0 + SPACE * 4.0)
                    )
                    .with_spacer(SPACE)
                    .with_child(score)
                    .with_spacer(SPACE)
                    .with_child(
                        theme_switch
                            .with_flex_spacer(SPACE)
                            .fix_height(SIZE * theme_max as f64 + SPACE * (theme_max + 1) as f64)
                    )
                    .with_flex_spacer(SPACE)
                    .fix_height(SIZE * 4.0 + SPACE * 5.0)
                    .fix_width(SIZE * 7.0 + SPACE * 9.0)
                    .background(
                        Painter::new(
                            |ctx: &mut PaintCtx, data: &Board, _env: &Env| {
                                let rnd = ctx.size().to_rounded_rect(SIZE/4.0 + 4.0);
                                ctx.fill(rnd, &THEMATICS[data.theme_no].main_bg);
                            }
                        )
                    )
            )
            .with_flex_spacer(SPACE)
            .background(
                Painter::new(
                    |ctx: &mut PaintCtx, data: &Board, _env: &Env| {
                        let rnd = ctx.size().to_rect();
                        ctx.fill(rnd, &THEMATICS[data.theme_no].main_bg);
                    }
            ))
}

fn winner(dt: &[[i8; 3]; 3]) -> i8 {
    let mut sums = [0; 8];
    let mut claimed = 0;
    let mut curr: i8;

    for i in 0..3 {
        for j in 0..3 {
            curr = dt[i][j];
            sums[i] += curr;
            sums[j + 3] += curr;

            if i == j { sums[6] += curr; }
            if i + j == 3 - 1 { sums[7] += curr; }

            if curr != 0 { claimed += 1 };
        }
    }

    if sums.contains(&3) { 1 }
    else if sums.contains(&-3) { -1 }
    else if claimed == 9 { -2 }
    else { 0 }
}