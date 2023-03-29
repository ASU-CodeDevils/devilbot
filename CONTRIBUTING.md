# Contributing Guidelines and Git Flow
This document plans to outline the following for *all* contributers of the Rust DevilBot project. Please follow these guidelines to the best of your knowledge and understanding, and feel free to message `@Json Michelson` or `@sjcampb2` in the CodeDevils Slack workspace if you have any questions or need help!

### Table of Contents
1. [Chain of Command](#chain-of-command)
1. [Git Flow Overview](#git-flow)
1. [Workflow Contribution Requirements](#contribution-requirements)
1. [Workflow Examples](#workflow-examples)
1. [Quality of Code Requirements](#quality-of-code)
1. [Git Cheatsheet](#git-cheatsheet)
1. [Rust Commands](#rust-commands)
1. [Other Resources](#other-resources)

---

# Chain of Command
The Rust DevilBot project managers are Jason Michelson ([@jtmichelson](https://github.com/jtmichelson)) and JRhett Harrison Campbell ([@reharri7](https://github.com/reharri7)). They control the day-to-day opperation of this project, with oversight and policy ditacted by the CodeDevil Officers.

> To see your CodeDevil officers, visit the `#about` channel of the CodeDevils Slack workspace.

# Git Flow
Overall, the Git flow is mostly laidback. You, as a contributer, have lots of wiggle room to make your own inputs in your commits to this repo. Forking is not neccesary - use the origin repo's branches for your own code bases. You have full control over your own development branches and Git flow. That said, there are some general guidelines you need to follow.

## Overview
#### Personal Branch
This is your own branch. You can call it whatever you want locally and push it into the remote repository. Your working commits will be on this branch and when you are ready to create a pull request (PR) for a specific GitHub issue you will use your personal branch for that PR.

#### Main Branch
The `main` branch is the *production code* that DevilBot is currently opperating on. Whatever is in `main` is what will be on Slack. PRs from personal branches to `main` will **require an approved code review from a project manager**.

## Contribution Requirements
Observe the following in your git flow:
- Make an Issue prior to working on your code. We don't want you to work on something that is actually banned or already implemented!
- Your personal development branches must start with your name and be consistent. E.g., `bobby-dev`, `bobby-new-feature-here`
- Though there is no limit to the number of reviewers you request, your pull requests to `main` **must** at least include **a project manager**.
- Comment in your code following the functionality and process within it.
- Make sure that you have tested your code using your personal AWS account and personal Slack development bot. Be sure to include in your PR a link to a screen capture of you testing your feature. **NO CODE WILL BE APPROVED WITHOUT BEING TESTED FIRST**.

### Workflow Examples
#### Marlee's Quick Patch (Simple)
> My name is Marlee, and I noticed a mispelling in the README.
> 
> I would create a GitHub issue for this, then create a branch called `marlee-hotfix`, make the correction, then submit a pull request to `main` making sure to request review from [@jtmichelson](https://github.com/jtmichelson) or [@reharri7](https://github.com/reharri7).

Good job Marlee!

#### Clyde & Darryl's Calendar (Collab)
> My name is Clyde, and I'm working with Darryl on a sweet new command that allows something to do with calendars.
> 
> I would create a GitHub issue for the new feature, then checkout a new branch entitled `calendar-dev`, acting as the default branch for the calendar between Darryl and I, making sure to keep it updated with `main`. Darryl and I would then have our own branches whatever we want to call them following the guideline, say  `darryl-calendar` and `clyde-dev`. We push and pull from `calendar-dev` for development.
> 
> When our cool new command is done, *making sure to pull `calendar-dev` from `main` so that there are not merge conflicts*, I would then make a pull request on `main`. Billy-Bob is good with calendars, so I would like his review too. I would request review from `@BillyBobUSA` along with a project manager such as [@jtmichelson](https://github.com/jtmichelson) or [@reharri7](https://github.com/reharri7).

Nicely done!

---

# Quality of Code
> TODO - TBD by Json and/or Stu
>
> Note: We currently have a Rust unit test framework in place. So we need to increase the coverage to ~100% then we can add code coverage velocity checks for all new changes to prevent uncovered code from being published. See [issue 18](https://github.com/ASU-CodeDevils/devil_bot_rust/issues/18).

----

# Git Cheatsheet
#### Checkout Existing Branch
```
git checkout existing-branch
```

#### Checkout a New Branch
```
git checkout -B your-new-branch
```

#### Push to Repo
```
git push origin destination-branch
```
Pro tip: add `-u` to skip the naming of the branch in future pushes.

#### Update Current Branch from Repo
```
git fetch && git pull
```

#### Update Current Branch from Target Branch
```
git fetch ; git merge target-branch
```
Use Case: Your development branch (current branch) is behind `dev` and you want to update your branch with the code from `dev` (or some other target branch). 

---

# Rust Commands
```
cargo fmt --all --check --manifest-path resources/Cargo.toml
```
Use Case: We want to make sure you code is formatted correctly to Rustacean standards. This command will run during your pull request, but you can run it each time before you commit code as well. We encourage it!
---

## Other Resources
* Installing Git: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
* Git Basics: https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository
* GitHub - Creating a Pull Request: https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request


