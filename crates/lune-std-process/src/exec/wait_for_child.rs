use std::{io::stdout, process::ExitStatus};

use mlua::prelude::*;

use async_process::Child;
use blocking::Unblock;
use futures_lite::{io, prelude::*};

use super::tee_writer::AsyncTeeWriter;
use crate::options::ProcessSpawnOptionsStdioKind;

#[derive(Debug, Clone)]
pub(super) struct WaitForChildResult {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

async fn read_with_stdio_kind<R>(
    read_from: Option<R>,
    kind: ProcessSpawnOptionsStdioKind,
) -> LuaResult<Vec<u8>>
where
    R: AsyncRead + Unpin,
{
    Ok(match kind {
        ProcessSpawnOptionsStdioKind::None | ProcessSpawnOptionsStdioKind::Forward => Vec::new(),
        ProcessSpawnOptionsStdioKind::Default => {
            let mut read_from =
                read_from.expect("read_from must be Some when stdio kind is Default");

            let mut buffer = Vec::new();

            read_from.read_to_end(&mut buffer).await.into_lua_err()?;

            buffer
        }
        ProcessSpawnOptionsStdioKind::Inherit => {
            let mut read_from =
                read_from.expect("read_from must be Some when stdio kind is Inherit");

            let mut stdout = Unblock::new(stdout());
            let mut tee = AsyncTeeWriter::new(&mut stdout);

            io::copy(&mut read_from, &mut tee).await.into_lua_err()?;

            tee.into_vec()
        }
    })
}

pub(super) async fn wait_for_child(
    mut child: Child,
    stdout_kind: ProcessSpawnOptionsStdioKind,
    stderr_kind: ProcessSpawnOptionsStdioKind,
) -> LuaResult<WaitForChildResult> {
    let stdout_opt = child.stdout.take();
    let stderr_opt = child.stderr.take();

    let stdout_task = read_with_stdio_kind(stdout_opt, stdout_kind);
    let stderr_task = read_with_stdio_kind(stderr_opt, stderr_kind);

    let status = child.status().await.into_lua_err()?;

    let stdout_buffer = stdout_task.await.into_lua_err()?;
    let stderr_buffer = stderr_task.await.into_lua_err()?;

    Ok(WaitForChildResult {
        status,
        stdout: stdout_buffer,
        stderr: stderr_buffer,
    })
}
