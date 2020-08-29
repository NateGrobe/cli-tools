use std::io;
use std::fs;
use std::path::PathBuf;

pub struct Input {
    pub root_dir: PathBuf,
}

impl Input {
    pub fn new(mut args: std::env::Args) -> Result<Input, &'static str> {
        if args.len() > 2 {
            return Err("Too many args");
        } 

        args.next();

        let root_dir = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => PathBuf::from("."),
        };

        Ok(Input { root_dir })
    }
}

#[derive(Clone, Debug)]
struct Todo {
    multi_line: bool,
    content: String,
    line_num: String,
    spaces: String,
}

impl Todo {
    fn new(todo: &str, line_num: usize, is_multi: bool) -> Todo {
        let raw_line: Vec<char> = todo.trim().chars().collect();
        let mut multi_line = false;
        if (raw_line[0] == '/' && raw_line[1] == '*') || is_multi {
            multi_line = true;
        }

        let content: String = raw_line.iter().collect();
        
        Todo {
            multi_line,
            content: remove_comment(content.as_str()),
            line_num: line_num.to_string(),
            spaces: "    "[line_num.to_string().len()..].to_string()
        }
    }

    fn add_line(&mut self, line: &str) {
        let multi_line = !line.contains("*/");
        self.content.push_str("\n");
        let trimmed_line = line.trim();
        let line_vec: Vec<char> = trimmed_line.chars().collect();

        // removes stars from beginning of the line
        if line_vec[0] == '*' && !trimmed_line.contains("*/") {
            let cleaned_line: String = line_vec[1..].iter().collect();
            let final_line = format!("      {}", cleaned_line.trim());
            self.content.push_str(&final_line);

        // removes stars from beginning and */ from end of line but super ugly
        } else if line_vec[0] == '*' && trimmed_line.contains("*/") && line_vec.len() > 2 {
            let mut cleaned_line: String = String::new();
            for (i, c) in line_vec.iter().enumerate() {
                if c == &'*' {
                    if i != 0 && i != line_vec.len() -2 {
                        cleaned_line.push(c.clone());
                    }
                } else if c == &'/' {
                    if i != 0 && i != line_vec.len() -1 {
                        cleaned_line.push(c.clone());

                    }      
                } else {
                    cleaned_line.push(c.clone());
                }
            }

            let final_line = format!("      {}", cleaned_line.trim());
            self.content.push_str(&final_line);
        }
        self.multi_line = multi_line;
    }

    fn format_todos(&self) -> String {
        format!("{}{}: {}", self.spaces, self.line_num, self.content)
    }
}

fn walk_dirs(start: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(start)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            dirs.push(path);
        } else {
            let mut nested_dirs = walk_dirs(path).unwrap();
            dirs.append(&mut nested_dirs);
        }
    }

    Ok(dirs)
}

fn find_todos(contents: &str) -> Vec<Todo> {
    let mut todos: Vec<Todo> = Vec::new();
    let mut line_num: usize = 1;
    let mut prev_was_td = false;

    let lines_vec: Vec<&str> = contents.lines().collect();
    let mut lines = lines_vec.iter();

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break
        };

        if line.contains("/*") && !line.contains("TODO") {
            prev_was_td = true;
        }

        if line.contains("TODO") {
            let mut todo = Todo::new(line, line_num, prev_was_td);

            loop {
                if !todo.multi_line {
                    todos.push(todo);
                    break
                }

                if lines_vec[line_num].contains("TODO") {
                    todo.multi_line = false;
                    todos.push(todo);
                    prev_was_td = true;
                    break
                } else {
                    line_num += 1;
                    let next_line = match lines.next(){
                        Some(line) => line,
                        None => break
                    };
                    todo.add_line(next_line);
                    if !todo.multi_line {
                        todos.push(todo);
                        break
                    }
                }
            }
        }

        line_num += 1;

    }

    todos
}

fn remove_comment(todo: &str) -> String {
    let to_pos = todo.chars().position(|c| c == 'T').unwrap();
    let chars: Vec<char> = todo.chars().collect();
    chars[to_pos..].iter().collect()
}


fn display_todos(dir: &PathBuf, todos: Vec<Todo>) {

    if todos.len() > 0 {
        println!("{}:", dir.to_str().unwrap());
        println!("");
        for td in todos.iter() {
            println!("{}\n", td.format_todos());
        }
    }
}

pub fn run(input: Input) {
    let entries = walk_dirs(input.root_dir).unwrap();
    for entry in entries.iter() {
        match fs::read_to_string(entry){
            Ok(dir) =>{
                let todos = find_todos(dir.as_str());
                display_todos(entry, todos);
            },
            Err(_) => continue
        };

    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_rust_comment() {
        let todo = "// TODO: testing that the double slash is removed";
        let result = remove_comment(&todo);
        let expected = "TODO: testing that the double slash is removed";
        assert_eq!(expected, result)
    }

    #[test]
    fn works_on_nested_todos() {
        let nested_todos = "\
/* TODO: testin nested
 * TODO: nested
 * part of nested
 */";
        let result = find_todos(nested_todos);
        let td1 = Todo {
                multi_line: false,
                content: String::from("TODO: testin nested"),
                line_num: String::from("1"),
                spaces: String::from("   "),
            };

        let td2 = Todo  {
            multi_line: false,
            content: String::from("TODO: nested\n      part of nested\n"),
            line_num: String::from("2"),
            spaces: String::from("   ")
        };

        assert_eq!(td1.multi_line, result[0].multi_line);
        assert_eq!(td1.content, result[0].content);
        assert_eq!(td1.line_num, result[0].line_num);
        assert_eq!(td1.spaces, result[0].spaces);

        assert_eq!(td2.multi_line, result[1].multi_line);
        assert_eq!(td2.content, result[1].content);
        assert_eq!(td2.line_num, result[1].line_num);
        assert_eq!(td2.spaces, result[1].spaces);
    }
}
