import { execSync } from 'child_process'

export function isAdminWindows() {
  const shell =
    `powershell -c "[bool]([System.Security.Principal.WindowsPrincipal][System.Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([System.Security.Principal.WindowsBuiltInRole]::Administrator)"`
  const output = execSync(shell).toString().trim()
  return output === 'True'
}

export function isAdminUnix() {
  return process.getuid?.() === 0
}

export function isAdmin(): boolean {
  return process.platform === 'win32' ? isAdminWindows() : isAdminUnix()
}
