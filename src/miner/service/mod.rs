use crate::library::error::AppResult;

pub mod exchange_rate;
pub mod miner_stat;
pub mod message_queue;
pub mod jwt_service;


pub struct Services<'a>{
    pub exchange_rate: exchange_rate::Server<'a>,
    pub miner_stat: miner_stat::Server<'a>,
    pub message_queue: message_queue::Server,
}

impl Services<'_> {
    pub fn init() -> Services<'static> {
        Services {
            exchange_rate: exchange_rate::Server::init(),
            miner_stat: miner_stat::Server::init(),
            message_queue: message_queue::Server::init(),
        }
    }

    pub async fn serve(&self) -> AppResult<()> {
        self.exchange_rate.serve().await?;
        self.miner_stat.serve().await?;
        self.message_queue.serve().await?;
        Ok(())
    }

    pub fn shutdown(&self) -> AppResult<()> {
        self.exchange_rate.shutdown()?;
        self.miner_stat.shutdown()?;
        self.message_queue.shutdown()?;
        Ok(())
    }

}


