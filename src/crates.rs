                                                    use std::fmt::Display;
                                                                          
                                             use async_trait::async_trait;
                                          use reqwest::header::USER_AGENT;
                                      use serde::{Deserialize, Serialize};
                                                                          
                                             use crate::state::UwukiState;
                                                                          
                     pub const API_URL: &str = "https://crates.io/api/v1";
                                                                          
                           #[derive(Debug, Clone, Serialize, Deserialize)]
                                                    pub struct CrateData {
                                            #[serde(rename = "crate")]    
                                                  pub info: CrateInfo,    
                                                                         }
                                                                          
                           #[derive(Debug, Clone, Serialize, Deserialize)]
                                                    pub struct CrateInfo {
                                          description: Option<String>,    
                                        documentation: Option<String>,    
                                                       downloads: u32,    
                                             homepage: Option<String>,    
                                                         name: String,    
                                               newest_version: String,    
                                           repository: Option<String>,    
                                                                         }
                                                                          
                                              impl Display for CrateData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {    
    writeln!(f, "<https://crates.io/crates/{}>", self.info.name)?;        
                                              writeln!(f, "```")?;        
               writeln!(f, "Name:           {}", self.info.name)?;        
           if let Some(documentation) = &self.info.documentation {        
            writeln!(f, "Documentation:  {}", documentation)?;            
                                                                 }        
                     if let Some(homepage) = &self.info.homepage {        
                 writeln!(f, "Home Page:      {}", homepage)?;            
                                                                 }        
     writeln!(f, "Newest Version: {}", self.info.newest_version)?;        
                 if let Some(repository) = &self.info.repository {        
               writeln!(f, "Repository:     {}", repository)?;            
                                                                 }        
                                      if self.info.downloads > 0 {        
      writeln!(f, "Downloads:      {}", self.info.downloads)?;            
                                                                 }        
               if let Some(description) = &self.info.description {        
                writeln!(f, "Description:\n{}", description)?;            
                                                                 }        
                                                  write!(f, "```")        
                                                                     }    
                                                                         }
                                                                          
                                                            #[async_trait]
                                                        pub trait Crates {
 async fn get_crate(&self, name: String) -> anyhow::Result<CrateData>;    
                                                                         }
                                                                          
                                                            #[async_trait]
                                              impl Crates for UwukiState {
async fn get_crate(&self, name: String) -> anyhow::Result<CrateData> {    
                                                           Ok(self        
                                                       .client            
                  .get(format!("{}/crates/{}", API_URL, name))            
             .header(USER_AGENT, "Uwuki (github.com/Enokiun)")            
                                                       .send()            
                                                       .await?            
                                                       .json()            
                                                      .await?)            
                                                                     }    
                                                                         }