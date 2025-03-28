fn main() -> i32 {

	let mut scr = init_screen();/* Start curses mode */
	raw();				/* Line buffering disabled	*/
	keypad(stdscr, TRUE);		/* We get F1, F2 etc..		*/
	noecho();			/* Don't echo() while we do getch */

    	println!(scr, "Type any character to see it in bold");
	ch = scr.getch();			/* If raw() hadn't been called
					 * we have to press enter before it
					 * gets to the program 		*/
	if(ch == key_fn!(1))		/* Without keypad enabled this will */
		print!(scr, "F1 Key pressed");/*  not get to us either	*/
					/* Without noecho() some ugly escape
					 * charachters might have been printed
					 * on screen			*/
	else
	{	print!(scr, "The pressed key is ");
		scr.attr_on(Bold);
		print!(scr, "%c", ch);
		scr.attr_off(Bold);
	}
	scr.refresh();			/* Print it on to the real screen */
    	scr.getch();			/* Wait for user input */
	scr.close();			/* End curses mode		  */

	return 0;
}