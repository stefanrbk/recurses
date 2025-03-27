fn main() -> i32
{
    let msg = "Just a string";/* message to be appeared on the screen */
    let scr = init_screen();				/* start the curses mode */
    let (row, col) = scr.getmaxyx();/* get the number of rows and columns */
    print!(row/2,(col-msg.len())/2,"%s",msg);
                                	/* print the message at the center of the screen */
 mvprintw(row-2,0,"This screen has %d rows and %d columns\n",row,col);
 printw("Try resizing your window(if possible) and then run this program again");
 refresh();
 getch();
 endwin();

 return 0;
}