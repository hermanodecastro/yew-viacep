use anyhow::Error;
use serde_derive::Deserialize;
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, InputData,
};

#[derive(Deserialize)]
struct Cep {
    cep: String,
    logradouro: String,
    complemento: String,
    bairro: String,
    localidade: String,
    uf: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String,
}

impl Cep {
    fn new() -> Self {
        Self {
            cep: "".to_string(),
            logradouro: "".to_string(),
            complemento: "".to_string(),
            bairro: "".to_string(),
            localidade: "".to_string(),
            uf: "".to_string(),
            ibge: "".to_string(),
            gia: "".to_string(),
            ddd: "".to_string(),
            siafi: "".to_string(),
        }
    }
}

struct Model {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    text: String,
    cep: Cep,
}

const URL: &str = "https://viacep.com.br/ws/";

enum Msg {
    FetchData(String),
    FetchReady(Cep),
    FetchResourceFailed,
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: "".to_string(),
            task: None,
            cep: Cep::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::FetchData(cep) => {
                let request = Request::get(format!("{}/{}/json/", URL, cep))
                    .body(Nothing)
                    .expect("Couldn't create request");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Cep, Error>>>| {
                            if let (meta, Json(Ok(data))) = response.into_parts() {
                                if meta.status.is_success() {
                                    return Msg::FetchReady(data);
                                }
                            }
                            Msg::FetchResourceFailed
                        });
                let task = FetchService::fetch(request, callback).expect("Failed to create task");
                self.task = Some(task);

                true
            }
            Msg::FetchReady(data) => {
                self.text = "Valid cep".to_string();
                self.cep = data;
                true
            }
            Msg::FetchResourceFailed => {
                self.text = "Invalid cep".to_string();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        html! {
            <>
            <nav>
                <div class="header">

                </div>
            </nav>
                <div class="container">
                    <h1>{ self.text.clone() }</h1>
                    <input oninput=self.link.callback(|cep: InputData| Msg::FetchData(cep.value)) />
                    <table class="table">
                        <tr>
                            <th scope="col">{"cep"}</th>
                            <th scope="col">{"logradouro"}</th>
                            <th scope="col">{"complemento"}</th>
                            <th scope="col">{"bairro"}</th>
                            <th scope="col">{"localidade"}</th>
                            <th scope="col">{"uf"}</th>
                            <th scope="col">{"ibge"}</th>
                            <th scope="col">{"gia"}</th>
                            <th scope="col">{"ddd"}</th>
                            <th scope="col">{"siafi"}</th>
                        </tr>
                        <tr>
                            <td>{self.cep.cep.clone()}</td>
                            <td>{self.cep.logradouro.clone()}</td>
                            <td>{self.cep.complemento.clone()}</td>
                            <td>{self.cep.bairro.clone()}</td>
                            <td>{self.cep.localidade.clone()}</td>
                            <td>{self.cep.uf.clone()}</td>
                            <td>{self.cep.ibge.clone()}</td>
                            <td>{self.cep.gia.clone()}</td>
                            <td>{self.cep.ddd.clone()}</td>
                            <td>{self.cep.siafi.clone()}</td>
                        </tr>
                    </table>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
