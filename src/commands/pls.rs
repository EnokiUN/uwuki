                                                        use eludrs::todel::Message;
                                                         use uwuki_macros::command;
                                                                                   
                         use crate::{command_handler::CommandResult, state::State};
                                                                                   
                                                                         #[command]
                                 #[uwuki(description = "Begs uwuki for something")]
                                               #[uwuki(usage = "pls <stuff here>")]
pub async fn pls(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                            if args.is_some() {    
                                               state.send("no lol").await?;        
                                                                              }    
                                                                                   
                                                                         Ok(())    
                                                                                  }