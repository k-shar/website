use maths::get_question;
use yew::prelude::*;
use yew::html::Scope;
use web_sys::HtmlInputElement;

mod maths;

pub enum Msg {
    WrongAnswer(String),
    CorrectAnswer,
}

pub struct App {
    question: maths::Question,
    response: String,
    completed: Vec<maths::Question>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            question: maths::get_question(),
            response: String::from(""), 
            completed: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WrongAnswer(v) => {
                self.response = v;
            }
            Msg::CorrectAnswer => {
                self.completed.push(self.question.clone());
                self.question = get_question();
                self.response = String::from("");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let display = format!("{} =", self.question);
        let answer = self.question.answer;
        html! {
            <div class="main">
                <div class="input_bar">
                    <p id="lhs">{ display }</p>
                    { self.view_input(ctx.link()) }
                </div>
                <ul class="completed"> { 
                    for self.completed
                            .iter()
                            .rev()
                            .map(|q| self.view_completed(q.clone())) 
                } </ul>
                <p>{ answer }</p>
            </div> 
        }
    }
}

impl App {

    fn view_completed(&self, question: maths::Question) -> Html {
        let content = question.solution();
        html! {
            <li>{ content }</li>
        }

    }

    fn view_input(&self, link: &Scope<Self>) -> Html {

        let answer = self.question.answer.to_string();

        let onkeyup = link.batch_callback(move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let content: String = input.value();
            if content == answer {
                input.set_value("");
                Some(Msg::CorrectAnswer)
            } else {
                Some(Msg::WrongAnswer(content))
            }
        });

        html! { <input type="number" {onkeyup} /> }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

