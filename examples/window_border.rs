fn main() {
    WINDOW *my_win;
	int startx, starty;
	int ch;

	let mut scr = init_screen();			/* Start curses mode 		*/
	cbreak();			/* Line buffering disabled, Pass on
					 * everty thing to me 		*/
	keypad(stdscr, TRUE);		/* I need that nifty F1 	*/

	let height = 3;
	let width = 10;
	let mut starty = (LINES - height) / 2;	/* Calculating for a center placement */
	let mut startx = (COLS - width) / 2;	/* of the window		*/
	scr.print("Press F1 to exit")
	    .refresh();
	let mut my_win = create_newwin(height, width, starty, startx);

    loop {
        let ch = scr.getch();
	    match(ch)
		{
		    Key::F(n) => {
		        if n == 1
		            break;
		    }
		    Key::LEFT => {
				destroy_win(my_win);
				startx -= 1;
				my_win = create_newwin(height, width, starty,startx);
			},
			Key::RIGHT => {
				destroy_win(my_win);
				startx += 1;
				my_win = create_newwin(height, width, starty,startx);
			},
			Key::UP => {
				destroy_win(my_win);
				starty -= 1;
				my_win = create_newwin(height, width, starty,startx);
			},
			Key::DOWN => {
				destroy_win(my_win);
				starty += 1;
				my_win = create_newwin(height, width, starty,startx);
			}
		}
	}
		
	scr.close();			/* End curses mode		  */
}

fn create_newwin(height: i32, width: i32, starty: i32, startx: i32) -> Window {

	let mut local_win = Window::new(height, width, starty, startx);
	local_win.box(0 , 0);		/* 0, 0 gives default characters 
					 * for the vertical and horizontal
					 * lines			*/
	local_win.refresh();		/* Show that box 		*/

	return local_win;
}

destroy_win(local_win: &mut Window)
{	
	/* box(local_win, ' ', ' '); : This won't produce the desired
	 * result of erasing the window. It will leave it's four corners 
	 * and so an ugly remnant of window. 
	 */
	local_win.border(' ', ' ', ' ',' ',' ',' ',' ',' ');
	/* The parameters taken are 
	 * 1. win: the window on which to operate
	 * 2. ls: character to be used for the left side of the window 
	 * 3. rs: character to be used for the right side of the window 
	 * 4. ts: character to be used for the top side of the window 
	 * 5. bs: character to be used for the bottom side of the window 
	 * 6. tl: character to be used for the top left corner of the window 
	 * 7. tr: character to be used for the top right corner of the window 
	 * 8. bl: character to be used for the bottom left corner of the window 
	 * 9. br: character to be used for the bottom right corner of the window
	 */
	local_win.refresh();
	local_win.close();
}