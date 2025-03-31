use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main()
{
    let args: Vec<String> = env::args().collect();
    if(args.len() != 2) {
        print!("Usage: {args[0]} <file name>\n");
        std::process::exit(1);
    }
    
    // Create a path to the desired file
    let path = Path::new(args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {
            print!("couldn't open {display}: {why}");
            std::process::exit(1)},
        Ok(file) => file,
    };
    
    let mut f = BufReader::new(file);
    
    let mut scr = init_screen();				/* Start curses mode */
    let (row, col) = scr.getmaxyx();		/* find the boundaries of the screeen */

    let mut prev = '\0';
    for line in f.lines() {
    	/* read the file till we reach the end */
        for ch in line.expect("lines failed").chars() {
            let (y, x) = scr.getyx();		/* get the current curser position */
            if(y == (row - 1))			/* are we are at the end of the screen */
            {
                scr.print("<-Press Any Key->")	/* tell the user to press a key */
                    .getanych()
                    .clear()				/* clear the screen */
                    .move(0, 0);			/* start at the beginning of the screen */
            }
            if(prev == '/' && ch == '*')    	/* If it is / and * then only
                                     	 * switch bold on */    
            {
                scr.attr_on(Attr::BOLD)			/* cut bold on */
                
                    .move(y, x - 1)			/* back up one space */
                    .print("/*"); 		/* The actual printing is done here */
            }
            else
                scr.print("{ch}");
            scr.refresh();
            
            if(prev == '*' && ch == '/')
                scr.attr_off(Attr::BOLD);        		/* Switch it off once we got *
                                 	 * and then / */
            prev = ch;
        }
    }
    scr.close();                       	/* End curses mode */
}