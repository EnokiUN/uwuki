                                                        use eludrs::todel::Message;
                                                         use uwuki_macros::command;
                                                                                   
                 use crate::{command_handler::CommandResult, state::State, PREFIX};
                                                                                   
                                                                         #[command]
                                #[uwuki(description = "Says what you need to say")]
                                                #[uwuki(usage = "say <shit here>")]
pub async fn say(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                     if let Some(args) = args {    
                                             if !args.starts_with(PREFIX) {        
                                               state.send(args).await?;            
                                                                          }        
                                                                              }    
                                                                                   
                                                                         Ok(())    
                                                                                  }