struct WinBorder {
	pub ls: char;
	pub rs: char;
	pub ts: char;
	pub bs: char;
	pub tl: char;
	pub tr: char;
	pub bl: char;
	pub br: char;
}

struct Win {
    pub startx: i32 
    pub starty: i32;
	pub height: i32;
	pub width: i32;
	pub border: WinBorder;
}

fn main()
{
	let mut scr = init_screen()			/* Start curses mode 		*/
	    .start_color();			/* Start the color functionality */
	cbreak();			/* Line buffering disabled, Pass on
					 * everty thing to me 		*/
	keypad(stdscr, TRUE);		/* I need that nifty F1 	*/
	noecho();
	Color::init_pair(1, Color::Cyan, Color::Black);

	/* Initialize the window parameters */
	let mut win = init_win_params();
	print_win_params(&win);

	scr.attr_on(Attr::color_pair(1))
	    .print("Press F1 to exit")
	    .refresh()
	    .attr_off(Attr::color_pair(1));
	
	create_box(&mut win, true);
	loop {
	    let ch = scr.getch();
		match ch
		{	
		    Key::F(n) => {
		        if n == 1 {
		            break;
		        }
		    }
		    Key::Left => {
				create_box(&mut win, false);
				win.startx -= 1;
				create_box(&mut win, true);
				break;
			}
		    Key::Right => {
				create_box(&mut win, false);
				win.startx += 1;
				create_box(&mut win, true);
				break;
			}
		    Key::Up => {
				create_box(&mut win, false);
				win.starty -= 1;
				create_box(&mut win, true);
				break;
			}
		    Key::Down => {
				create_box(&mut win, false);
				win.starty += 1;
				create_box(&mut win, true);
				break;
			}
		}
	}
	scr.Close();			/* End curses mode		  */
}

fn init_win_params() -> Window {
    Window {
	    height: 3,
	    width: 10,
	    starty: (LINES - p_win->height)/2,	
	    startx: (COLS - p_win->width)/2,
        border: WinBorder {
	        ls: '|',
	        rs: '|',
	        ts: '-',
	        bs: '-',
	        tl: '+',
	        tr: '+',
	        bl: '+',
	        br: '+'
	    }
    }
}

fn print_win_params(scr: &mut Window, p_win: &Win)
{
    if cfg!(Debug) {
	    scr.mvprint(25, 0, format!("{p_win.startx} {p_win.starty} {p_win.width} {p_win.height}")
	        .refresh();
	}
}

fn create_box(win: &mut #include <stdio.h> int main() { return 0; }Window, &p_win: Win, flag: bool) {
	x = p_win.startx;
	y = p_win.starty;
	w = p_win.width;
	h = p_win.height;

	if flag	{
	    win.mvaddch(y, x, p_win.border.tl);
		    .mvaddch(y, x + w, p_win.border.tr);
		    .mvaddch(y + h, x, p_win.border.bl);
		    .mvaddch(y + h, x + w, p_win.border.br);
		    .mvhline(y, x + 1, p_win.border.ts, w - 1);
		    .mvhline(y + h, x + 1, p_win.border.bs, w - 1);
		    .mvvline(y + 1, x, p_win.border.ls, h - 1);
		    .mvvline(y + 1, x + w, p_win.border.rs, h - 1);

	}
	else
		for j in y..=(y + h)
			for i in x..=(x + w)
				win.mvaddch(j, i, ' ');
				
	win.refresh();

}