use std::process;

fn main() {
    let mut scr = init_screen();			/* Start curses mode 		*/
	if !scr.has_colors() {	
	    scr.close();
		printf("Your terminal does not support color\n");
		process::exit(1);
	}
	scr.start_color();			/* Start color 			*/
	scr.init_pair(1, Color::Red, Color::Black);

	scr.attr_on(Attr::color_pair(1));
	print_in_middle(&mut scr, LINES / 2, 0, 0, "Viola !!! In color ...");
	scr.attr_off(Attr::color_pair(1))
    	.getanych()
	    .close();
}

fn print_in_middle(win: &mut Window, starty: i32, startx: i32, width: i32, string: &str) {
	let (y, x) = win.getyx();
	let x = if startx != 0 { startx } else { x };
	let y = if starty != 0 { starty } else { y };
	let width = if width == 0 { 80 } else { width };

	let length = string.len();
	let temp = (width as f32 - length as f32)/ 2f32;
	let x = startx + temp as i32;
	win.mvprint(y, x, string)
	    .refresh();
}