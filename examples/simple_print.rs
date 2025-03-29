fn main() -> i32
{
    let msg = "Just a string";/* message to be appeared on the screen */
    let mut scr = init_screen();				/* start the curses mode */
    let (row, col) = scr.getmaxyx();/* get the number of rows and columns */
    scr.mvprint(row/2,(col-msg.len())/2, format!("%s",msg))/* print the message at the center of the screen */
        .mvprintln(row-2,0,format!("This screen has %d rows and %d columns",row,col))
        .print("Try resizing your window(if possible) and then run this program again")
        .refresh()
        .getanych()
        .close();

    return 0;
}