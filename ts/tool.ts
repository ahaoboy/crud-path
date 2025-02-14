import { spawnSync } from 'child_process'
import { isAdmin } from './is-admin'
import { delimiter } from 'path'

export function addPathWindows(path: string): string | undefined {
  const mode = isAdmin() ? 'Machine' : 'User'
  const shell =
    `$currentPath = [Environment]::GetEnvironmentVariable("Path", "${mode}");$newPath = "$currentPath;${path}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "${mode}")`
  const output = spawnSync('powershell', ['-c', shell]).status
  if (output === 0) {
    return 'powershell'
  }
}

function whichShell(): string | undefined {
  const shell = process.env.SHELL
  if (!shell) return
  if (shell.includes('fish')) return 'fish'
  if (shell.includes('zsh')) return 'zsh'
  if (shell.includes('bash')) return 'bash'
}

export function addPathUnix(pathToAdd: string): string | undefined {
  const shell = whichShell()
  if (!shell) return

  let configFile = ''
  let cmd = ''

  switch (shell) {
    case 'fish':
      configFile = '~/.config/fish/config.fish'
      cmd = `echo 'set -gx PATH "${pathToAdd}" $PATH' >> ${configFile}`
      break

    case 'zsh':
      configFile = '~/.zshrc'
      cmd = `echo 'export PATH="${pathToAdd}:$PATH"' >> ${configFile}`
      break

    case 'bash':
      configFile = '~/.bashrc'
      cmd = `echo 'export PATH="${pathToAdd}:$PATH"' >> ${configFile}`
      break
    default:
      return
  }

  return spawnSync(shell, ['-c', cmd]).status === 0 ? shell : undefined
}

export function getPath() {
  return process.env['PATH']?.split(delimiter).map(i => i.replaceAll("\\", "/")) || []
}

export function hasPath(path: string) {
  return getPath().includes(path)
}

export const addPath = (path: string) => {
  const fn = process.platform === 'win32' ? addPathWindows : addPathUnix
  process.env['PATH'] = process.env['PATH'] + delimiter + path
  return fn(path)
}
