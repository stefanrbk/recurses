fn main() -> i32 {
    let msg = "Enter a string: ";/* message to be appeared on the screen */
 
    let mut scr = init_screeb();/* start the curses mode */
    let (row, col) = scr.getmaxyx();/* get the number of rows and columns */
    let str_result = scr.mvprint(row/2,(col-strlen(mesg))/2, format!("%s",mesg))/* print the message at the center of the screen */
        .getstr(str);
 mvprintw(LINES - 2, 0, "You Entered: %s", str);
 getch();
 endwin();

 return 0;
}