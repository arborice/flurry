use crate::{
    config::{types::*, write::overwrite_cmds},
    prelude::*,
    tui::prelude::*,
};
use rkyv::{de::deserializers::AllocDeserializer, Deserialize};

pub fn dispatch_interactive(gen_cmds: &ArchivedGeneratedCommands) -> Result<()> {
    if gen_cmds.commands.is_none() {
        return Err(anyhow!("No commands yet!"));
    }

    let mut gen_cmds = gen_cmds.deserialize(&mut AllocDeserializer)?;
    let mut exit_status = None;
    let mut success_msg = String::new();

    if let Some(ref mut commands) = gen_cmds.commands {
        let cmds_ref = RefCell::from(commands);

        let mut app = StatefulCmdsTable::with_items(&cmds_ref)
            .with_header_style(Style::default().fg(Color::Blue))
            .with_rm_style(Style::default().fg(Color::Red))
            .with_selection_style(Style::default().fg(Color::Cyan));

        let status = app.render(None)?;
        if status.success {
            if !status.rm_selection.is_empty() {
                let ref selection = status.rm_selection;
                cmds_ref
                    .borrow_mut()
                    .retain(|key, _| !selection.iter().any(|k| k == key));
                success_msg += &format!("Removed {}", selection.join(", "));
            }

            exit_status.replace(status);
        }
    }

    if let Some(status) = exit_status {
        if let Some(key) = status.go_request {
            if let Some(cmd) = gen_cmds.get(&key) {
                match &cmd.dfl_args {
                    Some(args) => run_cmd!(@ &cmd.bin => args),
                    None => run_cmd!(@ &cmd.bin =>),
                }?;
            }
        }

        overwrite_cmds(gen_cmds)?;
        println!("{}", success_msg);
    }
    Ok(())
}

#[test]
fn poll_state() {
    use std::{
        // fs::OpenOptions,
        // io::Write,
        fs::write,
        sync::mpsc::{channel, Receiver, Sender},
        thread,
    };

    let test_output_path = "test_output";
    let write_to_test_output = move |val: &str| {
        // OpenOptions::new()
        // .append(true)
        // .open(test_output_path)
        // .seppuku(None)
        // .write(val.as_bytes())
        // .seppuku(None);
        write(test_output_path, val).seppuku(None);
    };

    let (tx, rx): (Sender<usize>, Receiver<usize>) = channel();
    thread::spawn(move || loop {
        if let Ok(state) = rx.recv() {
            let state = match state {
                StatefulCmdsTable::DFL_STATE => "DFL_STATE\n",
                StatefulCmdsTable::ADD_STATE => "ADD_STATE\n",
                StatefulCmdsTable::EDIT_STATE => "EDIT_STATE\n",
                StatefulCmdsTable::RM_STATE => "RM_STATE\n",
                StatefulCmdsTable::EXIT_STATE => "EXIT_STATE\n",
                _ => "UNDEFINED\n",
            };

            write_to_test_output(state);
        }
    });

    let cmds_db = crate::config::get::CmdsDb::from_cfg().unwrap();
    let mut cmds_deser = cmds_db
        .archive()
        .deserialize(&mut AllocDeserializer)
        .unwrap();

    if let Some(ref mut cmds) = cmds_deser.commands {
        let cmds_ref = RefCell::from(cmds);
        let mut app = StatefulCmdsTable::with_items(&cmds_ref)
            .with_header_style(Style::default().fg(Color::Blue))
            .with_rm_style(Style::default().fg(Color::Red))
            .with_selection_style(Style::default().fg(Color::Cyan));
        let exit_status = app.render(Some(&tx)).unwrap();

        assert_eq!(exit_status.success, true);
        write_to_test_output(&format!("{:#?}", exit_status));
    }
}
