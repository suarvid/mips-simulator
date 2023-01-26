#![allow(dead_code)]
#![allow(unused)]
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::env;
use std::error::Error;
use std::io;
use std::time::Duration;
use std::time::Instant;
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use ui::App;

use self::assembler::run_assembler;
use self::mips::simulator::RunResult;

mod assembler;
mod mips;
mod ui;
mod utils;
mod valwriter;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let vec = run_assembler(args).expect("Failed to get instruction list from assembler!");

    let simulator = mips::simulator::Simulator::new(vec);

    //// setup terminal for ui
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);

    // create App and run it
    let app = App::new(simulator);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal after running
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Check if anything went wrong
    // This print is Ok, runs after shutting down
    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

// Main loop
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::ui(frame, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // Got event, user pressed key
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('s') => {
                        if !app.is_finished() {
                            let res = app.step();
                            if let RunResult::Failure(msg) = res {
                                eprintln!("{}", msg);
                                std::process::exit(1);
                            }
                        }
                    }
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('d') => app.set_val_writer('d'),
                    KeyCode::Char('h') => app.set_val_writer('h'),
                    KeyCode::Char('b') => app.set_val_writer('b'),
                    KeyCode::Char('r') => app.reset(),
                    KeyCode::Char('g') => app.toggle_run(), // g as in GO, todo should be running here..
                    KeyCode::Char('j') => app.reg_file_forward(), // Vim-down
                    KeyCode::Char('k') => app.reg_file_backward(), // Vim-up
                    KeyCode::Down => app.data_mem_forward(),
                    KeyCode::Up => app.data_mem_backward(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}


#[cfg(test)]
mod tests {
    use ux::u5;

    use crate::{mips::Simulator};

    use super::*;

    #[test]
    fn simple_loop_test() {
        let args = vec![
            "".to_string(), // program name
            "test_files/integration_test1.asm".to_string(),
            "test_files/integration_test1_instruction_output.txt".to_string(),
            "test_files/integration_test1_listing_output.txt".to_string()
            ];
        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        // t1 should be 3, t0 should be zero
        let regs = sim.get_registers();
        let t0 = regs.get(8).unwrap();
        let t1 = regs.get(9).unwrap();
        assert_eq!(t0.1, 0);
        assert_eq!(t1.1, 3);

        let got_expected_res = regs.iter().map(|reg| {
            if reg.0 == u5::new(9) {
                reg.1 == 3
            } else {
                reg.1 == 0
            }
        }).all(|b| b);

        assert!(got_expected_res);
        
    }

    #[test]
    fn sw_integration_test() {
        let args = vec![
            "".to_string(),
            "test_files/sw_integration_test.asm".to_string(),
            "test_files/sw_integration_test_instruction_output.txt".to_string(),
            "test_files/sw_integration_test_listing_output.txt".to_string(),
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        let data_mem = sim.get_data_mem();
        assert_eq!(data_mem.get(7).unwrap().1, 10);
        assert_eq!(data_mem.get(11).unwrap().1, 9);
        assert_eq!(data_mem.get(15).unwrap().1, 8);
        assert_eq!(data_mem.get(19).unwrap().1, 7);
        assert_eq!(data_mem.get(23).unwrap().1, 6);
        assert_eq!(data_mem.get(27).unwrap().1, 5);
        assert_eq!(data_mem.get(31).unwrap().1, 4);
        assert_eq!(data_mem.get(35).unwrap().1, 3);
        assert_eq!(data_mem.get(39).unwrap().1, 2);
        assert_eq!(data_mem.get(43).unwrap().1, 1);
        assert_eq!(data_mem.get(47).unwrap().1, 0);

        // t0 should be 0
        // t1 should be 44
        let reg_file = sim.get_registers();

        let t0_val = reg_file.get(8).unwrap();
        let t1_val = reg_file.get(9).unwrap();
        let t2_val = reg_file.get(10).unwrap();

        assert_eq!(t0_val.1, 0);
        assert_eq!(t1_val.1, 44);
        assert_eq!(t2_val.1, 1);
        
    }


    #[test]
    fn lw_integration_test() {
        let args = vec![
            "".to_string(),
            "test_files/lw_integration_test.asm".to_string(),
            "test_files/lw_integration_test_instruction_output.txt".to_string(),
            "test_files/lw_integration_test_listing_output.txt".to_string(),
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        let data_mem = sim.get_data_mem();
        let byte_1 = data_mem.get(6).unwrap();
        let byte_2 = data_mem.get(7).unwrap();
        assert_eq!(byte_1.1, 39);
        assert_eq!(byte_2.1, 16);

        let word_val = ((byte_1.1 as i32) << 8) + (byte_2.1 as i32);
        assert_eq!(word_val, 10000);

    }

    #[test]
    fn lw_with_offset_integration_test() {
        let args = vec![
            "".to_string(),
            "test_files/lw_with_offset.asm".to_string(),
            "test_files/lw_with_offset_integration_test_instruction_output.txt".to_string(),
            "test_files/lw_with_offset_integration_test_listing_output.txt".to_string(),
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        let regs = sim.get_registers();
        let t1_val = regs.get(9).unwrap();
        assert_eq!(t1_val.1, 200);
        let data_mem = sim.get_data_mem();
        let mem_val = data_mem.get(11).unwrap();
        assert_eq!(mem_val.1, 200);
    }

    #[test]
    fn sw_lw_neg_val_test() {
        let args = vec![
            "".to_string(),
            "test_files/sw_lw_neg_val_test.asm".to_string(),
            "test_files/sw_lw_neg_val_test_instr.txt".to_string(),
            "test_files/sw_lw_neg_val_test_listing.txt".to_string()
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        let regs = sim.get_registers();
        let data_mem = sim.get_data_mem();
        // t0 and t2 should both be -100,
        // word starting at 4 should be -100
        // -> bytes should be 4 = 255, 5 = 255, 6 = 255, 7 = 156
        let byte_0 = data_mem.get(4).unwrap().1;
        let byte_1 = data_mem.get(5).unwrap().1;
        let byte_2 = data_mem.get(6).unwrap().1;
        let byte_3 = data_mem.get(7).unwrap().1;
        assert_eq!(255, byte_0);
        assert_eq!(255, byte_1);
        assert_eq!(255, byte_2);
        assert_eq!(156, byte_3);
        let word_val = 
            ((byte_0 as u32).overflowing_shl(24).0 +
            (byte_1 as u32).overflowing_shl(16).0 +
            (byte_2 as u32).overflowing_shl(8).0 +
            (byte_3 as u32)) as i32;
        assert_eq!(word_val, -100);

        let t0_val = regs.get(8).unwrap().1;
        let t2_val = regs.get(10).unwrap().1;
        assert_eq!(t0_val, -100);
        assert_eq!(t2_val, -100);

    }

    #[test]
    fn beq_backwards() {
        let args = vec![
            "".to_string(),
            "test_files/beq_backwards.asm".to_string(),
            "test_files/beq_backwards_test_instr.txt".to_string(),
            "test_files/beq_backwards_test_listing.txt".to_string()
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        // Write program :)
        let regs = sim.get_registers();
        let t0_val = regs.get(8).unwrap().1;
        let t1_val = regs.get(9).unwrap().1;
        assert_eq!(t0_val, 7);
        assert_eq!(t1_val, 8);
    }

    #[test]
    fn jr_forwards() {
        let args = vec![
            "".to_string(),
            "test_files/jr_forwards.asm".to_string(),
            "test_files/jr_forwards_instr.txt".to_string(),
            "test_files/jr_forwards_listing.txt".to_string(),
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
            }
        }

        let regs = sim.get_registers();
        let t0_val = regs.get(8).unwrap().1;
        let t1_val = regs.get(9).unwrap().1;
        let t2_val = regs.get(10).unwrap().1;
        assert_eq!(t0_val, 16);
        assert_eq!(t1_val, 0);
        assert_eq!(t2_val, 10);
    }

    #[test]
    fn jr_backwards() {
        let args = vec![
            "".to_string(),
            "test_files/jr_backwards.asm".to_string(),
            "test_files/jr_backwards_instr.txt".to_string(),
            "test_files/jr_backwards_listing.txt".to_string(),
        ];

        let instructions = run_assembler(args).unwrap();
        let mut sim = Simulator::new(instructions);
        loop {
            let step_res = sim.step();
            if step_res != RunResult::Success {
                assert_eq!(step_res, RunResult::Completed);
                break;
                }
        }

        let t0_val = sim.get_registers().get(8).unwrap().1;
        let t1_val = sim.get_registers().get(9).unwrap().1;
        let t2_val = sim.get_registers().get(10).unwrap().1;
        let t3_val = sim.get_registers().get(11).unwrap().1;
        assert_eq!(t0_val, 10);
        assert_eq!(t1_val, 10);
        assert_eq!(t2_val, 20);
        assert_eq!(t3_val, 1);
    }

}
