// Copyright 2021-2022 Aaron Erhardt <aaron.erhardt@t-online.de>
// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT or Apache-2.0

use super::*;
use crate::Sender;
use std::marker::PhantomData;

/// Elm-style variant of a Component with view updates separated from input updates
pub trait Component: Sized + 'static {
    /// Internal commands to perform
    type Command: 'static + Send;

    /// Messages which are received from commands executing in the background.
    type CommandOutput: 'static + Send;

    /// The message type that the component accepts as inputs.
    type Input: 'static;

    /// The message type that the component provides as outputs.
    type Output: 'static;

    /// The initial parameter(s) for launch.
    type Payload;

    /// The widget that was constructed by the component.
    type Root: OnDestroy;

    /// The type that's used for storing widgets created for this component.
    type Widgets: 'static;

    /// Initializes the root widget
    fn init_root() -> Self::Root;

    /// Initializes the root widget and prepares a `Bridge` for docking.
    fn init() -> Bridge<Self, Self::Root> {
        Bridge {
            root: Self::init_root(),
            component: PhantomData,
        }
    }

    /// Creates the initial model and view, docking it into the component.
    fn dock(
        params: Self::Payload,
        root: &Self::Root,
        input: &mut Sender<Self::Input>,
        output: &mut Sender<Self::Output>,
    ) -> Fuselage<Self, Self::Widgets>;

    /// Processes inputs received by the component.
    #[allow(unused)]
    fn update(
        &mut self,
        message: Self::Input,
        input: &mut Sender<Self::Input>,
        output: &mut Sender<Self::Output>,
    ) -> Option<Self::Command> {
        None
    }

    /// Defines how the component should respond to command updates.
    #[allow(unused)]
    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        input: &mut Sender<Self::Input>,
        output: &mut Sender<Self::Output>,
    ) {
    }

    /// Updates the view after the model has been updated.
    #[allow(unused)]
    fn update_view(
        &self,
        widgets: &mut Self::Widgets,
        input: &mut Sender<Self::Input>,
        output: &mut Sender<Self::Output>,
    ) {
    }

    /// A command to perform in a background thread.
    #[allow(unused)]
    fn command(message: Self::Command) -> CommandFuture<Self::CommandOutput> {
        Box::pin(async move { None })
    }
}