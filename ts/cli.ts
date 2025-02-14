import { whichShell } from 'which-shell'
import { addGithubPath, isGithub } from './github'
import { addPath, getPath, hasPath } from './tool'

const args = process.argv.slice(2)
if (args.length === 0) {
  console.log('Usage: crud-path get/has/add <PATH>')
}

const cmd = args[0]

switch (cmd) {
  case 'shell': {
    console.log(whichShell())
    break
  }

  case 'add': {
    if (args[1]) {
      console.log(addPath(args[1]))
    }
    break
  }

  case 'has': {
    if (args[1]) {
      console.log(hasPath(args[1]))
    }
    break
  }

  case 'get': {
    console.log(getPath().join('\n'))
    break
  }

  case 'is_github': {
    console.log(isGithub())
    break
  }

  case 'add_github_path': {
    if (args[1] && isGithub()) {
      addGithubPath(args[1])
    }
    break
  }

  default:
    console.log(
      'Usage: crud-path get/has/add/is_github/add_github_path/shell <PATH>',
    )
    break
}
