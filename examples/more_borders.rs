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
	init_win_params(&win);
	print_win_params(&win);

	scr.attr_on(Attr::color_pair(1))
	    .print("Press F1 to exit")
	    .refresh()
	    .attr_off(Attr::color_pair(1));
	
	create_box(&win, TRUE);
	while((ch = getch()) != KEY_F(1))
	{	switch(ch)
		{	case KEY_LEFT:
				create_box(&win, FALSE);
				--win.startx;
				create_box(&win, TRUE);
				break;
			case KEY_RIGHT:
				create_box(&win, FALSE);
				++win.startx;
				create_box(&win, TRUE);
				break;
			case KEY_UP:
				create_box(&win, FALSE);
				--win.starty;
				create_box(&win, TRUE);
				break;
			case KEY_DOWN:
				create_box(&win, FALSE);
				++win.starty;
				create_box(&win, TRUE);
				break;	
		}
	}
	endwin();			/* End curses mode		  */
	return 0;
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

fn print_win_params(&mut scr: Window, &p_win: Win)
{
    if cfg!(Debug) {
	    scr.mvprint(25, 0, format!("{p_win.startx} {p_win.starty} {p_win.width p_win->height);
	refresh();
	}
#endif
}
void create_box(WIN *p_win, bool flag)
{	int i, j;
	int x, y, w, h;

	x = p_win->startx;
	y = p_win->starty;
	w = p_win->width;
	h = p_win->height;

	if(flag == TRUE)
	{	mvaddch(y, x, p_win->border.tl);
		mvaddch(y, x + w, p_win->border.tr);
		mvaddch(y + h, x, p_win->border.bl);
		mvaddch(y + h, x + w, p_win->border.br);
		mvhline(y, x + 1, p_win->border.ts, w - 1);
		mvhline(y + h, x + 1, p_win->border.bs, w - 1);
		mvvline(y + 1, x, p_win->border.ls, h - 1);
		mvvline(y + 1, x + w, p_win->border.rs, h - 1);

	}
	else
		for(j = y; j <= y + h; ++j)
			for(i = x; i <= x + w; ++i)
				mvaddch(j, i, ' ');
				
	refresh();

}