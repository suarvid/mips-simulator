use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::{
    mips::{simulator::RunResult, Simulator},
    valwriter::{BinValWriter, DecValWriter, HexValWriter, NumValWriter},
};

struct StatefulTable<T> {
    state: TableState,
    items: Vec<T>,
}

impl<T> StatefulTable<T> {
    fn new() -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items: Vec::new(),
        }
    }

    fn next(&mut self) {
        let new_selection = match self.state.selected() {
            Some(curr_selection) => {
                if curr_selection >= self.items.len() - 1 {
                    0
                } else {
                    curr_selection + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(new_selection));
    }

    fn previous(&mut self) {
        let new_selection = match self.state.selected() {
            Some(curr_selection) => {
                if curr_selection == 0 {
                    self.items.len() - 1
                } else {
                    curr_selection - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(new_selection));
    }
}

pub struct App {
    instr_mem: StatefulTable<InstructionViewModel>,
    //instr_mem: StatefulList<(&'a str, usize, usize, usize, usize, usize)>,
    data_mem: StatefulTable<DataViewModel>,
    registers: StatefulTable<RegisterViewModel>,
    pc: String, // Just show PC
    simulator: Simulator,
    val_writer: Box<dyn NumValWriter>,
    run_flag: bool,
    finish: bool,
}

impl App {
    pub fn step(&mut self) -> RunResult {
        if !self.is_finished() {
            match self.simulator.step() {
                RunResult::Success => return RunResult::Success,
                RunResult::Completed => self.finish = true,
                RunResult::Failure(msg) => return RunResult::Failure(msg),
            }
        }
        RunResult::Completed
    }

    pub fn reset(&mut self) {
        self.finish = false;
        self.run_flag = false;
        self.simulator.reset();
    }

    pub fn reg_file_forward(&mut self) {
        self.registers.next()
    }

    pub fn reg_file_backward(&mut self) {
        self.registers.previous()
    }

    pub fn data_mem_forward(&mut self) {
        self.data_mem.next()
    }

    pub fn data_mem_backward(&mut self) {
        self.data_mem.previous()
    }

    pub fn is_finished(&self) -> bool {
        self.finish
    }
}

struct InstructionViewModel {
    addr: String,
    hex_rep: String,
    op: String,
    rs: String,
    rt: String,
    rd: String,
    imm: String,
    shamt: String,
    func: String,
    j_target: String,
    mnemonic: String,
}

impl InstructionViewModel {
    fn new(
        addr: String,
        hex_rep: String,
        op: String,
        rs: String,
        rt: String,
        rd: String,
        imm: String,
        shamt: String,
        func: String,
        j_target: String,
        mnemonic: String,
    ) -> InstructionViewModel {
        InstructionViewModel {
            addr,
            hex_rep,
            op,
            rs,
            rt,
            rd,
            imm,
            shamt,
            func,
            j_target,
            mnemonic,
        }
    }
}

struct DataViewModel {
    addr: String,
    contents: String,
}

impl DataViewModel {
    fn new(addr: String, contents: String) -> DataViewModel {
        DataViewModel { addr, contents }
    }
}

struct RegisterViewModel {
    register_name: String,
    contents: String,
}

impl RegisterViewModel {
    fn new(reg: String, val: String) -> RegisterViewModel {
        RegisterViewModel {
            register_name: reg,
            contents: val,
        }
    }
}

impl App {
    pub fn new(simulator: Simulator) -> App {
        App {
            instr_mem: StatefulTable::new(),
            data_mem: StatefulTable::new(),
            registers: StatefulTable::new(),
            pc: String::from("0"),
            simulator,
            val_writer: Box::new(DecValWriter {}),
            run_flag: false,
            finish: false,
        }
    }

    pub fn toggle_run(&mut self) {
        self.run_flag = !self.run_flag;
    }

    pub fn on_tick(&mut self) {
        if self.run_flag && !self.finish {
            match self.simulator.step() {
                RunResult::Success => (), // do nada??
                RunResult::Completed => self.finish = true,
                RunResult::Failure(msg) => panic!("{}", msg), // todo, how to print error message? or should we not
            }
        }

        self.update_pc();
        self.update_data_mem();
        self.update_regs();
        self.update_instr_mem();
    }

    fn update_instr_mem(&mut self) {
        self.instr_mem.items = Vec::new();
        for (addr, val) in self.simulator.get_instr_mem() {
            let instrvm = InstructionViewModel::new(
                addr,
                val.to_hex_string(),
                self.val_writer.write(Some(val.get_op_val())),
                self.val_writer.write(val.get_rs_val()),
                self.val_writer.write(val.get_rt_val()),
                self.val_writer.write(val.get_rd_val()),
                self.val_writer.write(val.get_imm_val()),
                self.val_writer.write(val.get_shamt_val()),
                self.val_writer.write(val.get_funct_val()),
                self.val_writer.write(val.get_jump_address_val()),
                val.to_mnemonic_string(),
            );
            //let instr_vm = InstructionViewModel::new(addr, val);
            //self.instr_mem.items.push(instr_vm);
            self.instr_mem.items.push(instrvm);
        }
        let current_instr_idx = self.simulator.get_current_pc() / 4;
        self.instr_mem.state.select(Some(current_instr_idx));
    }

    fn update_data_mem(&mut self) {
        self.data_mem.items = Vec::new();
        for (addr, val) in self.simulator.get_data_mem() {
            let data_vm = DataViewModel::new(
                self.val_writer.write_unsigned(Some(addr)),
                self.val_writer.write(Some(val as i32)),
            );
            self.data_mem.items.push(data_vm);
        }
    }

    fn update_regs(&mut self) {
        self.registers.items = Vec::new();
        for (reg, val) in self.simulator.get_registers() {
            let reg_string = crate::assembler::instructions::get_register_name(reg).unwrap();
            let reg_vm = RegisterViewModel::new(reg_string, self.val_writer.write(Some(val)));
            self.registers.items.push(reg_vm);
        }
    }

    fn update_pc(&mut self) {
        let pc_val = self.simulator.get_current_pc();
        self.pc = pc_val.to_string();
    }

    pub fn set_val_writer(&mut self, read_char: char) {
        match read_char {
            'd' => self.val_writer = Box::new(DecValWriter {}),
            'h' => self.val_writer = Box::new(HexValWriter {}),
            'b' => self.val_writer = Box::new(BinValWriter {}),
            _ => panic!(),
        }
    }
}

fn draw_instruction_mem_widget<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let header_cells = [
        "Address",
        "Hex Rep",
        "Op",
        "Rs",
        "Rt",
        "Rd",
        "Imm",
        "Shamt",
        "Funct",
        "Target Addr",
        "Mnemonic",
    ]
    .iter()
    .map(|header| Cell::from(*header).style(Style::default().fg(Color::LightRed)));

    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app
        .instr_mem
        .items
        .iter()
        .map(|item| {
            let height = 1;
            let cells = vec![
                Cell::from(item.addr.clone()),
                Cell::from(item.hex_rep.clone()),
                Cell::from(item.op.clone()),
                Cell::from(item.rs.clone()),
                Cell::from(item.rt.clone()),
                Cell::from(item.rd.clone()),
                Cell::from(item.imm.clone()),
                Cell::from(item.shamt.clone()),
                Cell::from(item.func.clone()),
                Cell::from(item.j_target.clone()),
                Cell::from(item.mnemonic.clone()),
            ];
            Row::new(cells).height(height).bottom_margin(1)
        })
        .to_owned();

    let im_widget = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Instruction Memory"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("--> ")
        .widths(&[
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(12),
            Constraint::Percentage(7),
            Constraint::Percentage(7),
            Constraint::Percentage(12),
            Constraint::Percentage(20),
        ]);

    frame.render_stateful_widget(im_widget, area, &mut app.instr_mem.state);
}

fn draw_data_mem_widget<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let header_cells = ["(Byte) Address", "Contents"]
        .iter()
        .map(|header| Cell::from(*header).style(Style::default().fg(Color::LightRed)));

    let header_row = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app
        .data_mem
        .items
        .iter()
        .map(|item| {
            let height = 1;
            let cells = vec![
                Cell::from(item.addr.clone()),
                Cell::from(item.contents.clone()),
            ];
            Row::new(cells).height(height).bottom_margin(1)
        })
        .to_owned();

    let data_widget = Table::new(rows)
        .header(header_row)
        .block(Block::default().borders(Borders::ALL).title("Data Memory"))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("--> ")
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    frame.render_stateful_widget(data_widget, area, &mut app.data_mem.state);
}

fn draw_register_widget<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let header_cells = ["Register Name", "Value"]
        .iter()
        .map(|header| Cell::from(*header).style(Style::default().fg(Color::LightRed)));

    let header_row = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app
        .registers
        .items
        .iter()
        .map(|item| {
            let height = 1;
            let cells = vec![
                Cell::from(item.register_name.clone()),
                Cell::from(item.contents.clone()),
            ];
            Row::new(cells).height(height).bottom_margin(1)
        })
        .to_owned();

    let reg_widget = Table::new(rows)
        .header(header_row)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Register File"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("--> ")
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

    frame.render_stateful_widget(reg_widget, area, &mut app.registers.state);
}

fn draw_pc_widget<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let content = format!("PC: {}", app.pc);
    let pc_widget = Block::default()
        .title(Span::styled(
            content,
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    frame.render_widget(pc_widget, area)
}

fn draw_bottom_half<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);
    draw_registers_and_pc(frame, app, chunks[0]);
    draw_data_mem_widget(frame, app, chunks[1]);
}

fn draw_registers_and_pc<'a, B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(05)].as_ref())
        .split(area);
    draw_register_widget(frame, app, chunks[0]);
    draw_pc_widget(frame, app, chunks[1]);
}

// Gets the widgets to render, then renders them
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    draw_instruction_mem_widget(f, app, main_chunks[0]);
    draw_bottom_half(f, app, main_chunks[1]);
}
