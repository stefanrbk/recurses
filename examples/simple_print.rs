fn main() -> i32
{
    let msg = "Just a string";/* message to be appeared on the screen */
    let scr = init_screen();				/* start the curses mode */
    let (row, col) = scr.getmaxyx();/* get the number of rows and columns */
    print!(scr, row/2,(col-msg.len())/2,"%s",msg);/* print the message at the center of the screen */
    println!(scr, row-2,0,"This screen has %d rows and %d columns",row,col);
    print!(scr, "Try resizing your window(if possible) and then run this program again");
    scr.refresh();
    getch();
    scr.close();

    return 0;
}