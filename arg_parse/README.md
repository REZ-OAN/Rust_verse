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

Here we implemented an arg_parser for the below structure

```
command
   | user
   | todo
subcommand
    
    | for user 
        | - create
        | - update
        | - delete
        | - show
    | for todo    
        | - create
        | - update
        | - delete
        | - show
required values
    | for user
        | - create 
            | - name
            | - email
            | - password
        | - update
            | - email
            | - new_name
            | - new_password
            | - old_password
        | - delete
            | - email
        | - show
            | - email
    | for todo 
        | - create
            | - index
            | - description
            | - status
        | - update
            | - index
            | - status
            | - new_description
        | - delete
            | - index
        | - show 
            | - index
```

## How CLI looks like  of our implementation
For the commands `user` or `todo`

![Packet Encapsulation](https://github.com/REZ-OAN/Rust_verse/blob/arg_parse/images/commands.png)

For the subcommands of `user`

![Packet Encapsulation](https://github.com/REZ-OAN/Rust_verse/blob/arg_parse/images/user_subcommands.png)

For the subcommand `create`

![Packet Encapsulation](https://github.com/REZ-OAN/Rust_verse/blob/arg_parse/images/user_create_values.png)

You can try your self with the `todo` command!!

