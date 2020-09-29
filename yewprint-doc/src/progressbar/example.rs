use yew::prelude::*;
use yewprint::{ProgressBar, Intent, ConditionalClass};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties, Default)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub animate: ConditionalClass,
    pub stripes: ConditionalClass,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        html! {
            <ProgressBar animate=self.props.animate stripes=self.props.stripes intent=self.props.intent value=0.3 />
        }
    }
}