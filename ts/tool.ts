import { spawnSync } from 'child_process'
import { isAdmin } from '@easy-install/is-admin'
import { delimiter } from 'path'
import { whichShell } from 'which-shell'

export function isMsys() {
  return !!process.env['MSYSTEM']
}

// C:\a\b -> /c/a/b
export function toMsysPath(path: string): string {
  path = path.replaceAll('\\', '/')
  const prefix = path.slice(0, 3)
  const tail = path.slice(3)
  if (prefix.length === 3 && prefix.endsWith(':/')) {
    return `/${prefix[0].toLowerCase()}/${tail}`
  }
  return path
}

// c:/a/b -> C:\a\b
export function toWinPath(path: string): string {
  path = path.replaceAll('/', '\\')
  const prefix = path.slice(0, 3)
  const tail = path.slice(3)
  if (prefix.length === 3 && prefix.endsWith(':\\')) {
    return `${prefix.toUpperCase()}${tail}`
  }
  return path
}

export function addPathWindows(path: string): string | undefined {
  const mode = isAdmin() ? 'Machine' : 'User'
  const winPath = toWinPath(path)
  const shell =
    `$currentPath = [Environment]::GetEnvironmentVariable("Path", "${mode}");$newPath = "$currentPath;${winPath}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "${mode}")`
  const output = spawnSync('powershell', ['-c', shell]).status
  if (output === 0) {
    const sh = addPathUnix(path)
    if (sh) {
      return sh
    }
    return 'powershell'
  }
}

export function addPathUnix(path: string): string | undefined {
  const shell = whichShell()?.shell
  if (!shell) return

  if (isMsys() || process.platform === 'win32') {
    path = toMsysPath(path)
  }

  let configFile = ''
  let cmd = ''
  const home = '~'
  switch (shell) {
    case 'fish':
      configFile = `${home}/.config/fish/config.fish`
      cmd = `echo 'set -gx PATH "${path}" $PATH' >> ${configFile}`
      break

    case 'zsh':
      configFile = `${home}/.zshrc`
      cmd = `echo 'export PATH="${path}:$PATH"' >> ${configFile}`
      break

    case 'bash':
      configFile = `${home}/.bashrc`
      cmd = `echo 'export PATH="${path}:$PATH"' >> ${configFile}`
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
  return process.env['PATH']?.split(delimiter) || []
}

export function hasPath(path: string) {
  if (process.platform === 'win32') {
    path = toWinPath(path).replaceAll('\\', '/')
  }
  return !!getPath().find((i) => i.toLowerCase().replaceAll("\\", '/') === path.toLowerCase())
}

export const addPath = (path: string) => {
  const fn = process.platform === 'win32' ? addPathWindows : addPathUnix
  process.env['PATH'] = process.env['PATH'] + delimiter + path
  return fn(path)
}
