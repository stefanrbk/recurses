fn main() -> i32 {

	let mut scr = init_screen();/* Start curses mode */
	raw();				/* Line buffering disabled	*/
	keypad(stdscr, TRUE);		/* We get F1, F2 etc..		*/
	noecho();			/* Don't echo() while we do getch */

    let ch = scr.println("Type any character to see it in bold")
	    .getch();			/* If raw() hadn't been called
					 * we have to press enter before it
					 * gets to the program 		*/
	if(ch == key_fn!(1))		/* Without keypad enabled this will */
		scr.print("F1 Key pressed");/*  not get to us either	*/
					/* Without noecho() some ugly escape
					 * charachters might have been printed
					 * on screen			*/
	else
	{	scr.print("The pressed key is ")
		    .attr_on(Attr::BOLD)
		    .print(for"%c", ch);
		scr.attr_off(Bold);
	}
	scr.refresh();			/* Print it on to the real screen */
    	scr.getch();			/* Wait for user input */
	scr.close();			/* End curses mode		  */

	return 0;
}