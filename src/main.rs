extern crate console;
extern crate termion;

use console::{Key, Term};
use termion::screen::AlternateScreen;
use termion::terminal_size;

use std::collections::HashMap;
use std::io::stdout;
use std::io::Write;

fn main() {
    let mut screen = AlternateScreen::from(stdout());
    write!(screen, "Enter search: ").unwrap();
    screen.flush().unwrap();
    run_loop(screen);
}

fn run_loop(mut screen: AlternateScreen<std::io::Stdout>) {
    let prompt = "Enter search:";
    let mut search = String::new();
    let term = Term::stdout();
    loop {
        clear_screen();
        let input = Term::read_key(&term).expect("Invalid char");
        match input {
            Key::Char(key) => {
                if key == '\u{15}' {
                    search.clear();
                } else {
                    search.push(key);
                }
            }
            Key::Backspace => {
                search.pop();
                ()
            }
            Key::UnknownEscSeq(seq) => {
                if seq == ['\u{7f}'] {
                    // Delete word
                    let mut words: Vec<&str> = search.split(" ").collect();
                    words.pop();
                    search = words.join(" ");
                }
            }
            Key::Enter | Key::Escape => break,
            _ => (),
        }
        writeln!(screen, "{} {}", prompt, search).unwrap();
        print_results(&search);
        let index = search.len() + prompt.len() + 2;
        write!(
            screen,
            "{}",
            termion::cursor::Goto(index.try_into().unwrap(), 1)
        )
        .unwrap();
        flush();
    }
}

