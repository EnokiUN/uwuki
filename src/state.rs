       use std::{error::Error, fmt::Display, sync::Arc};
                                                        
               use eludrs::{todel::Message, HttpClient};
                                 use rand::rngs::StdRng;
                                    use reqwest::Client;
                                 use tokio::sync::Mutex;
                                                        
                       pub type State = Arc<UwukiState>;
                                                        
                                        #[derive(Debug)]
                                 pub struct UwukiState {
                               pub http: HttpClient,    
                                 pub client: Client,    
                   pub github_token: Option<String>,    
                             pub rng: Mutex<StdRng>,    
                                                       }
                                                        
                                       impl UwukiState {
                                  pub async fn send(    
                                          &self,        
                          content: impl Display,        
) -> Result<Message, Box<dyn Error + Send + Sync>> {    
                   self.http.send(content).await        
                                                   }    
                                                        
                          pub async fn send_message(    
                                          &self,        
                           author: impl Display,        
                          content: impl Display,        
) -> Result<Message, Box<dyn Error + Send + Sync>> {    
   self.http.send_message(author, content).await        
                                                   }    
                                                       }