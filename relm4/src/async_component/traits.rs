// Copyright 2021-2022 Aaron Erhardt <aaron.erhardt@t-online.de>
// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT or Apache-2.0

use std::fmt::Debug;

use crate::{sender::AsyncComponentSender, Sender};

use super::{AsyncComponentBuilder, AsyncComponentParts};

/// Elm-style variant of a Component with view updates separated from input updates
#[async_trait::async_trait(?Send)]
pub trait AsyncComponent: Sized + 'static {
    /// Messages which are received from commands executing in the background.
    type CommandOutput: Debug + Send + 'static;

    /// The message type that the component accepts as inputs.
    type Input: Debug + 'static;

    /// The message type that the component provides as outputs.
    type Output: Debug + 'static;

    /// The parameter used to initialize the component.
    type Init;

    /// The widget that was constructed by the component.
    type Root: Debug + Clone;

    /// The type that's used for storing widgets created for this component.
    type Widgets: 'static;

    /// Create a builder for this component.
    #[must_use]
    fn builder() -> AsyncComponentBuilder<Self> {
        AsyncComponentBuilder::<Self>::default()
    }

    /// Initializes the root widget
    fn init_root() -> Self::Root;

    /// Creates the initial model and view, docking it into the component.
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self>;

    /// Processes inputs received by the component.
    #[allow(unused)]
    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {}

    /// Defines how the component should respond to command updates.
    #[allow(unused)]
    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
    ) {
    }

    /// Handles updates from a command.
    async fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
    ) {
        self.update_cmd(message, sender.clone()).await;
        self.update_view(widgets, sender);
    }

    /// Updates the view after the model has been updated.
    #[allow(unused)]
    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {}

    /// Updates the model and view. Optionally returns a command to run.
    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
    ) {
        self.update(message, sender.clone()).await;
        self.update_view(widgets, sender);
    }

    /// Last method called before a component is shut down.
    #[allow(unused)]
    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: Sender<Self::Output>) {}

    /// An identifier for the component used for debug logging.
    ///
    /// The default implementation of this method uses the address of the component, but
    /// implementations are free to provide more meaningful identifiers.
    fn id(&self) -> String {
        format!("{:p}", &self)
    }
}

/// Elm-style variant of a Component with view updates separated from input updates
#[async_trait::async_trait(?Send)]
pub trait SimpleAsyncComponent: Sized + 'static {
    /// The message type that the component accepts as inputs.
    type Input: Debug + 'static;

    /// The message type that the component provides as outputs.
    type Output: Debug + 'static;

    /// The parameter used to initialize the component.
    type Init;

    /// The widget that was constructed by the component.
    type Root: Debug + Clone;

    /// The type that's used for storing widgets created for this component.
    type Widgets: 'static;

    /// Initializes the root widget
    fn init_root() -> Self::Root;

    /// Creates the initial model and view, docking it into the component.
    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self>;

    /// Processes inputs received by the component.
    #[allow(unused)]
    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {}

    /// Defines how the component should respond to command updates.
    #[allow(unused)]
    async fn update_cmd(&mut self, input: &Sender<Self::Input>, output: Sender<Self::Output>) {}

    /// Updates the view after the model has been updated.
    #[allow(unused)]
    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {}

    /// Last method called before a component is shut down.
    #[allow(unused)]
    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: Sender<Self::Output>) {}
}

#[async_trait::async_trait(?Send)]
impl<C> AsyncComponent for C
where
    C: SimpleAsyncComponent,
{
    type Init = C::Init;
    type Input = C::Input;
    type Output = C::Output;
    type Root = C::Root;
    type Widgets = C::Widgets;

    type CommandOutput = ();

    fn init_root() -> Self::Root {
        C::init_root()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        C::init(init, root, sender).await
    }

    async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
        C::update(self, message, sender).await;
    }

    fn update_view(&self, widgets: &mut Self::Widgets, sender: AsyncComponentSender<Self>) {
        C::update_view(self, widgets, sender);
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: Sender<Self::Output>) {
        self.shutdown(widgets, output);
    }
}

/// An empty, non-interactive component as a placeholder for tests.
#[async_trait::async_trait(?Send)]
impl SimpleAsyncComponent for () {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = ();
    type Widgets = ();

    fn init_root() -> Self::Root {}

    async fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        AsyncComponentParts {
            model: (),
            widgets: (),
        }
    }
}
