import { spawnSync } from 'child_process'
import { isAdmin } from './is-admin'
import { delimiter } from 'path'
import { homedir } from 'os'
import { whichShell } from 'which-shell'

export function addPathWindows(path: string): string | undefined {
  const mode = isAdmin() ? 'Machine' : 'User'
  const shell =
    `$currentPath = [Environment]::GetEnvironmentVariable("Path", "${mode}");$newPath = "$currentPath;${path}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "${mode}")`
  const output = spawnSync('powershell', ['-c', shell]).status
  if (output === 0) {
    return 'powershell'
  }
}
export function addPathUnix(pathToAdd: string): string | undefined {
  const shell = whichShell()?.shell
  if (!shell) return

  let configFile = ''
  let cmd = ''
  const home = homedir()
  switch (shell) {
    case 'fish':
      configFile = `${home}/.config/fish/config.fish`
      cmd = `echo 'set -gx PATH "${pathToAdd}" $PATH' >> ${configFile}`
      break

    case 'zsh':
      configFile = `${home}/.zshrc`
      cmd = `echo 'export PATH="${pathToAdd}:$PATH"' >> ${configFile}`
      break

    case 'bash':
      configFile = `${home}/.bashrc`
      cmd = `echo 'export PATH="${pathToAdd}:$PATH"' >> ${configFile}`
      break
    default:
      return
  }
  const addLine = `echo "" >> ${configFile}`
  try {
    if (
      [
        spawnSync(shell, ['-c', addLine]),
        spawnSync(shell, ['-c', cmd]),
        spawnSync(shell, ['-c', addLine]),
      ].every((i) => i.status === 0)
    ) {
      return shell
    }
  } catch {
  }
}

export function getPath() {
  return process.env['PATH']?.split(delimiter).map((i) =>
    i.replaceAll('\\', '/')
  ) || []
}

export function hasPath(path: string) {
  return getPath().includes(path)
}

export const addPath = (path: string) => {
  const fn = process.platform === 'win32' ? addPathWindows : addPathUnix
  process.env['PATH'] = process.env['PATH'] + delimiter + path
  return fn(path)
}
