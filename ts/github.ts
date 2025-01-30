import { appendFileSync, existsSync } from "fs"
import { EOL } from "os"
import { delimiter } from "path"

export function isGithub() {
  return process.env["GITHUB_ACTIONS"] === "true"
}

export function issueFileCommand(cmd: string, message: string) {
  const varName = `GITHUB_${cmd}`
  const filePath = process.env[varName]
  if (filePath && existsSync(filePath)) {
    appendFileSync(filePath, `${message}${EOL}`, {
      encoding: "utf8",
    })
  }
}

export function addGithubPath(inputPath: string) {
  const githubPath = process.env["GITHUB_PATH"]
  if (githubPath?.length) {
    issueFileCommand("PATH", inputPath)
  }
  process.env["PATH"] = `${inputPath}${delimiter}${process.env["PATH"]}`
}
