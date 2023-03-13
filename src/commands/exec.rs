                                                         use eludrs::todel::Message;
                                                       use lazy_static::lazy_static;
                                                                   use regex::Regex;
                                                          use uwuki_macros::command;
                                                                                    
                                                                        use crate::{
                                                 command_handler::CommandResult,    
                                    playground::{Playground, PlaygroundRequest},    
                                                                   state::State,    
                                                                                  };
                                                                                    
                                                                          #[command]
                                       #[uwuki(description = "Runs some rust code")]
                                                     #[uwuki(usage = "exec <code>")]
pub async fn exec(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                                  lazy_static! {    
                                              static ref CODE_REGEX: Regex =        
 Regex::new(r"(?:```(?:rs|rust)?\n?)?(?P<code>.+)\n?(?:```)?").unwrap();            
                                                                              };    
                                                      if let Some(code) = args {    
                                                                       state        
                                                                  .send(            
                                                               state                
                                .execute(PlaygroundRequest::new(                    
                                                  CODE_REGEX                        
                                        .captures(&code)                            
                                               .unwrap()                            
                                           .name("code")                            
                                               .unwrap()                            
                                               .as_str()                            
                                           .to_string(),                            
                                                              ))                    
                                                         .await?                    
                                                   .to_string(),                    
                                                                       )            
                                                                .await?;            
                                                                               }    
                                                                                    
                                                                          Ok(())    
                                                                                   }