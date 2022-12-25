use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Write};

fn _main() -> Result<()> {
    // let mut stdout = stdout();

    // stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // for y in 0..40 {
    //     for x in 0..150 {
    //         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
    //             // in this loop we are more efficient by not flushing the buffer.
    //             stdout
    //                 .queue(cursor::MoveTo(x, y))?
    //                 .queue(style::PrintStyledContent("█".magenta()))?;
    //         }
    //     }
    // }
    // stdout.flush()?;
    terminal::enable_raw_mode().expect("Could not turn on Raw mode");
    print_events();
    Ok(())
}
const ARROW: &str = "> ";
const MARGIN: &str = "  ";

fn print_events() -> crossterm::Result<()> {
    let mut arrow_position = 0;
    let options = vec!["foo", "bar", "cow"];
    draw_options(arrow_position, &options)?;
    loop {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => {
                    println!("{:?}", event);
                    if event.code == KeyCode::Down {
                        arrow_position = std::cmp::min(options.len() - 1, arrow_position + 1);
                    }
                    if event.code == KeyCode::Up {
                        arrow_position = std::cmp::max(0, arrow_position - 1);
                    }
                    draw_options(arrow_position, &options)?;
                }
                Event::Mouse(event) => println!("{:?}", event),
                #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                Event::Paste(data) => println!("paste with '{}'", data),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }
    Ok(())
}

fn draw_options(arrow_position: u16, options: &Vec<&str>) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..options.len() {
        stdout
            .queue(cursor::MoveTo(0, y as u16))?
            .queue(style::PrintStyledContent(
                (if y == arrow_position as usize {
                    ARROW
                } else {
                    MARGIN
                })
                .magenta(),
            ))?
            .queue(style::PrintStyledContent(options[y].magenta()))?;
    }
    stdout.flush()?;
    Ok(())
}

fn draw_arrow(arrow_position: u16) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(cursor::MoveTo(0, arrow_position as u16))?
        .queue(style::PrintStyledContent(ARROW.magenta()))?;
    stdout.flush()?;
    Ok(())
}
