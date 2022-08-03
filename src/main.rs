use macroquad::prelude::*;
use macroquad::ui::{root_ui, Layout};
 
extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
//extern crate env_logger;
use std::sync::Arc;
use thrussh::*;
use thrussh::server::{Auth, Session};
use thrussh_keys::*;
use futures::Future;
use futures::executor::block_on;
use std::io::Read;

struct Client {
}

impl client::Handler for Client {
   type Error = anyhow::Error;
   type FutureUnit = futures::future::Ready<Result<(Self, client::Session), anyhow::Error>>;
   type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;

   fn finished_bool(self, b: bool) -> Self::FutureBool {
       futures::future::ready(Ok((self, b)))
   }
   fn finished(self, session: client::Session) -> Self::FutureUnit {
       futures::future::ready(Ok((self, session)))
   }
   fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
       println!("check_server_key: {:?}", server_public_key);
       self.finished_bool(true)
   }
   fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
       println!("channel_open_confirmation: {:?}", channel);
       self.finished(session)
   }
   fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
       println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
       self.finished(session)
   }
}

fn issue_command() {

      let mut rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        println!("hello");
        let config = thrussh::client::Config::default();
        let config = Arc::new(config);
        let sh = Client{};
        let key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
        let mut agent = thrussh_keys::agent::client::AgentClient::connect_env().await.unwrap();
        agent.add_identity(&key, &[]).await.unwrap();
        let mut session = thrussh::client::connect(config, "localhost:22", sh).await.unwrap();

        //if session.authenticate_future(std::env::var("USER").unwrap(), key.clone_public_key(), agent).await.1.unwrap() {
        if session.authenticate_password(std::env::var("USER").unwrap(),"****").await.unwrap() {
            println!("Authetication succeeded!");
            let mut channel = session.channel_open_session().await.unwrap(); // Hangs here
            println!("Channel open session succeeded!");
            channel.data(&b"Hello, world!"[..]).await.unwrap();
            if let Some(msg) = channel.wait().await {
                println!("MESIDZ: {:?}", msg)
            }
        } else {
            println!("Authenticatefutre failed!");
        }

      })
}


//TODO(jczaja) Make all widgets alligned horizontal
//TODO(jczaja) Make a group
fn place_widgets<F1,F2>(label : &str, colour : Color, speak : F1, shutdown : F2 )
    where F1 : FnOnce(),  F2 : FnOnce()
{

        let mut skin1 = root_ui().default_skin().clone();
        skin1.button_style = root_ui().style_builder().text_color(colour).font_size(80).build(); 
        skin1.label_style = root_ui().style_builder().text_color(colour).font_size(80).build(); 
    root_ui().push_skin(&skin1);

        root_ui().group(1, Vec2::new(800.0,300.0), |ui| {

            ui.label(None, label);
            if ui.button(None, "Wylacz") {
               println!("pushed wylacz");
               shutdown();
               return;
            }
            if ui.button(None, "Zawolaj") {
               println!("pushed zawolaj");
               speak();
               return;
            }
        });
        

        root_ui().pop_skin();
}


#[macroquad::main("TEst")]
async fn main()
{
    loop {
        clear_background(WHITE);


        let func1 = || {issue_command()}; 

        place_widgets("Kasia",Color::from_rgba(0, 255, 0, 255),func1, || {issue_command()});
        place_widgets("Stefcia",Color::from_rgba(0, 0, 255, 255),func1,func1);
        place_widgets("Obie",Color::from_rgba(0, 255, 255, 255),func1, func1);

        next_frame().await
    }
}