fn flush() {
    stdout().flush().expect("Failed to flush");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_results(search: &str) {
    let table = get_table();
    let (_, height) = terminal_size().unwrap();
    let term_height: usize = (height - 5).into();

    let mut results: Vec<&str> = table
        .keys()
        .map(|m| *m)
        .filter(|k| k.contains(search))
        .collect();

    results.sort();

    if !results.is_empty() {
        let end = std::cmp::min(results.len(), term_height);
        results[0..end]
            .iter()
            .for_each(|s| println!("{:10} - {}", table[s], s))
    }
}

fn get_table() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("git add", "ga"),
        ("git add --all", "gaa"),
        ("git add --patch", "gapa"),
        ("git add --update", "gau"),
        ("git add --verbose", "gav"),
        ("git apply", "gap"),
        ("git apply --3way", "gapt"),
        ("git branch", "gb"),
        ("git branch -a", "gba"),
        ("git branch -d", "gbd"),
        ("git branch -D", "gbD"),
        ("git blame -b -w", "gbl"),
        ("git branch --no-merged", "gbnm"),
        ("git branch --remote", "gbr"),
        ("git bisect", "gbs"),
        ("git bisect bad", "gbsb"),
        ("git bisect good", "gbsg"),
        ("git bisect reset", "gbsr"),
        ("git bisect start", "gbss"),
        ("git commit -v", "gc"),
        ("git commit -v --amend", "gc"),
        ("git commit -v --no-edit --amend", "gcn"),
        ("git commit -v -a", "gca"),
        ("git commit -v -a --amend", "gca"),
        ("git commit -v -a --no-edit --amend", "gcan"),
        ("git commit -v -a -s --no-edit --amend", "gcans"),
        ("git commit -a -m", "gcam"),
        ("git commit -s -m", "gcsm"),
        ("git checkout -b", "gcb"),
        ("git config --list", "gcf"),
        ("git clone --recurse-submodules", "gcl"),
        ("git clean -id", "gclean"),
        ("git reset --hard && git clean -dffx", "gpristine"),
        ("git checkout $(git_main_branch)", "gcm"),
        ("git checkout develop", "gcd"),
        ("git commit -m", "gcmsg"),
        ("git checkout", "gco"),
        ("git shortlog -sn", "gcount"),
        ("git cherry-pick", "gcp"),
        ("git cherry-pick --abort", "gcpa"),
        ("git cherry-pick --continue", "gcpc"),
        ("git commit -S", "gcs"),
        ("git diff", "gd"),
        ("git diff --cached", "gdca"),
        ("git diff --cached --word-diff", "gdcw"),
        (
            "git describe --tags $(git rev-list --tags --max-count=1)",
            "gdct",
        ),
        ("git diff --staged", "gds"),
        ("git diff-tree --no-commit-id --name-only -r", "gdt"),
        // ("git diff $@ ",(exclude)package-lock.json" ",(exclude)*.lock"", "gdnolock"),
        ("git diff -w $@ | view -", "gdv"),
        ("git diff --word-diff", "gdw"),
        ("git fetch", "gf"),
        ("git fetch --all --prune", "gfa"),
        ("git ls-files | grep", "gfg"),
        ("git fetch origin", "gfo"),
        ("git gui citool", "gg"),
        ("git gui citool --amend", "gga"),
        ("git push --force origin $(current_branch)", "ggf"),
        (
            "git push --force-with-lease origin $(current_branch)",
            "ggfl",
        ),
        ("git pull origin $(current_branch)", "ggl"),
        ("git push origin $(current_branch)", "ggp"),
        ("ggl && ggp", "ggpnp"),
        // ("git pull origin "$(git_current_branch)"", "ggpull"),
        // ("git push origin "$(git_current_branch)"", "ggpush"),
        (
            "git branch --set-upstream-to=origin/$(git_current_branch)",
            "ggsup",
        ),
        ("git pull --rebase origin $(current_branch)", "ggu"),
        (
            "git push --set-upstream origin $(git_current_branch)",
            "gpsup",
        ),
        ("git help", "ghh"),
        ("git update-index --assume-unchanged", "gignore"),
        // ("git ls-files -v | grep "^((,lower,))"", "gignored"),
        ("gitk --all --branches", "gk"),
        ("gitk --all $(git log -g --pretty=%h)", "gke"),
        ("git pull", "gl"),
        ("git log --stat", "glg"),
        ("git log --stat -p", "glgp"),
        ("git log --graph", "glgg"),
        ("git log --graph --decorate --all", "glgga"),
        ("git log --graph --max-count=10", "glgm"),
        ("git log --oneline --decorate", "glo"),
        //   "git log --graph --pretty="%Cred%h%Creset -%C(auto)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset"", "glol"),
        //   "git log --graph --pretty="%Cred%h%Creset -%C(auto)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset" --stat", "glols"),
        //   "git log --graph --pretty="%Cred%h%Creset -%C(auto)%d%Creset %s %Cgreen(%ad) %C(bold blue)<%an>%Creset"", "glod"),
        //   "git log --graph --pretty="%Cred%h%Creset -%C(auto)%d%Creset %s %Cgreen(%ad) %C(bold blue)<%an>%Creset" --date=short", "glods"),
        //   "git log --graph --pretty="%Cred%h%Creset -%C(auto)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset" --all", "glola"),
        ("git log --oneline --decorate --graph", "glog"),
        ("git log --oneline --decorate --graph --all", "gloga"),
        ("git log --pretty=<format>", "glp"),
        ("git merge", "gm"),
        ("git merge origin/$(git_main_branch)", "gmom"),
        ("git mergetool --no-prompt", "gmt"),
        ("git mergetool --no-prompt --tool=vimdiff", "gmtvim"),
        ("git merge upstream/$(git_main_branch)", "gmum"),
        ("git merge --abort", "gma"),
        ("git push", "gp"),
        ("git push --dry-run", "gpd"),
        ("git push --force-with-lease", "gpf"),
        ("git push --force", "gpf"),
        ("git push origin --all && git push origin --tags", "gpoat"),
        ("git push upstream", "gpu"),
        ("git push -v", "gpv"),
        ("git remote", "gr"),
        ("git remote add", "gra"),
        ("git rebase", "grb"),
        ("git rebase --abort", "grba"),
        ("git rebase --continue", "grbc"),
        ("git rebase develop", "grbd"),
        ("git rebase -i", "grbi"),
        ("git rebase $(git_main_branch)", "grbm"),
        ("git rebase --skip", "grbs"),
        ("git revert", "grev"),
        ("git reset", "grh"),
        ("git reset --hard", "grhh"),
        ("git reset origin/$(git_current_branch) --hard", "groh"),
        ("git rm", "grm"),
        ("git rm --cached", "grmc"),
        ("git remote rename", "grmv"),
        ("git remote remove", "grrm"),
        ("git restore", "grs"),
        ("git remote set-url", "grset"),
        ("git restore --source", "grss"),
        // ("cd "$(git rev-parse --show-toplevel || echo .)"", "grt"),
        ("git reset --", "gru"),
        ("git remote update", "grup"),
        ("git remote -v", "grv"),
        ("git status -sb", "gsb"),
        ("git svn dcommit", "gsd"),
        ("git show", "gsh"),
        ("git submodule init", "gsi"),
        ("git show --pretty=short --show-signature", "gsps"),
        ("git svn rebase", "gsr"),
        ("git status -s", "gss"),
        ("git status", "gst"),
        ("git stash push", "gsta"),
        ("git stash save", "gsta"),
        ("git stash apply", "gstaa"),
        ("git stash clear", "gstc"),
        ("git stash drop", "gstd"),
        ("git stash list", "gstl"),
        ("git stash pop", "gstp"),
        ("git stash show --text", "gsts"),
        ("git stash --include-untracked", "gstu"),
        ("git stash --all", "gstall"),
        ("git submodule update", "gsu"),
        ("git switch", "gsw"),
        ("git switch -c", "gswc"),
        ("git tag -s", "gts"),
        ("git tag | sort -V", "gtv"),
        (
            "gtl(){ git tag --sort=-v,refname -n -l ${1}* }; noglob gtl",
            "gtl",
        ),
        ("git update-index --no-assume-unchanged", "gunignore"),
        // ("git log -n 1 | grep -q -c "--wip--" && git reset HEAD~1", "gunwip"),
        ("git pull --rebase", "gup"),
        ("git pull --rebase -v", "gupv"),
        ("git pull --rebase --autostash", "gupa"),
        ("git pull --rebase --autostash -v", "gupav"),
        ("git pull upstream $(git_main_branch)", "glum"),
        ("git whatchanged -p --abbrev-commit --pretty=medium", "gwch"),
        // ("git add -A; git rm $(git ls-files --deleted) 2> /dev/null; git commit --no-verify --no-gpg-sign -m "--wip-- (skip ci)"", "gwip"),
        ("git am", "gam"),
        ("git am --continue", "gamc"),
        ("git am --skip", "gams"),
        ("git am --abort", "gama"),
        ("git am --show-current-patch", "gamscp"),
    ])
}
