# Create New Project using cargo

```
cargo new [project_name]
```
Here `project_name` is arg_parse

# Add clap module to the project
clap is a the most popular argument libary for rust to handle the CLI
```
cargo add clap --feature derive
```
You just have to define the structure of the arguments and clap will implement the logic of be half of you
