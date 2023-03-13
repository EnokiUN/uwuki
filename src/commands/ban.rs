                                                           use eludrs::todel::Message;
                                                            use uwuki_macros::command;
                                                                                      
                            use crate::{command_handler::CommandResult, state::State};
                                                                                      
                                                                            #[command]
                        #[uwuki(description = "Bans someone because they deserve it")]
                                                      #[uwuki(usage = "ban <target>")]
   pub async fn ban(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                        if let Some(args) = args {    
                       state.send(format!("Banned {} :hammer:", args)).await?;        
                                                                                 }    
                                                                                      
                                                                            Ok(())    
                                                                                     }
                                                                                      
                                                                            #[command]
                             #[uwuki(description = "Unbans someone because fuck you")]
                                                    #[uwuki(usage = "unban <target>")]
 pub async fn unban(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                        if let Some(args) = args {    
                   state.send(format!("unBanned {} un:hammer:", args)).await?;        
                                                                                 }    
                                                                                      
                                                                            Ok(())    
                                                                                     }
                                                                                      
                                                                            #[command]
                                               #[uwuki(description = "no horny bonk")]
                                                     #[uwuki(usage = "bonk <target>")]
  pub async fn bonk(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                        if let Some(args) = args {    
                      state.send(format!("Bonkned {} :hammer:", args)).await?;        
                                                                                 }    
                                                                                      
                                                                            Ok(())    
                                                                                     }
                                                                                      
                                                                            #[command]
                                               #[uwuki(description = "knob ynroh on")]
                                                   #[uwuki(usage = "unbonk <target>")]
pub async fn unbonk(state: State, _: Message, args: Option<String>) -> CommandResult {
                                                        if let Some(args) = args {    
                  state.send(format!("unBonkned {} un:hammer:", args)).await?;        
                                                                                 }    
                                                                                      
                                                                            Ok(())    
                                                                                     }