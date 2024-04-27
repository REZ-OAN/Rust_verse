use clap::{
    Args,
    Parser,
    Subcommand
 };
 
 
 #[derive(Debug, Parser)]
 #[clap(author, version, about)]
 // defining the structure
 pub struct MainArgs{
  // creating subcommand using a macro
  #[clap(subcommand)]
  pub entity_type : EntityType,
 }
 #[derive(Debug,Subcommand)]
 pub enum EntityType{
    /// create, update, delete or show users
    User(UserCommand),
    /// create, update, delete or show todos
    Todos(TodoCommand),
 }
 
 
 #[derive(Debug,Args)]
 pub struct UserCommand{
    #[clap(subcommand)]
    pub command: UserSubcommand,
 }
 #[derive(Debug,Args)]
 pub struct TodoCommand{
    #[clap(subcommand)]
    pub command: TodoSubcommand,
 }
 #[derive(Debug,Subcommand)]
 pub enum UserSubcommand{
    /// Creates a new user
    Create(CreateUser),
    /// Updates a user
    Update(UpdateUser),
    /// Delete a User
    Delete(DeleteUser),
    /// Show  User Details
    Show(ShowUser),
 }
 #[derive(Debug,Subcommand)]
 pub enum TodoSubcommand{
    /// Creates a new Todo
    Create(CreateTodo),
    /// Updates a Todo
    Update(UpdateTodo),
    /// Delete a Todo
    Delete(DeleteTodo),
    /// Show  Todo Details
    Show(ShowTodo),
 }
 
 
 
 
 #[derive(Debug,Args)]
 pub struct CreateUser{
    ///  The name of the user
    pub name: String,
    /// The email of the user
    pub email: String,
    pub password: String,
 }
 #[derive(Debug,Args)]
 pub struct UpdateUser{
    /// The email of the user
    pub email: String,
    /// Update body  new_name
    pub new_name : String,
    /// Update body new_pass
    pub new_pass : String,
    /// Update body old_pass
    pub old_pass : String,
 }
 #[derive(Debug,Args)]
 pub struct DeleteUser{
    /// The email of an existing user
    pub email: String,
 }
 #[derive(Debug,Args)]
 pub struct ShowUser{
    /// The email of an existing user
    pub email: String,
 }
 
 
 
 
 #[derive(Debug,Args)]
 pub struct CreateTodo {
    /// The index of the Todo
    pub index: String,
    /// The Description of the Todo
    pub description: String,
    /// The status of the Todo
    #[clap(default_value="not_done_yet")]
    pub status:String,
 }
 #[derive(Debug,Args)]
 pub struct UpdateTodo{
    /// The index of the Todo
    pub index: String,
    /// Update Body new_description
    pub new_description: String,
    /// Update Body status
    pub status:String,
 }
 #[derive(Debug,Args)]
 pub struct DeleteTodo{
    /// The index of the Todo
    pub index: String,
 }
 #[derive(Debug,Args)]
 pub struct ShowTodo{
    /// The index of Todo
    pub index: String,
 }
 
 
 
 