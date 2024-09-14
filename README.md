# minigrep
This is a very useful program that lets you search for lines that contain a specific string
within a file.

# Example usage
I do not know how to compile to an executable right now, so that's an issue.

Search for lines containing "frog":
```
cargo run -- frog poem.txt
# Outputs "How public, like a frog"
```

Search for lines containing "you know":
```
cargo run -- "you know" poem.txt
# Outputs "They'd banish us, you know."
```

Search for lines containing "to", but case insensitive:
```
IGNORE_CASE=1 cargo run -- "to" poem.txt
# Outputs 
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
