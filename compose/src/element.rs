use indextree::NodeId;

use crate::view::{
    Configuration, RenderObjectConfiguration, StatefulConfiguration, StatelessConfiguration,
};

pub trait BuildContext {}

#[derive(Debug)]
pub struct StatefulElement(pub Configuration<dyn StatefulConfiguration>);

impl BuildContext for StatefulElement {}

#[derive(Debug)]
pub struct StatelessElement(pub Configuration<dyn StatelessConfiguration>);

impl BuildContext for StatelessElement {}

#[derive(Debug)]
pub struct RenderElement(pub Configuration<dyn RenderObjectConfiguration>);

impl BuildContext for RenderElement {}

#[derive(Debug)]
pub enum Element {
    Stateful(StatefulElement),
    Stateless(StatelessElement),
    Render(RenderElement),
}

pub type ElementId = NodeId;
